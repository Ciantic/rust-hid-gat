use std::io::{BufReader, BufWriter};

use bt_only_headers::{messages::*, packer::FixedSizeUtf8};

const BLUETOOTH_BASE_UUID: &str = "00000000-0000-1000-8000-00805F9B34FB";

fn initialize_bluetooth() -> Vec<H4Packet> {
    vec![
        H4Packet::HciCommand(HciCommand::Reset),
        H4Packet::HciCommand(HciCommand::SetEventMask {
            event_mask: 0x3DBFF807FFFBFFFF,
        }),
        H4Packet::HciCommand(HciCommand::LeSetEventMask {
            event_mask: 0x00000000000005ff,
        }),
        H4Packet::HciCommand(HciCommand::WriteScanEnable(
            ScanEnable::InquiryScanEnabled_PageScanEnabled,
        )),
        H4Packet::HciCommand(HciCommand::WriteConnectionAcceptTimeout(16288)),
        H4Packet::HciCommand(HciCommand::WritePageTimeout(16384)),
        H4Packet::HciCommand(HciCommand::ReadLocalSupportedCommands),
        H4Packet::HciCommand(HciCommand::ReadBdAddr),
        H4Packet::HciCommand(HciCommand::LeReadBufferSize),
        H4Packet::HciCommand(HciCommand::WriteLocalName(FixedSizeUtf8::new("My Pi"))),
        H4Packet::HciCommand(HciCommand::LeSetRandomAddress(BdAddr([
            0xa9, 0x36, 0x3c, 0xde, 0x52, 0xd7,
        ]))),
        H4Packet::HciCommand(HciCommand::LeSetAdvertisingParameters {
            advertising_interval_min: 512,
            advertising_interval_max: 512,
            advertising_type: 0x00,
            own_address_type: 0x01,
            peer_address_type: 0x00,
            peer_address: BdAddr([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
            advertising_channel_map: 0x07,
            advertising_filter_policy: 0x00,
        }),
        H4Packet::HciCommand(HciCommand::LeSetAdvertisingData {
            advertising_data_length: 16,
            advertising_data: [
                0x2, 0x1, 0x6, 0x3, 0x19, 0xc1, 0x3, 0x4, 0x8, 0x48, 0x49, 0x44, 0x3, 0x2, 0x12,
                0x18, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
        }),
        H4Packet::HciCommand(HciCommand::LeReadLocalP256PublicKey),
        H4Packet::HciCommand(HciCommand::LeSetRandomAddress(BdAddr([
            0x6, 0x33, 0x74, 0xd6, 0x56, 0xd3,
        ]))),
    ]
}

struct DummyHciServer {
    inputs: Vec<H4Packet>,
    handled_index: usize,
}

impl DummyHciServer {
    fn new() -> Self {
        DummyHciServer {
            inputs: vec![],
            handled_index: 0,
        }
    }

    fn read(&mut self) -> Option<H4Packet> {
        if self.inputs.is_empty() {
            None
        } else {
            Some(self.inputs.remove(0))
        }
    }

    fn write(&mut self, packet: H4Packet) {
        self.inputs.push(packet);
    }
}

fn main() {
    let v = u64::from_le_bytes([0xff, 0xff, 0xfb, 0xff, 0x7, 0xf8, 0xbf, 0x3d]);
    println!("Value: {}", v);
    println!("Hello, world!");
}
