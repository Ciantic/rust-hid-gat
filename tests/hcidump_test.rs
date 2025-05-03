use bt_only_headers::core::*;
use bt_only_headers::packer::*;

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
        let packet = packer.unpack::<H4Packet>();
        if d {
            println!("> {:?}", packet.unwrap());
        } else {
            println!("< {:?}", packet.unwrap());
        }
    }
}

#[test]
fn test_parsing2() {
    let data = parse_hci_dump_from_file("tests/hcidump-02.txt");

    for (d, bytes) in data {
        let mut packer = Packet::from_slice(&bytes);
        let packet = packer.unpack::<H4Packet>();
        if d {
            println!("> {:?}", packet.unwrap());
        } else {
            println!("< {:?}", packet.unwrap());
        }
    }
}
