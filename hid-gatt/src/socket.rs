use std::collections::VecDeque;

use bt_only_headers::messages::*;
use bt_only_headers::packer::{FromToPacket, PacketIdentifier};

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
    inputs: VecDeque<H4Packet>,
    outputs: VecDeque<H4Packet>,
}

impl MockSocket {
    pub fn new() -> Self {
        MockSocket {
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
        }
    }

    fn mock_response(&mut self, packet: &H4Packet) {
        println!("HCI SERVER: {:?}", packet);
        // Mock response for HCI commands
        if let H4Packet::Command(cmd) = packet {
            let cmd_id = cmd.get_id();
            self.outputs
                .push_back(H4Packet::Event(HciEvent::CommandComplete {
                    num_hci_command_packets: 1,
                    command_opcode: cmd_id,
                    status: HciStatus::Success,
                    data: vec![],
                }));
        }

        // Mock client connection after advertising enable
        if let H4Packet::Command(HciCommand::LeSetAdvertisingEnable(true)) = packet {
            self.outputs.push_back(H4Packet::Event(HciEvent::LeMeta(
                LeMeta::LeConnectionComplete {
                    status: HciStatus::Success,
                    connection_handle: ConnectionHandle(0x0040),
                    role: Role::Central,
                    peer_address_type: AddressType::Random,
                    peer_address: BdAddr([0x26, 0xe, 0xd6, 0xe8, 0xc2, 0x50]),
                    connection_interval: 48,
                    peripheral_latency: 0,
                    supervision_timeout: 960,
                    central_clock_accuracy: ClockAccuracy::Ppm250,
                },
            )));
        }
    }
}

impl Socket for MockSocket {
    fn read(&mut self) -> Result<Option<H4Packet>, SocketError> {
        Ok(self.outputs.pop_front())
    }

    fn write(&mut self, packet: H4Packet) -> Result<(), SocketError> {
        self.mock_response(&packet);
        self.inputs.push_back(packet);
        Ok(())
    }
}
