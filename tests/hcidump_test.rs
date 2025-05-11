use std::collections::VecDeque;

use bt_only_headers::hcimanager::AppMsg;
use bt_only_headers::hcimanager::HciManager;
use bt_only_headers::hcimanager::MsgProcessor;
use bt_only_headers::messages::*;
use bt_only_headers::packer::*;
use bt_only_headers::socket::MockSocket;
use bt_only_headers::socket::Socket;

fn parse_hci_dump(dump: String) -> Vec<(bool, Vec<u8>)> {
    // Parse the HCI dump like this:
    // If line starts with "<" then it is a command from the host to the controller
    // If line starts with ">" then it is a command from the controller to the host
    // If line starts wit " " then it is continued data from the previous line

    // true = host to controller
    // false = controller to host

    let mut result = Vec::new();
    let mut current: Option<(bool, Vec<u8>)> = None;

    for line in dump.lines() {
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }
        let (direction, data_str) = match line.chars().next() {
            Some('<') => (true, &line[1..]),
            Some('>') => (false, &line[1..]),
            Some(' ') => {
                if let Some((_, ref mut bytes)) = current {
                    let data = line.trim();
                    for byte in data.split_whitespace() {
                        if let Ok(b) = u8::from_str_radix(byte, 16) {
                            bytes.push(b);
                        }
                    }
                }
                continue;
            }
            _ => continue,
        };

        let data = data_str
            .trim()
            .split_whitespace()
            .filter_map(|b| u8::from_str_radix(b, 16).ok())
            .collect::<Vec<u8>>();

        if let Some(c) = current.take() {
            result.push(c);
        }
        current = Some((direction, data));
    }

    if let Some(c) = current {
        result.push(c);
    }

    result
}

fn parse_hci_dump_from_file(file: &str) -> Vec<(bool, Vec<u8>)> {
    let dump = std::fs::read_to_string(file).unwrap();
    parse_hci_dump(dump)
}

#[test]
fn test_parse_hci_dump() {
    let dump = r#"
< 01 03 0C 00
> 04 0E 04 01 03 0C 00
< 01 01 0C 08 FF FF FB FF 07 F8 BF 3D
> 04 0E 
  04 01
"#;

    let parsed = parse_hci_dump(dump.to_string());

    assert_eq!(parsed[0].0, true);
    assert_eq!(parsed[0].1, vec![1, 3, 12, 0]);
    assert_eq!(parsed[1].0, false);
    assert_eq!(parsed[1].1, vec![4, 14, 4, 1, 3, 12, 0]);
    assert_eq!(parsed[2].0, true);
    assert_eq!(
        parsed[2].1,
        vec![1, 1, 12, 8, 255, 255, 251, 255, 7, 248, 191, 61]
    );
}

#[test]
fn test_parsing() {
    let data = parse_hci_dump_from_file("tests/hcidump-01.txt");

    for (d, bytes) in data {
        let mut packer = Packet::from_slice(&bytes);
        let packet = packer.unpack::<H4Packet>().unwrap();
        if d {
            println!("> {:?}", &packet);
        } else {
            println!("< {:?}", &packet);
        }
        // Test that serialization works
        let mut test_packer = Packet::new();
        test_packer.pack::<H4Packet>(&packet).unwrap();
        assert_eq!(test_packer.get_bytes(), &bytes);
    }
}

#[test]
fn test_parsing2() {
    let data = parse_hci_dump_from_file("tests/hcidump-02.txt");

    for (d, bytes) in data {
        let mut packer = Packet::from_slice(&bytes);
        let packet = packer.unpack::<H4Packet>().unwrap();
        if d {
            println!("> {:?}", &packet);
        } else {
            println!("< {:?}", &packet);
        }

        // Test that serialization works
        let mut test_packer = Packet::new();
        test_packer.pack::<H4Packet>(&packet).unwrap();
        assert_eq!(test_packer.get_bytes(), &bytes);
    }
}

#[test]
fn test_parsing3() {
    let data = parse_hci_dump_from_file("tests/hcidump-03.txt");

    for (d, bytes) in data {
        let mut packer = Packet::from_slice(&bytes);
        let packet = packer.unpack::<H4Packet>().unwrap();
        if d {
            println!("> {:?}", &packet);
        } else {
            println!("< {:?}", &packet);
        }

        // Test that serialization works
        let mut test_packer = Packet::new();
        test_packer.pack::<H4Packet>(&packet).unwrap();
        assert_eq!(test_packer.get_bytes(), &bytes);
    }
}

#[test]
fn test_hcimanager() {
    let data = parse_hci_dump_from_file("tests/hcidump-03.txt");
    let mut socket = Box::new(MockSocket::new(data.into()));
    let init_bluetooth = vec![
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
        (HciCommand::LeSetRandomAddress(BdAddr([137, 146, 48, 216, 52, 243]))),
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
        (HciCommand::LeReadLocalP256PublicKey),
        (HciCommand::LeSetRandomAddress(BdAddr([6, 51, 116, 214, 86, 211]))),
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
    .map(|f| (true, H4Packet::Command(f.clone()).to_bytes()))
    .collect::<VecDeque<(bool, Vec<u8>)>>();

    let mut mgr = HciManager::new().unwrap();
    let mut socket = MockSocket::new(init_bluetooth);
    let mut queue = VecDeque::new();

    while let Some(packet) = socket.read().unwrap() {
        queue.push_front(AppMsg::Recv(packet.clone()));

        while let Some(msg) = queue.pop_front() {
            // Process the message
            queue.append(&mut mgr.process(msg.clone()).unwrap().into());

            // Handle the message in main
            match msg {
                AppMsg::Send(packet) => {
                    socket.write(packet).unwrap();
                }
                AppMsg::Recv(packet) => {
                    panic!("Recv should come only out of socket: {:?}", packet);
                }
                _ => {}
            }
        }
    }

    assert_eq!(queue.len(), 0, "Queue is now empty");
}
