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

use crate::{
    pairinghandler::PairingHandler,
    socket::{Socket, SocketError},
};

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

pub trait MsgProcessor {
    fn process(&mut self, packet: H4Packet) -> Result<Vec<H4Packet>, HciError>;
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
                self.processors.push(Box::new(PairingHandler::new(
                    e.clone(),
                    BdAddr([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
                    AddressType::Public,
                )));
            }
            _ => {}
        }

        Ok(())
    }

    fn process_acl(&mut self, acl: &HciAcl) -> Result<(), HciError> {
        Ok(())
    }

    fn process_packet(&mut self, packet: &H4Packet) -> Result<(), HciError> {
        use H4Packet::*;
        for processor in self.processors.iter_mut() {
            self.outbox
                .append(&mut processor.process(packet.clone())?.into())
        }

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
