use std::collections::VecDeque;

use crate::messages::{EvtCommandComplete, LeConnectionComplete, *};
use crate::packer::{FromToPacket, Packet, PacketIdentifier};

#[derive(Debug, Clone)]
pub enum SocketError {
    ReadError,
    WriteError,
}

pub trait Socket {
    fn read(&mut self) -> Result<Option<H4Packet>, SocketError>;
    fn write(&mut self, packet: H4Packet) -> Result<(), SocketError>;
}

pub struct MockSocket {
    packets: VecDeque<(bool, Vec<u8>)>,
}

impl MockSocket {
    pub fn new(packets: VecDeque<(bool, Vec<u8>)>) -> Self {
        MockSocket { packets }
    }
}

impl Socket for MockSocket {
    fn read(&mut self) -> Result<Option<H4Packet>, SocketError> {
        if let Some((false, data)) = self.packets.front() {
            let mut p = Packet::from_slice(&data);
            let m = H4Packet::from_packet(&mut p).map_err(|_| SocketError::ReadError)?;
            println!("Reading packet: {:?}", m);
            self.packets.pop_front();
            Ok(Some(m))
        } else {
            Ok(None)
        }
    }

    fn write(&mut self, packet: H4Packet) -> Result<(), SocketError> {
        println!("Writing packet: {:?}", packet);
        if let Some((true, data)) = self.packets.front() {
            let mut p = Packet::from_slice(&data);
            let h4msg = H4Packet::from_packet(&mut p).map_err(|_| SocketError::WriteError)?;
            if h4msg != packet {
                println!("Expected packet: {:?}", packet);
                println!("Got packet 🔴: {:?}", h4msg);
                return Err(SocketError::WriteError);
            }
            self.packets.pop_front();
            Ok(())
        } else {
            return Err(SocketError::WriteError);
        }
    }
}
