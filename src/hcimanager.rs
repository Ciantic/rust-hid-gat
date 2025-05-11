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
use crate::{atthandler::AttHandler, packer::PacketIdentifier};
use crate::{c1::c1_rev, packer::FixedSizeUtf8};
use crate::{
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppMsg {
    InitAttHandler,
    InitPairingHandler,
    Send(H4Packet),
    Recv(H4Packet),
    Disconnect(ConnectionHandle),
    DisconnectComplete(ConnectionHandle),
    Pairing(ConnectionHandle),
    PairingComplete(ConnectionHandle),
}

impl From<SocketError> for HciError {
    fn from(err: SocketError) -> Self {
        HciError::SocketError(err)
    }
}

pub trait MsgProcessor {
    fn process(&mut self, msg: AppMsg) -> Result<Vec<AppMsg>, HciError>;
}

pub struct HciManager {
    processors: Vec<Box<dyn MsgProcessor>>,
    allowed_hci_command_packets: u8,
    unpaired_connections: BTreeSet<ConnectionHandle>,
    paired_connections: BTreeSet<ConnectionHandle>,
}

impl HciManager {
    pub fn new() -> Result<Self, HciError> {
        let allowed_hci_command_packets = 0;
        let unpaired_connections = BTreeSet::new();
        let paired_connections = BTreeSet::new();
        let processors = Vec::new();

        Ok(HciManager {
            processors,
            allowed_hci_command_packets,
            unpaired_connections,
            paired_connections,
        })
    }
}

impl MsgProcessor for HciManager {
    fn process(&mut self, msg: AppMsg) -> Result<Vec<AppMsg>, HciError> {
        let mut msgs = vec![];
        for processor in self.processors.iter_mut() {
            msgs.append(&mut processor.process(msg.clone())?.into())
        }

        //
        match msg {
            AppMsg::Send(_) => {}
            AppMsg::Recv(H4Packet::Event(evt)) => {
                use HciEvent::*;
                match evt {
                    CommandComplete(e) => {
                        self.allowed_hci_command_packets = e.num_hci_command_packets;
                    }

                    CommandStatus(e) => {
                        self.allowed_hci_command_packets = e.num_hci_command_packets;
                    }
                    LeMeta(EvtLeMeta::LeConnectionComplete(e)) => {
                        println!("LE Connection Complete: {:?}", e);
                        let mut att_handler = AttHandler::new(e.clone());
                        msgs.append(&mut att_handler.process(AppMsg::InitAttHandler)?);
                        self.processors.push(Box::new(att_handler));

                        let mut pairing_handler = PairingHandler::new(
                            e.clone(),
                            // TODO: Real randoms
                            BdAddr([6, 51, 116, 214, 86, 211]),
                            AddressType::Random,
                            49055469533520638048878300062363381969,
                            282559536878159528170380446798965774951,
                            723151060346651216,
                        );
                        msgs.append(&mut pairing_handler.process(AppMsg::InitPairingHandler)?);
                        self.processors.push(Box::new(pairing_handler));
                    }
                    _ => {}
                }
            }
            AppMsg::Recv(_) => {}
            AppMsg::Disconnect(_) => {}
            AppMsg::DisconnectComplete(handle) => {
                self.paired_connections.remove(&handle);
            }
            AppMsg::Pairing(handle) => {
                self.unpaired_connections.insert(handle);
            }
            AppMsg::PairingComplete(handle) => {
                self.paired_connections.insert(handle);
            }
            _ => {}
        }

        Ok(msgs)
    }
}

struct AttHanlder {
    connection: ConnectionHandle,
    mtu: u16,
}

struct PairedConnection {}
