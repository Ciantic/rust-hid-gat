use bt_only_headers::c1::c1_rev;
use bt_only_headers::messages::*;
use bt_only_headers::packer::*;

use crate::hcimanager::HciError;
use crate::hcimanager::MsgProcessor;

/// Take raw connection handle and returns PairedConnection, or SmpPairingFailure
pub struct PairingHandler {
    server_address: BdAddr,
    server_address_type: AddressType,
    server_random: u128,
    server_confirm_value: Option<u128>,
    server_long_term_key: u128,
    server_cid_random: u64,
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

impl<'a> PairingHandler {
    pub fn new(
        lecon: LeConnectionComplete,
        server_address: BdAddr,
        server_address_type: AddressType,
    ) -> Self {
        PairingHandler {
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

    // Copied from my parrot.rs
    //
    // Wait for SmpPdu::PairingRequest
    // -> Send SmpPdu::PairingResponse
    // Wait for SmpPdu::PairingConfirm
    // -> Send SmpPdu::PairingConfirm
    // Wait for SmpPdu::PairingRandom
    // -> Send SmpPdu::PairingRandom or SmpPdu::PairingFailed
    // Wait for LeLongTermKeyRequest
    // -> Send LeLongTermKeyRequestReply or LeLongTermKeyRequestNegativeReply
    // Wait for HciEvent::EncryptionChange
    // -> Send SmpEncryptionInformation
    // -> Send SmpCentralIdentification

    fn produce_smp(&mut self, msg: Vec<SmpPdu>) -> Result<Vec<H4Packet>, HciError> {
        Ok(msg
            .into_iter()
            .map(|smp| {
                H4Packet::Acl(HciAcl {
                    connection_handle: self.connection_handle.clone(),
                    bc: BroadcastFlag::PointToPoint,
                    pb: PacketBoundaryFlag::FirstNonFlushable,
                    msg: L2CapMessage::Smp(smp),
                })
            })
            .collect())
    }

    fn produce_cmd(&mut self, cmd: Vec<HciCommand>) -> Result<Vec<H4Packet>, HciError> {
        Ok(cmd.into_iter().map(|cmd| H4Packet::Command(cmd)).collect())
    }

    /// Handle SMP pairing process
    fn process_acl(&mut self, packet: HciAcl) -> Result<Vec<H4Packet>, HciError> {
        // Check if the packet is for this connection
        if packet.connection_handle != self.connection_handle {
            return Ok(vec![]);
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
            return self.produce_smp(vec![pres]);
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
            return self.produce_smp(vec![pconfirm]);
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
                return self.produce_smp(vec![pairing_failed]);
                // TODO: Send Disconnect?
            } else {
                // 6. Send SmpPdu::PairingRandom
                let prandom = SmpPdu::PairingRandom(SmpPairingRandom {
                    random_value: self.server_random,
                });
                return self.produce_smp(vec![prandom]);
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
        Ok(vec![])
    }

    /// Handle HciEvent::LeLongTermKeyRequest
    fn process_event(&mut self, packet: HciEvent) -> Result<Vec<H4Packet>, HciError> {
        use HciEvent::*;
        if let LeMeta(EvtLeMeta::LeLongTermKeyRequest(e)) = packet {
            // Check if the packet is for this connection
            if e.connection_handle != self.connection_handle {
                return Ok(vec![]);
            }

            // TODO: or LeLongTermKeyRequestNegativeReply
            let reply = HciCommand::LeLongTermKeyRequestReply(LeLongTermKeyRequestReply {
                connection_handle: e.connection_handle,
                long_term_key: 0xe906ea2ad9e76155e955033d694bbcfa,
            });
            return self.produce_cmd(vec![reply]);
        }

        if let EncryptionChange(e) = packet {
            // Check if the packet is for this connection
            if e.connection_handle != self.connection_handle {
                return Ok(vec![]);
            }

            return self.produce_smp(vec![
                SmpPdu::EncryptionInformation(SmpEncryptionInformation {
                    long_term_key: self.server_long_term_key,
                }),
                SmpPdu::CentralIdentification(SmpCentralIdentification {
                    random_number: self.server_cid_random,
                    encrypted_diversifier: 0x0000,
                }),
            ]);
        }

        Ok(vec![])
    }
}

impl MsgProcessor for PairingHandler {
    fn process(&mut self, packet: H4Packet) -> Result<Vec<H4Packet>, HciError> {
        match packet {
            H4Packet::Acl(acl) => self.process_acl(acl),
            H4Packet::Event(event) => self.process_event(event),
            _ => Ok(vec![]),
        }
    }
}
