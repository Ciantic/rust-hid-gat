use bt_hci::data::AclPacket;

use crate::c1::c1_rev;
use crate::c1::s1_rev;
use crate::hcimanager::AppMsg;
use crate::messages::*;
use crate::packer::*;

use crate::hcimanager::HciError;
use crate::hcimanager::MsgProcessor;

/// Take raw connection handle and returns PairedConnection, or SmpPairingFailure
pub struct AttHandler {
    connection_handle: ConnectionHandle,
    server_mtu: u16,
    peer_mtu: Option<u16>,
}

impl AttHandler {
    pub fn new(lecon: LeConnectionComplete) -> Self {
        AttHandler {
            connection_handle: lecon.connection_handle.clone(),
            server_mtu: 244,
            peer_mtu: None,
        }
    }

    fn produce_att(&mut self, msg: Vec<AttPdu>) -> Result<Vec<AppMsg>, HciError> {
        Ok(msg
            .into_iter()
            .map(|att| {
                AppMsg::Send(H4Packet::Acl(HciAcl {
                    connection_handle: self.connection_handle.clone(),
                    bc: BroadcastFlag::PointToPoint,
                    pb: PacketBoundaryFlag::FirstNonFlushable,
                    msg: L2CapMessage::Att(att),
                }))
            })
            .collect())
    }

    fn process_att(&mut self, packet: AttPdu) -> Result<Vec<AppMsg>, HciError> {
        match packet {
            AttPdu::ExchangeMtuRequest(peer_mtu) => {
                self.peer_mtu = Some(peer_mtu);
                return self.produce_att(vec![AttPdu::ExchangeMtuResponse(self.server_mtu)]);
            }
            AttPdu::ExchangeMtuResponse(peer_mtu) => {
                self.peer_mtu = Some(peer_mtu);
                return Ok(vec![]);
            }
            _ => Ok(vec![]),
        }
    }

    fn process_event(&mut self, packet: HciEvent) -> Result<Vec<AppMsg>, HciError> {
        use EvtLeMeta::*;
        use HciEvent::*;
        match &packet {
            LeMeta(LeLongTermKeyRequest(e)) => {
                // TODO: Att Write commands are allowed after LeLongTermKeyRequest or EncryptionChange
                // TODO: ..
                Ok(vec![])
            }
            EncryptionChange(_) => {
                // TODO: First HID Report Characteristic Write is allowed after EncryptionChange
                Ok(vec![])
            }
            _ => Ok(vec![]),
        }
    }

    fn start_negotiate_mtu(&mut self) -> Result<Vec<AppMsg>, HciError> {
        // Start negotiating MTU size
        let mtu_request = AppMsg::Send(H4Packet::Acl(HciAcl {
            connection_handle: self.connection_handle.clone(),
            bc: BroadcastFlag::PointToPoint,
            pb: PacketBoundaryFlag::FirstNonFlushable,
            msg: L2CapMessage::Att(AttPdu::ExchangeMtuRequest(self.server_mtu)),
        }));
        return Ok(vec![mtu_request]);
    }
}

impl MsgProcessor for AttHandler {
    fn process(&mut self, packet: AppMsg) -> Result<Vec<AppMsg>, HciError> {
        match packet {
            AppMsg::InitAttHandler => self.start_negotiate_mtu(),

            // Att Handler
            AppMsg::Recv(H4Packet::Acl(HciAcl {
                connection_handle,
                msg: L2CapMessage::Att(att),
                ..
            })) if connection_handle == self.connection_handle => self.process_att(att),

            // HCI Event handler
            AppMsg::Recv(H4Packet::Event(event)) => self.process_event(event),

            // Other messages
            _ => Ok(vec![]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_att_handler() {
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
        // > Send MTU request 244
        // < Get MTU request 512
        // > Send MTU response 244
        // < Get MTU response 400
        let res1 = att_handler.process(AppMsg::InitAttHandler).unwrap();
        assert_eq!(
            res1[0],
            AppMsg::Send(H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                bc: BroadcastFlag::PointToPoint,
                pb: PacketBoundaryFlag::FirstNonFlushable,
                msg: L2CapMessage::Att(AttPdu::ExchangeMtuRequest(244)),
            }))
        );
        let res2 = att_handler
            .process(AppMsg::Recv(H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                bc: BroadcastFlag::PointToPoint,
                pb: PacketBoundaryFlag::FirstNonFlushable,
                msg: L2CapMessage::Att(AttPdu::ExchangeMtuRequest(512)),
            })))
            .unwrap();
        assert_eq!(att_handler.peer_mtu, Some(512));
        assert_eq!(
            res2[0],
            AppMsg::Send(H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                bc: BroadcastFlag::PointToPoint,
                pb: PacketBoundaryFlag::FirstNonFlushable,
                msg: L2CapMessage::Att(AttPdu::ExchangeMtuResponse(244)),
            }))
        );
        let res2 = att_handler
            .process(AppMsg::Recv(H4Packet::Acl(HciAcl {
                connection_handle: ConnectionHandle(64),
                bc: BroadcastFlag::PointToPoint,
                pb: PacketBoundaryFlag::FirstNonFlushable,
                msg: L2CapMessage::Att(AttPdu::ExchangeMtuResponse(400)),
            })))
            .unwrap();
        assert_eq!(att_handler.peer_mtu, Some(400));
    }
}
