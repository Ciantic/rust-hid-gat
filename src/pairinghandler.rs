use crate::c1::c1_rev;
use crate::c1::s1_rev;
use crate::messages::*;
use crate::packer::*;

use crate::hcimanager::HciError;
use crate::hcimanager::MsgProcessor;

/// Take raw connection handle and returns PairedConnection, or SmpPairingFailure
pub struct PairingHandler {
    server_address: BdAddr,
    server_address_type: AddressType,
    server_random: u128,
    server_encinfo_long_term_key: u128,
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
    short_term_key: Option<u128>,
}

impl<'a> PairingHandler {
    pub fn new(
        lecon: LeConnectionComplete,
        server_address: BdAddr,
        server_address_type: AddressType,
        server_random: u128,
        server_encinfo_long_term_key: u128,
        server_cid_random: u64,
    ) -> Self {
        PairingHandler {
            server_address,
            server_address_type,
            server_random,
            server_encinfo_long_term_key,
            server_cid_random,
            connection_handle: lecon.connection_handle.clone(),
            mtu: None,
            max_key_size: None,
            preq: None,
            pres: None,
            peer_confirm_value: None,
            peer_random: None,
            peer_address: lecon.peer_address.clone(),
            peer_address_type: lecon.peer_address_type.clone(),
            short_term_key: None,
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

        // MTU Exchange from the peer
        if let HciAcl {
            msg: L2CapMessage::Att(AttPdu::ExchangeMtuRequest(ref resp)),
            ..
        } = packet
        {
            let new_mtu = self.mtu.unwrap_or_default().min(*resp);
            self.mtu = Some(new_mtu);
            return Ok(vec![H4Packet::Acl(HciAcl {
                connection_handle: self.connection_handle.clone(),
                bc: BroadcastFlag::PointToPoint,
                pb: PacketBoundaryFlag::FirstNonFlushable,
                msg: L2CapMessage::Att(AttPdu::ExchangeMtuResponse(new_mtu)),
            })]);
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
            self.peer_confirm_value = Some(value.confirm_value);
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

            if self.peer_confirm_value != Some(peer_confirm_value) {
                println!(
                    "Configuration failed server {:?} peer {:?}",
                    self.peer_confirm_value, peer_confirm_value
                );
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

            // If peer_random is received, calculate short term key
            // and send LeLongTermKeyRequestReply
            if let Some(peer_random) = self.peer_random {
                let short_term_key = u128::from_le_bytes(s1_rev(
                    &[0; 16],
                    &self.server_random.to_le_bytes(),
                    &peer_random.to_le_bytes(),
                ));
                self.short_term_key = Some(short_term_key);

                // TODO: or LeLongTermKeyRequestNegativeReply
                let reply = HciCommand::LeLongTermKeyRequestReply(LeLongTermKeyRequestReply {
                    connection_handle: e.connection_handle,
                    long_term_key: short_term_key,
                });
                return self.produce_cmd(vec![reply]);
            } else {
                // If peer_random is not received, send negative reply
                return self.produce_cmd(vec![HciCommand::LeLongTermKeyRequestNegativeReply(
                    self.connection_handle.clone(),
                )]);
            }
        }

        if let EncryptionChange(e) = packet {
            // Check if the packet is for this connection
            if e.connection_handle != self.connection_handle {
                return Ok(vec![]);
            }

            return self.produce_smp(vec![
                SmpPdu::EncryptionInformation(SmpEncryptionInformation {
                    long_term_key: self.server_encinfo_long_term_key,
                }),
                SmpPdu::CentralIdentification(SmpCentralIdentification {
                    random_number: self.server_cid_random,
                    encrypted_diversifier: 0x0000,
                }),
            ]);
        }

        Ok(vec![])
    }

    fn start_negotiate_mtu(&mut self) -> Result<Vec<H4Packet>, HciError> {
        // Start negotiating MTU size
        let mtu_request = H4Packet::Acl(HciAcl {
            connection_handle: self.connection_handle.clone(),
            bc: BroadcastFlag::PointToPoint,
            pb: PacketBoundaryFlag::FirstNonFlushable,
            msg: L2CapMessage::Att(AttPdu::ExchangeMtuRequest(244)),
        });
        self.mtu = Some(244);
        return Ok(vec![mtu_request]);
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

    fn execute(&mut self) -> Result<Vec<H4Packet>, HciError> {
        if let None = self.mtu {
            return self.start_negotiate_mtu();
        }

        return Ok(vec![]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairinghandler() {
        let mut pairing_handler = PairingHandler::new(
            LeConnectionComplete {
                status: HciStatus::Success,
                connection_handle: ConnectionHandle(64),
                role: Role::Peripheral,
                peer_address_type: AddressType::Public,
                peer_address: BdAddr([38, 14, 214, 232, 194, 80]),
                connection_interval: 48,
                peripheral_latency: 0,
                supervision_timeout: 960,
                central_clock_accuracy: ClockAccuracy::Ppm250,
            },
            BdAddr([6, 51, 116, 214, 86, 211]),
            AddressType::Random,
            49055469533520638048878300062363381969,
            282764688399516531784019739061630454460,
            723151060346651216,
        );

        // MTU test:
        // > Send MTU request 244
        // < Get MTU request 512
        // > Send MTU response 244
        let res1 = pairing_handler.execute().unwrap();
        assert_eq!(
            res1[0],
            H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                bc: BroadcastFlag::PointToPoint,
                pb: PacketBoundaryFlag::FirstNonFlushable,
                msg: L2CapMessage::Att(AttPdu::ExchangeMtuRequest(244)),
            })
        );
        let res2 = pairing_handler
            .process(H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                bc: BroadcastFlag::PointToPoint,
                pb: PacketBoundaryFlag::FirstNonFlushable,
                msg: L2CapMessage::Att(AttPdu::ExchangeMtuRequest(512)),
            }))
            .unwrap();
        assert_eq!(
            res2[0],
            H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                bc: BroadcastFlag::PointToPoint,
                pb: PacketBoundaryFlag::FirstNonFlushable,
                msg: L2CapMessage::Att(AttPdu::ExchangeMtuResponse(244)),
            })
        );

        // Pairing test:
        let res = pairing_handler
            .process(H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                pb: PacketBoundaryFlag::FirstFlushable,
                bc: BroadcastFlag::PointToPoint,
                msg: L2CapMessage::Smp(SmpPdu::PairingRequest(SmpPairingReqRes {
                    io_capability: IOCapability::KeyboardDisplay,
                    oob_data_flag: OOBDataFlag::OobNotAvailable,
                    authentication_requirements: AuthenticationRequirements {
                        bonding: true,
                        mitm_protection: true,
                        secure_connections: true,
                        keypress_notification: false,
                        ct2: true,
                        _reserved: 0,
                    },
                    max_encryption_key_size: 16,
                    initiator_key_distribution: KeyDistributionFlags {
                        enc_key: false,
                        id_key: true,
                        sign_key: true,
                        link_key: true,
                        _reserved: 0,
                    },
                    responder_key_distribution: KeyDistributionFlags {
                        enc_key: true,
                        id_key: true,
                        sign_key: true,
                        link_key: true,
                        _reserved: 0,
                    },
                })),
            }))
            .unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(
            res[0],
            H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                pb: PacketBoundaryFlag::FirstNonFlushable,
                bc: BroadcastFlag::PointToPoint,
                msg: L2CapMessage::Smp(SmpPdu::PairingResponse(SmpPairingReqRes {
                    io_capability: IOCapability::NoInputNoOutput,
                    oob_data_flag: OOBDataFlag::OobNotAvailable,
                    authentication_requirements: AuthenticationRequirements {
                        bonding: true,
                        mitm_protection: false,
                        secure_connections: false,
                        keypress_notification: false,
                        ct2: false,
                        _reserved: 0
                    },
                    max_encryption_key_size: 16,
                    initiator_key_distribution: KeyDistributionFlags {
                        enc_key: false,
                        id_key: false,
                        sign_key: false,
                        link_key: false,
                        _reserved: 0
                    },
                    responder_key_distribution: KeyDistributionFlags {
                        enc_key: true,
                        id_key: false,
                        sign_key: false,
                        link_key: false,
                        _reserved: 0
                    }
                }))
            })
        );

        // Pairing confirmation
        let res = pairing_handler
            .process(H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                pb: PacketBoundaryFlag::FirstFlushable,
                bc: BroadcastFlag::PointToPoint,
                msg: L2CapMessage::Smp(SmpPdu::PairingConfirmation(SmpPairingConfirmation {
                    confirm_value: 261697470624594529963220105986517218981,
                })),
            }))
            .unwrap();
        assert_eq!(res.len(), 1, "Single pairing confirmation response");
        assert_eq!(
            res[0],
            H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                pb: PacketBoundaryFlag::FirstNonFlushable,
                bc: BroadcastFlag::PointToPoint,
                msg: L2CapMessage::Smp(SmpPdu::PairingConfirmation(SmpPairingConfirmation {
                    confirm_value: 191883834750298512932102445673732519000,
                })),
            }),
            "Ensure pairing confirmation is sent"
        );

        // Pairing random
        let res = pairing_handler
            .process(H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                pb: PacketBoundaryFlag::FirstFlushable,
                bc: BroadcastFlag::PointToPoint,
                msg: L2CapMessage::Smp(SmpPdu::PairingRandom(SmpPairingRandom {
                    random_value: 80250483669964320715789065333977362930,
                })),
            }))
            .unwrap();
        assert_eq!(res.len(), 1, "Single pairing random response");
        assert_eq!(
            res[0],
            H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                pb: PacketBoundaryFlag::FirstNonFlushable,
                bc: BroadcastFlag::PointToPoint,
                msg: L2CapMessage::Smp(SmpPdu::PairingRandom(SmpPairingRandom {
                    random_value: 49055469533520638048878300062363381969,
                })),
            }),
            "Ensure pairing random is sent"
        );

        // Recieving LongTermKeyRequest
        let res = pairing_handler
            .process(H4Packet::Event(HciEvent::LeMeta(
                EvtLeMeta::LeLongTermKeyRequest(LeLongTermKeyRequest {
                    connection_handle: ConnectionHandle(64),
                    random_number: 0,
                    encrypted_diversifier: 0,
                }),
            )))
            .unwrap();
        assert_eq!(res.len(), 1, "Single long term key request response");
        assert_eq!(
            res[0],
            H4Packet::Command(HciCommand::LeLongTermKeyRequestReply(
                LeLongTermKeyRequestReply {
                    connection_handle: ConnectionHandle(64),
                    long_term_key: 282559536878159528170380446798965774951,
                },
            )),
            "Ensure long term key request is sent"
        );

        // Recieving EncryptionChange
        let res = pairing_handler
            .process(H4Packet::Event(HciEvent::EncryptionChange(
                EvtEncryptionChange {
                    status: HciStatus::Success,
                    connection_handle: ConnectionHandle(64),
                    encryption_enabled: true,
                },
            )))
            .unwrap();
        assert_eq!(res.len(), 2, "Two responses to encryption change");
        assert_eq!(
            res[0],
            H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                pb: PacketBoundaryFlag::FirstNonFlushable,
                bc: BroadcastFlag::PointToPoint,
                msg: L2CapMessage::Smp(SmpPdu::EncryptionInformation(SmpEncryptionInformation {
                    long_term_key: 282764688399516531784019739061630454460,
                })),
            }),
            "Ensure encryption information is sent"
        );
        assert_eq!(
            res[1],
            H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                pb: PacketBoundaryFlag::FirstNonFlushable,
                bc: BroadcastFlag::PointToPoint,
                msg: L2CapMessage::Smp(SmpPdu::CentralIdentification(SmpCentralIdentification {
                    random_number: 723151060346651216,
                    encrypted_diversifier: 0x0000,
                })),
            }),
            "Ensure central identification is sent"
        );
    }
}
