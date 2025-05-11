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
use crate::packer::PacketIdentifier;
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

// pub enum HciMsg {
//     Send(H4Packet),
//     Recv(H4Packet),
//     Disconnect(ConnectionHandle),
//     DisconnectComplete(ConnectionHandle),
//     Pairing(ConnectionHandle),
//     PairingComplete(ConnectionHandle),
// }

impl From<SocketError> for HciError {
    fn from(err: SocketError) -> Self {
        HciError::SocketError(err)
    }
}

pub trait MsgProcessor {
    fn process(&mut self, packet: H4Packet) -> Result<Vec<H4Packet>, HciError>;
    fn execute(&mut self) -> Result<Vec<H4Packet>, HciError>;
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
    pub fn new(
        init_packets: VecDeque<H4Packet>,
        socket: Box<dyn Socket>,
    ) -> Result<Self, HciError> {
        let mut outbox = init_packets;
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
                    // TODO: Real randoms
                    BdAddr([6, 51, 116, 214, 86, 211]),
                    AddressType::Random,
                    49055469533520638048878300062363381969,
                    282559536878159528170380446798965774951,
                    723151060346651216,
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

        for processor in self.processors.iter_mut() {
            self.outbox.append(&mut processor.execute()?.into());
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
