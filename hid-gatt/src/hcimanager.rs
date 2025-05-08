#![allow(unused)]

use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    io::{BufReader, BufWriter},
    ops::Add,
    os::windows::process,
    rc::{Rc, Weak},
};

const BLUETOOTH_BASE_UUID: &str = "00000000-0000-1000-8000-00805F9B34FB";

// use crate::hciserver::DummyHciServer;
use bt_only_headers::packer::PacketIdentifier;
use bt_only_headers::{c1::c1_rev, packer::FixedSizeUtf8};
use bt_only_headers::{
    messages::{EvtCommandComplete, EvtCommandStatus, HciAcl, *},
    packer::FromToPacket,
};

use crate::socket::{Socket, SocketError};

#[derive(Debug, Clone)]
pub enum HciError {
    SocketError(SocketError),
    PacketError,
    Unknown(String),
}

impl From<SocketError> for HciError {
    fn from(err: SocketError) -> Self {
        HciError::SocketError(err)
    }
}

trait MsgProcessor {
    fn process_acl(&mut self, packet: HciAcl) -> Result<bool, HciError>;
    fn process_event(&mut self, packet: HciEvent) -> Result<bool, HciError>;
}

pub struct HciManager {
    outbox: VecDeque<H4Packet>,
    inbox: VecDeque<H4Packet>,
    processors: Vec<Box<dyn MsgProcessor>>,
    socket: Box<dyn Socket>,
    allowed_hci_command_packets: u8,
    unpaired_connections: BTreeSet<ConnectionHandle>,
    paired_connections: BTreeSet<ConnectionHandle>,
}

impl HciManager {
    pub fn new(socket: Box<dyn Socket>) -> Result<Self, HciError> {
        let mut outbox = initialize_bluetooth();
        let inbox = VecDeque::new();
        let allowed_hci_command_packets = 0;
        let unpaired_connections = BTreeSet::new();
        let paired_connections = BTreeSet::new();
        let processors = Vec::new();

        Ok(HciManager {
            outbox,
            inbox,
            socket,
            processors,
            allowed_hci_command_packets,
            unpaired_connections,
            paired_connections,
        })
    }

    fn process_event(&mut self, event: &HciEvent) -> Result<(), HciError> {
        use HciEvent::*;
        match event {
            CommandComplete(e) => {
                self.allowed_hci_command_packets = e.num_hci_command_packets;
            }

            CommandStatus(e) => {
                self.allowed_hci_command_packets = e.num_hci_command_packets;
            }
            LeMeta(EvtLeMeta::LeConnectionComplete(e)) => {
                println!("LE Connection Complete: {:?}", e);
                self.processors.push(Box::new(ConnectionHandler::new(
                    todo!(),
                    e.clone(),
                    BdAddr([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
                    AddressType::Public,
                )));
            }
            _ => {}
        }

        for processor in self.processors.iter_mut() {
            if processor.process_event(event.clone())? {
                // return Ok(());
            }
        }
        Ok(())
    }

    fn process_acl(&mut self, acl: &HciAcl) -> Result<(), HciError> {
        for processor in self.processors.iter_mut() {
            if processor.process_acl(acl.clone())? {
                // return Ok(());
            }
        }
        Ok(())
    }

    fn process_packet(&mut self, packet: &H4Packet) -> Result<(), HciError> {
        use H4Packet::*;

        match &packet {
            H4Packet::Event(event) => {
                self.process_event(event)?;
            }
            H4Packet::Acl(acl) => {
                self.process_acl(acl)?;
            }
            H4Packet::Command(command) => {
                panic!(
                    "HCI Host can't send commands to the controller: {:?}",
                    command
                );
            }
        }
        Ok(())
    }

    pub fn process(&mut self) -> Result<(), HciError> {
        while let Some(packet) = self.inbox.pop_front() {
            if let Err(err) = self.process_packet(&packet) {
                println!("Error processing packet: {:?}", err);
            }
        }
        Ok(())
    }

    pub fn execute(&mut self) -> Result<(), HciError> {
        if let Some(packet) = self.outbox.pop_front() {
            self.socket.write(packet.clone())?;
        };

        while let Some(packet) = self.socket.read()? {
            self.inbox.push_back(packet.clone());
        }
        Ok(())
    }

    pub fn send(&mut self, packet: H4Packet) -> Result<(), HciError> {
        self.outbox.push_back(packet);
        Ok(())
    }
}

struct AttHanlder {
    connection: ConnectionHandle,
    mtu: u16,
}

/// Take raw connection handle and returns PairedConnection, or SmpPairingFailure
struct ConnectionHandler {
    server_address: BdAddr,
    server_address_type: AddressType,
    server_random: u128,
    server_confirm_value: Option<u128>,
    server_long_term_key: u128,
    server_cid_random: u64,
    hci: Weak<RefCell<HciManager>>,
    connection_handle: ConnectionHandle,
    mtu: Option<u16>,
    max_key_size: Option<u8>,
    preq: Option<[u8; 7]>,
    pres: Option<[u8; 7]>,
    peer_confirm_value: Option<u128>,
    peer_random: Option<u128>,
    peer_address: BdAddr,
    peer_address_type: AddressType,
}

impl<'a> ConnectionHandler {
    fn new(
        hci: Weak<RefCell<HciManager>>,
        lecon: LeConnectionComplete,
        server_address: BdAddr,
        server_address_type: AddressType,
    ) -> Self {
        ConnectionHandler {
            hci,
            server_address,
            server_address_type,
            server_random: u128::from_le_bytes([
                // TODO: This should be randomized at some point
                0x6d, 0xde, 0x61, 0xf5, 0x68, 0x16, 0x96, 0x67, 0x8a, 0x5e, 0x28, 0x70, 0x1a, 0x34,
                0x38, 0x0,
            ]),
            server_confirm_value: None,
            server_long_term_key: 0xe906ea2ad9e76155e955033d694bbcfa,
            server_cid_random: u64::from_le_bytes([0x50, 0xc2, 0xe8, 0xd6, 0xe, 0x26, 0x9, 0xa]),
            connection_handle: lecon.connection_handle.clone(),
            mtu: None,
            max_key_size: None,
            preq: None,
            pres: None,
            peer_confirm_value: None,
            peer_random: None,
            peer_address: lecon.peer_address.clone(),
            peer_address_type: lecon.peer_address_type.clone(),
        }
    }

    fn send_smp(&mut self, msg: SmpPdu) -> Result<(), HciError> {
        let acl = H4Packet::Acl(HciAcl {
            connection_handle: self.connection_handle.clone(),
            bc: BroadcastFlag::PointToPoint,
            pb: PacketBoundaryFlag::FirstNonFlushable,
            msg: L2CapMessage::Smp(msg),
        });
        if let Some(ptr) = self.hci.upgrade() {
            let mut hci = ptr.borrow_mut();
            hci.send(acl)?;
        } else {
            return Err(HciError::Unknown("HCI Manager is gone".to_string()));
        }
        Ok(())
    }

    fn send_cmd(&mut self, cmd: HciCommand) -> Result<(), HciError> {
        let packet = H4Packet::Command(cmd);
        if let Some(ptr) = self.hci.upgrade() {
            let mut hci = ptr.borrow_mut();
            hci.send(packet)?;
        } else {
            return Err(HciError::Unknown("HCI Manager is gone".to_string()));
        }
        Ok(())
    }
}

// Copied from my parrot.rs
//
// 1. Wait for SmpPdu::PairingRequest
// 2. Send SmpPdu::PairingResponse
// 3. Wait for SmpPdu::PairingConfirm
// 4. Send SmpPdu::PairingConfirm
// 5. Wait for SmpPdu::PairingRandom
// 6. Send SmpPdu::PairingRandom or SmpPdu::PairingFailed
// 7. Wait for LeLongTermKeyRequest
// 8. Send LeLongTermKeyRequestReply or LeLongTermKeyRequestNegativeReply
// 9. Wait for HciEvent::EncryptionChange
// 10. Send SmpEncryptionInformation
// 11. Send SmpCentralIdentification

impl MsgProcessor for ConnectionHandler {
    fn process_acl(&mut self, packet: HciAcl) -> Result<bool, HciError> {
        // Check if the packet is for this connection
        if packet.connection_handle != self.connection_handle {
            return Ok(false);
        }

        // 1. Wait for SmpPdu::PairingRequest
        if let HciAcl {
            msg: L2CapMessage::Smp(ref preq @ SmpPdu::PairingRequest(ref payload)),
            ..
        } = packet
        {
            let pres = SmpPdu::PairingResponse(SmpPairingReqRes {
                authentication_requirements: AuthenticationRequirements {
                    bonding: true,
                    mitm_protection: false,
                    secure_connections: false,
                    keypress_notification: false,
                    ct2: false,
                    _reserved: 0,
                },
                io_capability: IOCapability::NoInputNoOutput,
                max_encryption_key_size: 16,
                oob_data_flag: OOBDataFlag::OobNotAvailable,
                initiator_key_distribution: KeyDistributionFlags {
                    enc_key: false,
                    id_key: false,
                    sign_key: false,
                    link_key: false,
                    _reserved: 0,
                },
                responder_key_distribution: KeyDistributionFlags {
                    enc_key: true,
                    id_key: false,
                    sign_key: false,
                    link_key: false,
                    _reserved: 0,
                },
            });
            // Store values for C1
            self.max_key_size = Some(payload.max_encryption_key_size);
            self.preq = Some(preq.to_bytes().try_into().unwrap());
            self.pres = Some(pres.to_bytes().try_into().unwrap());

            // 2. Send SmpPdu::PairingResponse
            self.send_smp(pres);
        }

        // 3. Wait for SmpPdu::PairingConfirm
        if let HciAcl {
            msg: L2CapMessage::Smp(SmpPdu::PairingConfirmation(ref value)),
            ..
        } = packet
        {
            let server_confirm_value = u128::from_le_bytes(c1_rev(
                &[0; 16],
                &self.server_random.to_le_bytes(),
                &self.pres.unwrap(),
                &self.preq.unwrap(),
                self.peer_address_type.to_bytes()[0],
                &self.peer_address.to_bytes().try_into().unwrap(),
                self.server_address_type.to_bytes()[0],
                &self.server_address.to_bytes().try_into().unwrap(),
            ));
            self.peer_confirm_value = Some(value.confirm_value);
            self.server_confirm_value = server_confirm_value.into();

            // 4. Send SmpPdu::PairingConfirm
            let pconfirm = SmpPdu::PairingConfirmation(SmpPairingConfirmation {
                confirm_value: server_confirm_value,
            });
            self.send_smp(pconfirm)?;
        }

        // 5. Wait for SmpPdu::PairingRandom
        if let HciAcl {
            msg: L2CapMessage::Smp(SmpPdu::PairingRandom(ref value)),
            ..
        } = packet
        {
            self.peer_random = Some(value.random_value);
            let peer_confirm_value = u128::from_le_bytes(c1_rev(
                &[0; 16],
                &value.random_value.to_le_bytes(),
                &self.pres.unwrap(),
                &self.preq.unwrap(),
                self.peer_address_type.to_bytes()[0],
                &self.peer_address.to_bytes().try_into().unwrap(),
                self.server_address_type.to_bytes()[0],
                &self.server_address.to_bytes().try_into().unwrap(),
            ));

            if self.server_confirm_value != Some(peer_confirm_value) {
                // 6. Send SmpPdu::PairingFailed
                let pairing_failed = SmpPdu::PairingFailed(SmpPairingFailure::ConfirmValueFailed);
                self.send_smp(pairing_failed)?;
                // TODO: Disconnect
            } else {
                // 6. Send SmpPdu::PairingRandom
                let prandom = SmpPdu::PairingRandom(SmpPairingRandom {
                    random_value: self.server_random,
                });
                self.send_smp(prandom)?;
            }
        }

        // Check if the packet is a pairing request
        // if let H4Packet::Acl(acl) = packet {
        // if let Some(smp_request) = acl.get_smp_request() {
        //     // Process the pairing request
        //     match self.process(smp_request) {
        //         Ok(_) => return true,
        //         Err(_) => return false,
        //     }
        // }
        // }
        Ok(false)
    }

    fn process_event(&mut self, packet: HciEvent) -> Result<bool, HciError> {
        if let HciEvent::LeMeta(EvtLeMeta::LeLongTermKeyRequest(e)) = packet {
            // Check if the packet is for this connection
            if e.connection_handle != self.connection_handle {
                return Ok(false);
            }

            // TODO: or LeLongTermKeyRequestNegativeReply
            let reply = HciCommand::LeLongTermKeyRequestReply(LeLongTermKeyRequestReply {
                connection_handle: e.connection_handle,
                long_term_key: 0xe906ea2ad9e76155e955033d694bbcfa,
            });
            self.send_cmd(reply)?;
            return Ok(true);
        }

        if let HciEvent::EncryptionChange(e) = packet {
            // Check if the packet is for this connection
            if e.connection_handle != self.connection_handle {
                return Ok(false);
            }

            self.send_smp(SmpPdu::EncryptionInformation(SmpEncryptionInformation {
                long_term_key: self.server_long_term_key,
            }))?;

            self.send_smp(SmpPdu::CentralIdentification(SmpCentralIdentification {
                random_number: self.server_cid_random,
                encrypted_diversifier: 0x0000,
            }))?;
        }
        Ok(false)
    }
}

struct PairedConnection {}

fn initialize_bluetooth() -> VecDeque<H4Packet> {
    vec![
        (HciCommand::Reset),
        (HciCommand::SetEventMask(0x3DBFF807FFFBFFFF)),
        (HciCommand::LeSetEventMask(0x00000000000005ff)),
        (HciCommand::WriteScanEnable(CmdScanEnable::InquiryScanEnabled_PageScanEnabled)),
        (HciCommand::WriteConnectionAcceptTimeout(16288)),
        (HciCommand::WritePageTimeout(16384)),
        (HciCommand::ReadLocalSupportedCommands),
        (HciCommand::ReadBdAddr),
        (HciCommand::LeReadBufferSize),
        (HciCommand::WriteLocalName(FixedSizeUtf8::new("My Pi"))),
        (HciCommand::LeSetRandomAddress(BdAddr([0xa9, 0x36, 0x3c, 0xde, 0x52, 0xd7]))),
        // (HciCommand::LeReadLocalP256PublicKey),
        (HciCommand::LeSetAdvertisingParameters(LeSetAdvertisingParameters {
            advertising_interval_min: 512,
            advertising_interval_max: 512,
            advertising_type: 0x00,
            own_address_type: 0x01,
            peer_address_type: 0x00,
            peer_address: BdAddr([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
            advertising_channel_map: 0x07,
            advertising_filter_policy: 0x00,
        })),
        (HciCommand::LeSetAdvertisingData(LeSetAdvertisingData {
            advertising_data_length: 16,
            advertising_data: [
                0x2, 0x1, 0x6, 0x3, 0x19, 0xc1, 0x3, 0x4, 0x8, 0x48, 0x49, 0x44, 0x3, 0x2, 0x12,
                0x18, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
        })),
        (HciCommand::LeSetAdvertisingEnable(true)),
    ]
    .iter()
    .map(|f| H4Packet::Command(f.clone()))
    .collect::<VecDeque<H4Packet>>()
}
