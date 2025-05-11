use crate::c1::c1_rev;
use crate::c1::s1_rev;
use crate::messages::*;
use crate::packer::*;

use crate::hcimanager::HciError;
use crate::hcimanager::MsgProcessor;

/// Take raw connection handle and returns PairedConnection, or SmpPairingFailure
pub struct AttHandler {
    connection_handle: ConnectionHandle,
    server_mtu: u16,
    peer_mtu: Option<u16>,
    mtu_negotation_started: bool,
}

impl<'a> AttHandler {
    pub fn new(lecon: LeConnectionComplete) -> Self {
        AttHandler {
            connection_handle: lecon.connection_handle.clone(),
            mtu_negotation_started: false,
            server_mtu: 247,
            peer_mtu: None,
        }
    }

    fn produce_att(&mut self, msg: Vec<AttPdu>) -> Result<Vec<H4Packet>, HciError> {
        Ok(msg
            .into_iter()
            .map(|att| {
                H4Packet::Acl(HciAcl {
                    connection_handle: self.connection_handle.clone(),
                    bc: BroadcastFlag::PointToPoint,
                    pb: PacketBoundaryFlag::FirstNonFlushable,
                    msg: L2CapMessage::Att(att),
                })
            })
            .collect())
    }

    /// Handle SMP pairing process
    fn process_acl(&mut self, packet: HciAcl) -> Result<Vec<H4Packet>, HciError> {
        let msg = &packet.msg;

        // Check if the packet is for this connection
        if packet.connection_handle != self.connection_handle {
            return Ok(vec![]);
        }

        // MTU Exchange from the peer
        if let L2CapMessage::Att(AttPdu::ExchangeMtuRequest(peer_mtu)) = msg {
            self.peer_mtu = Some(*peer_mtu);
            return self.produce_att(vec![AttPdu::ExchangeMtuResponse(self.server_mtu)]);
        }

        // MTU Response from the peer
        if let L2CapMessage::Att(AttPdu::ExchangeMtuResponse(peer_mtu)) = msg {
            self.peer_mtu = Some(*peer_mtu);
        }

        Ok(vec![])
    }

    /// Handle HciEvent::LeLongTermKeyRequest
    fn process_event(&mut self, packet: HciEvent) -> Result<Vec<H4Packet>, HciError> {
        use HciEvent::*;

        // TODO: Att Write commands are allowed after LeLongTermKeyRequest or EncryptionChange
        if let LeMeta(EvtLeMeta::LeLongTermKeyRequest(e)) = &packet {
            // TODO: ..
        }

        if let EncryptionChange(_) = &packet {
            // TODO: First HID Report Characteristic Write is allowed after EncryptionChange
        }

        Ok(vec![])
    }

    fn start_negotiate_mtu(&mut self) -> Result<Vec<H4Packet>, HciError> {
        self.mtu_negotation_started = true;
        // Start negotiating MTU size
        let mtu_request = H4Packet::Acl(HciAcl {
            connection_handle: self.connection_handle.clone(),
            bc: BroadcastFlag::PointToPoint,
            pb: PacketBoundaryFlag::FirstNonFlushable,
            msg: L2CapMessage::Att(AttPdu::ExchangeMtuRequest(self.server_mtu)),
        });
        return Ok(vec![mtu_request]);
    }
}

impl MsgProcessor for AttHandler {
    fn process(&mut self, packet: H4Packet) -> Result<Vec<H4Packet>, HciError> {
        match packet {
            H4Packet::Acl(acl) => self.process_acl(acl),
            H4Packet::Event(event) => self.process_event(event),
            _ => Ok(vec![]),
        }
    }

    fn execute(&mut self) -> Result<Vec<H4Packet>, HciError> {
        if !self.mtu_negotation_started {
            return self.start_negotiate_mtu();
        }

        return Ok(vec![]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_Atthandler() {
        let mut att_handler = AttHandler::new(LeConnectionComplete {
            status: HciStatus::Success,
            connection_handle: ConnectionHandle(64),
            role: Role::Peripheral,
            peer_address_type: AddressType::Public,
            peer_address: BdAddr([38, 14, 214, 232, 194, 80]),
            connection_interval: 48,
            peripheral_latency: 0,
            supervision_timeout: 960,
            central_clock_accuracy: ClockAccuracy::Ppm250,
        });

        // MTU test:
        // > Send MTU request 247
        // < Get MTU request 512
        // > Send MTU response 247
        // < Get MTU response 400
        let res1 = att_handler.execute().unwrap();
        assert_eq!(
            res1[0],
            H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                bc: BroadcastFlag::PointToPoint,
                pb: PacketBoundaryFlag::FirstNonFlushable,
                msg: L2CapMessage::Att(AttPdu::ExchangeMtuRequest(247)),
            })
        );
        let res2 = att_handler
            .process(H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                bc: BroadcastFlag::PointToPoint,
                pb: PacketBoundaryFlag::FirstNonFlushable,
                msg: L2CapMessage::Att(AttPdu::ExchangeMtuRequest(512)),
            }))
            .unwrap();
        assert_eq!(att_handler.peer_mtu, Some(512));
        assert_eq!(
            res2[0],
            H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                bc: BroadcastFlag::PointToPoint,
                pb: PacketBoundaryFlag::FirstNonFlushable,
                msg: L2CapMessage::Att(AttPdu::ExchangeMtuResponse(247)),
            })
        );
        att_handler
            .process(H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                bc: BroadcastFlag::PointToPoint,
                pb: PacketBoundaryFlag::FirstNonFlushable,
                msg: L2CapMessage::Att(AttPdu::ExchangeMtuResponse(400)),
            }))
            .unwrap();
        assert_eq!(att_handler.peer_mtu, Some(400));
    }
}
