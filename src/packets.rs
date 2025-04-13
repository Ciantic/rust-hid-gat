use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

// HCI ACL Data packets:
//
// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bc4ffa33-44ef-e93c-16c8-14aa99597cfc

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidHciPacketType(u8),
    InvalidHciAclPacketHandle(u16),
    InvalidL2capChannelId(u16),
    InsufficientData,
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HciPacket {
    Command(HciCommand),
    AclData(HciAclData),
    Event(HciEvent),
    Unknown(u8, Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HciCommand {
    pub opcode: u16,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HciEvent {
    pub event_code: u8,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum L2capPacket {
    Att(AttPdu),
    Smp(SmpPdu),
    Unknown(u16, Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum PacketBoundaryFlag {
    FirstNonFlushable = 0b00,
    Continuation = 0b01,
    FirstFlushable = 0b10,
    Deprecated = 0b11,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum BroadcastFlag {
    PointToPoint = 0b00,
    BdEdrBroadcast = 0b01,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HciAclData {
    pub handle: u16,
    pub pb: PacketBoundaryFlag,
    pub bc: BroadcastFlag,
    pub data: L2capPacket,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttPdu {
    pub opcode: u8,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmpPdu {
    pub code: u8,
    pub data: Vec<u8>,
}

// --- Packet Parsing and Serialization ---

impl HciPacket {
    /// Parses a complete HCI packet (including the type indicator) from a byte slice.
    pub fn from_bytes(data: &[u8]) -> ParseResult<Self> {
        if data.is_empty() {
            return Err(ParseError::InsufficientData);
        }
        let packet_type = data[0];
        let payload = &data[1..]; // Data after the type indicator

        match packet_type {
            1 => {
                // Command
                let cmd = HciCommand::from_bytes(payload)?;
                Ok(HciPacket::Command(cmd))
            }
            2 => {
                // ACL Data
                let acl = HciAclData::from_bytes(payload)?;
                Ok(HciPacket::AclData(acl))
            }
            4 => {
                // Event
                let evt = HciEvent::from_bytes(payload)?;
                Ok(HciPacket::Event(evt))
            }
            unknown => Ok(HciPacket::Unknown(unknown, payload.to_vec())),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            HciPacket::Command(cmd) => vec![vec![0x01], cmd.to_bytes()].concat(),
            HciPacket::AclData(acl) => vec![vec![0x02], acl.to_bytes()].concat(),
            HciPacket::Event(evt) => vec![vec![0x04], evt.to_bytes()].concat(),
            HciPacket::Unknown(packet_type, data) => {
                let mut bytes = Vec::with_capacity(1 + data.len());
                bytes.push(*packet_type);
                bytes.extend_from_slice(data);
                bytes
            }
        }
    }
}

impl HciCommand {
    /// Parses the payload of an HCI Command packet (Opcode, Length, Parameters).
    /// Assumes the HCI packet type indicator (0x01) has already been consumed.
    pub fn from_bytes(data: &[u8]) -> ParseResult<Self> {
        // Expects Opcode (2) + Length (1) = 3 bytes minimum payload
        if data.len() < 3 {
            return Err(ParseError::InsufficientData);
        }

        let opcode = Cursor::new(&data[0..2])
            .read_u16::<LittleEndian>()
            .map_err(|_| ParseError::InsufficientData)?; // Index 0, 1
        let param_len = data[2] as usize; // Index 2

        let total_expected_payload_len = 3 + param_len;
        if data.len() < total_expected_payload_len {
            return Err(ParseError::InsufficientData);
        }

        Ok(Self {
            opcode,
            // Parameters start after opcode (2) and length (1)
            data: data[3..total_expected_payload_len].to_vec(),
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        assert!(self.data.len() <= 255, "HCI Command data too long");
        let capacity = 4 + self.data.len(); // 1 (Type) + 2 (Opcode) + 1 (Len) + Params
        let mut bytes = Vec::with_capacity(capacity);
        bytes.write_u16::<LittleEndian>(self.opcode).unwrap();
        bytes.write_u8(self.data.len() as u8).unwrap();
        bytes.extend_from_slice(&self.data);
        bytes
    }
}

impl HciEvent {
    /// Parses the payload of an HCI Event packet (Event Code, Length, Parameters).
    /// Assumes the HCI packet type indicator (0x04) has already been consumed.
    pub fn from_bytes(data: &[u8]) -> ParseResult<Self> {
        // Expects Event Code (1) + Length (1) = 2 bytes minimum payload
        if data.len() < 2 {
            return Err(ParseError::InsufficientData);
        }
        let event_code = data[0]; // Index 0
        let param_len = data[1] as usize; // Index 1

        let total_expected_payload_len = 2 + param_len;
        if data.len() < total_expected_payload_len {
            return Err(ParseError::InsufficientData);
        }

        Ok(Self {
            event_code,
            // Parameters start after event code (1) and length (1)
            data: data[2..total_expected_payload_len].to_vec(),
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        assert!(self.data.len() <= 255, "HCI Event data too long");
        let capacity = 3 + self.data.len(); // 1 (Type) + 1 (Code) + 1 (Len) + Params
        let mut bytes = Vec::with_capacity(capacity);
        bytes.write_u8(self.event_code).unwrap();
        bytes.write_u8(self.data.len() as u8).unwrap();
        bytes.extend_from_slice(&self.data);
        bytes
    }
}

impl L2capPacket {
    /// Parses an L2CAP PDU (Length, CID, Payload).
    pub fn from_bytes(data: &[u8]) -> ParseResult<Self> {
        // L2CAP Basic Header: Length (2) + CID (2) = 4 bytes minimum
        if data.len() < 4 {
            return Err(ParseError::InsufficientData);
        }
        let l2cap_payload_len = Cursor::new(&data[0..2])
            .read_u16::<LittleEndian>()
            .map_err(|_| ParseError::InsufficientData)? as usize; // Index 0, 1

        let channel_id = Cursor::new(&data[2..4])
            .read_u16::<LittleEndian>()
            .map_err(|_| ParseError::InsufficientData)?; // Index 2, 3

        let total_expected_l2cap_len = 4 + l2cap_payload_len;
        if data.len() < total_expected_l2cap_len {
            return Err(ParseError::InsufficientData);
        }

        // Slice the actual inner payload data (ATT, SMP, etc.)
        let inner_payload_data = &data[4..total_expected_l2cap_len];

        let l2cap_packet_data = match channel_id {
            0x0004 => {
                // ATT CID
                let att_pdu = AttPdu::from_bytes(inner_payload_data)?;
                L2capPacket::Att(att_pdu)
            }
            0x0006 => {
                // SMP CID (BLE)
                let smp_pdu = SmpPdu::from_bytes(inner_payload_data)?;
                L2capPacket::Smp(smp_pdu)
            }
            _ => L2capPacket::Unknown(channel_id, inner_payload_data.to_vec()),
        };

        Ok(l2cap_packet_data)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let (channel_id, inner_pdu_bytes) = self.to_bytes_with_cid();
        let l2cap_payload_len = inner_pdu_bytes.len();
        assert!(
            l2cap_payload_len <= u16::MAX as usize,
            "L2CAP payload too long"
        );

        let capacity = 4 + l2cap_payload_len;
        let mut bytes = Vec::with_capacity(capacity);

        bytes
            .write_u16::<LittleEndian>(l2cap_payload_len as u16)
            .unwrap();
        bytes.write_u16::<LittleEndian>(channel_id).unwrap();
        bytes.extend_from_slice(&inner_pdu_bytes);

        bytes
    }
    fn to_bytes_with_cid(&self) -> (u16, Vec<u8>) {
        match self {
            L2capPacket::Att(pdu) => (self.channel_id(), pdu.to_bytes()),
            L2capPacket::Smp(pdu) => (self.channel_id(), pdu.to_bytes()),
            L2capPacket::Unknown(cid, data) => (*cid, data.clone()),
        }
    }
    pub fn channel_id(&self) -> u16 {
        match self {
            L2capPacket::Att(_) => 0x0004,
            L2capPacket::Smp(_) => 0x0006,
            L2capPacket::Unknown(cid, _) => *cid,
        }
    }
}

impl HciAclData {
    /// Assumes the HCI packet type indicator (0x02) has already been consumed
    pub fn from_bytes(data: &[u8]) -> ParseResult<Self> {
        if data.len() < 4 {
            return Err(ParseError::InsufficientData);
        }

        let handle_and_flags = Cursor::new(&data[0..2])
            .read_u16::<LittleEndian>()
            .map_err(|_| ParseError::InsufficientData)?;

        // Handle range: 0x000 to 0xEFF (all other values reserved for future use)
        let handle = handle_and_flags & 0x0FFF; // Mask to 12 bits
        if handle > 0x0EFF {
            return Err(ParseError::InvalidHciAclPacketHandle(handle_and_flags));
        }

        let pb = match (handle_and_flags >> 12) & 0b11 {
            0 => PacketBoundaryFlag::FirstNonFlushable,
            1 => PacketBoundaryFlag::Continuation,
            2 => PacketBoundaryFlag::FirstFlushable,
            _ => PacketBoundaryFlag::Deprecated,
        };

        let bc = match (handle_and_flags >> 14) & 0b11 {
            0 => BroadcastFlag::PointToPoint,
            1 => BroadcastFlag::BdEdrBroadcast,
            _ => return Err(ParseError::InvalidHciAclPacketHandle(handle_and_flags)),
        };

        let l2cap_packet = L2capPacket::from_bytes(&data[4..])?;

        Ok(HciAclData {
            handle,
            pb,
            bc,
            data: l2cap_packet,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let l2cap_pdu_bytes = self.data.to_bytes();
        let hci_total_data_len = l2cap_pdu_bytes.len();
        assert!(
            hci_total_data_len <= u16::MAX as usize,
            "HCI ACL data (L2CAP PDU) too long"
        );
        assert!(self.handle <= 0x0FFF, "Handle value exceeds 12 bits");

        let capacity = 5 + hci_total_data_len;
        let mut bytes = Vec::with_capacity(capacity);

        let mut handle_and_flags = self.handle & 0x0FFF; // Mask to 12 bits
        handle_and_flags |= (self.pb as u16) << 12; // Set Packet Boundary Flag
        handle_and_flags |= (self.bc as u16) << 14; // Set Broadcast Flag

        bytes.write_u16::<LittleEndian>(handle_and_flags).unwrap();
        bytes
            .write_u16::<LittleEndian>(hci_total_data_len as u16)
            .unwrap();
        bytes.extend_from_slice(&l2cap_pdu_bytes);

        bytes
    }
}

impl AttPdu {
    /// Parses an ATT PDU payload (Opcode, Parameters).
    pub fn from_bytes(data: &[u8]) -> ParseResult<Self> {
        if data.is_empty() {
            return Err(ParseError::InsufficientData);
        }
        let opcode = data[0]; // Index 0
        Ok(AttPdu {
            opcode,
            data: data[1..].to_vec(), // Parameters start after opcode
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let capacity = 1 + self.data.len();
        let mut bytes = Vec::with_capacity(capacity);
        bytes.write_u8(self.opcode).unwrap();
        bytes.extend_from_slice(&self.data);
        bytes
    }
}

impl SmpPdu {
    /// Parses an SMP PDU payload (Code, Parameters).
    pub fn from_bytes(data: &[u8]) -> ParseResult<Self> {
        if data.is_empty() {
            return Err(ParseError::InsufficientData);
        }
        let code = data[0]; // Index 0
        Ok(SmpPdu {
            code,
            data: data[1..].to_vec(), // Parameters start after code
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let capacity = 1 + self.data.len();
        let mut bytes = Vec::with_capacity(capacity);
        bytes.write_u8(self.code).unwrap();
        bytes.extend_from_slice(&self.data);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hci_command() {
        let data: Vec<u8> = vec![0x01, 0x03, 0x0C, 0x00];
        let result = HciPacket::from_bytes(&data).unwrap();
        match result {
            HciPacket::Command(cmd) => {
                assert_eq!(cmd.opcode, 0x0C03);
                assert_eq!(cmd.data.len(), 0);
            }
            _ => panic!("Expected HciCommand"),
        }
    }

    #[test]
    fn test_parse_hci_command_with_params() {
        let data_params: Vec<u8> = vec![0x01, 0xCD, 0xAB, 0x02, 0xFE, 0xDC];
        let result_params = HciPacket::from_bytes(&data_params).unwrap();
        match result_params {
            HciPacket::Command(cmd) => {
                assert_eq!(cmd.opcode, 0xABCD);
                assert_eq!(cmd.data, vec![0xFE, 0xDC]);
            }
            _ => panic!("Expected HciCommand with params"),
        }
    }

    #[test]
    fn test_parse_hci_event() {
        let data: Vec<u8> = vec![0x04, 0x05, 0x04, 0x00, 0x40, 0x00, 0x13];
        let result = HciPacket::from_bytes(&data).unwrap();
        match result {
            HciPacket::Event(evt) => {
                assert_eq!(evt.event_code, 0x05);
                assert_eq!(evt.data.len(), 4);
                assert_eq!(evt.data, vec![0x00, 0x40, 0x00, 0x13]);
            }
            _ => panic!("Expected HciEvent"),
        }
    }

    #[test]
    fn test_parse_hci_acl_data() {
        let att_handle: u16 = 0x001A;
        let att_value: Vec<u8> = vec![0x01];
        let att_payload: Vec<u8> =
            [&[0x12], &att_handle.to_le_bytes()[..], &att_value[..]].concat();
        let l2cap_payload_len: u16 = att_payload.len() as u16; // 4
        let l2cap_cid: u16 = 0x0004; // ATT
        let l2cap_pdu: Vec<u8> = [
            &l2cap_payload_len.to_le_bytes()[..],
            &l2cap_cid.to_le_bytes()[..],
            &att_payload[..],
        ]
        .concat();
        let hci_total_data_len: u16 = l2cap_pdu.len() as u16; // 8
        let handle: u16 = 0x0040;
        let data: Vec<u8> = vec![
            0x02,
            (handle & 0x0FFF).to_le_bytes()[0],
            (handle & 0x0FFF).to_le_bytes()[1],
            hci_total_data_len.to_le_bytes()[0],
            hci_total_data_len.to_le_bytes()[1],
            l2cap_pdu[0],
            l2cap_pdu[1],
            l2cap_pdu[2],
            l2cap_pdu[3],
            l2cap_pdu[4],
            l2cap_pdu[5],
            l2cap_pdu[6],
            l2cap_pdu[7],
        ];

        let result = HciPacket::from_bytes(&data).unwrap(); // Use the main entry point
        match result {
            HciPacket::AclData(acl) => {
                assert_eq!(acl.handle, handle);
                let cid = acl.data.channel_id();
                match acl.data {
                    L2capPacket::Att(att) => {
                        assert_eq!(cid, l2cap_cid);
                        assert_eq!(att.opcode, 0x12);
                        let expected_att_data =
                            [&att_handle.to_le_bytes()[..], &att_value[..]].concat();
                        assert_eq!(att.data, expected_att_data);
                    }
                    _ => panic!("Expected ATT PDU"),
                }
            }
            _ => panic!("Expected ACL Data Packet"),
        }
    }
    #[test]
    fn test_parse_unknown_packet() {
        let data: Vec<u8> = vec![0x08, 0x01, 0x02, 0x03];
        let result = HciPacket::from_bytes(&data).unwrap();
        match result {
            HciPacket::Unknown(packet_type, payload) => {
                assert_eq!(packet_type, 0x08);
                assert_eq!(payload, vec![0x01, 0x02, 0x03]);
            }
            _ => panic!("Expected Unknown packet"),
        }
    }

    #[test]
    fn test_insufficient_data_hci_header() {
        assert!(matches!(
            HciPacket::from_bytes(&[0x01]),
            Err(ParseError::InsufficientData)
        )); // Only type
        assert!(matches!(
            HciPacket::from_bytes(&[0x01, 0x03]),
            Err(ParseError::InsufficientData)
        )); // Too short for command payload hdr
        assert!(matches!(
            HciPacket::from_bytes(&[0x02, 0x40, 0x00]),
            Err(ParseError::InsufficientData)
        )); // Too short for ACL payload hdr
        assert!(matches!(
            HciPacket::from_bytes(&[0x04]),
            Err(ParseError::InsufficientData)
        )); // Only type
        assert!(matches!(
            HciPacket::from_bytes(&[0x04, 0x05]),
            Err(ParseError::InsufficientData)
        )); // Too short for Event payload hdr
    }

    #[test]
    fn test_insufficient_data_hci_payload() {
        assert!(matches!(
            HciPacket::from_bytes(&[0x01, 0xCD, 0xAB, 0x02, 0xFE]),
            Err(ParseError::InsufficientData)
        )); // Command payload short
        assert!(matches!(
            HciPacket::from_bytes(&[0x04, 0x05, 0x04, 0x00, 0x40, 0x00]),
            Err(ParseError::InsufficientData)
        )); // Event payload short
        let data3: Vec<u8> = vec![0x02, 0x40, 0x00, 8, 0, 4, 0, 4, 0, 0x12, 0x1A, 0x00]; // ACL payload (L2CAP part) short
        assert!(matches!(
            HciPacket::from_bytes(&data3),
            Err(ParseError::InsufficientData)
        ));
    }

    #[test]
    fn test_insufficient_data_l2cap() {
        let data: Vec<u8> = vec![0x02, 0x40, 0x00, 8, 0, 5, 0, 4, 0, 0x12, 0x1A, 0x00, 0x01]; // L2CAP hdr says len=5, but only 4 bytes follow CID
        assert!(matches!(
            HciPacket::from_bytes(&data),
            Err(ParseError::InsufficientData)
        ));

        let data2: Vec<u8> = vec![0x02, 0x40, 0x00, 3, 0, 0, 0, 0]; // HCI ACL payload len = 3, too short for L2CAP header
        assert!(matches!(
            HciPacket::from_bytes(&data2),
            Err(ParseError::InsufficientData)
        ));
    }

    #[test]
    fn test_insufficient_data_att_smp() {
        // L2CAP header okay (len=0), but ATT requires opcode
        let data_att: Vec<u8> = vec![0x02, 0x40, 0x00, 4, 0, 0, 0, 4, 0];
        assert!(matches!(
            HciPacket::from_bytes(&data_att),
            Err(ParseError::InsufficientData)
        ));

        // L2CAP header okay (len=0), but SMP requires code
        let data_smp: Vec<u8> = vec![0x02, 0x40, 0x00, 4, 0, 0, 0, 6, 0];
        assert!(matches!(
            HciPacket::from_bytes(&data_smp),
            Err(ParseError::InsufficientData)
        ));
    }

    // --- SERIALIZATION TESTS ---

    #[test]
    fn test_serialize_hci_command() {
        let cmd = HciCommand {
            opcode: 0x0C03,
            data: vec![],
        };
        assert_eq!(cmd.to_bytes(), vec![0x03, 0x0C, 0x00]);
        assert_eq!(
            HciPacket::Command(cmd).to_bytes(),
            vec![0x01, 0x03, 0x0C, 0x00]
        );

        let cmd_params = HciCommand {
            opcode: 0xABCD,
            data: vec![0xFE, 0xDC],
        };
        assert_eq!(cmd_params.to_bytes(), vec![0xCD, 0xAB, 0x02, 0xFE, 0xDC]);
        assert_eq!(
            HciPacket::Command(cmd_params).to_bytes(),
            vec![0x01, 0xCD, 0xAB, 0x02, 0xFE, 0xDC]
        );
    }

    #[test]
    fn test_serialize_hci_event() {
        let evt = HciEvent {
            event_code: 0x05,
            data: vec![0x00, 0x40, 0x00, 0x13],
        };
        assert_eq!(evt.to_bytes(), vec![0x05, 0x04, 0x00, 0x40, 0x00, 0x13]);
        assert_eq!(
            HciPacket::Event(evt).to_bytes(),
            vec![0x04, 0x05, 0x04, 0x00, 0x40, 0x00, 0x13]
        );
    }

    #[test]
    fn test_serialize_hci_acl_data() {
        let att_handle = 0x001A_u16;
        let att_value = vec![0x01];
        let att_pdu = AttPdu {
            opcode: 0x12,
            data: [&att_handle.to_le_bytes()[..], &att_value[..]].concat(),
        };
        let l2cap_packet = L2capPacket::Att(att_pdu);
        let acl_packet = HciAclData {
            handle: 0x0040,
            pb: PacketBoundaryFlag::FirstNonFlushable,
            bc: BroadcastFlag::PointToPoint,
            data: l2cap_packet,
        };
        assert_eq!(
            acl_packet.to_bytes(),
            vec![0x40, 0x00, 8, 0, 4, 0, 4, 0, 0x12, 0x1A, 0x00, 0x01]
        );
        assert_eq!(
            HciPacket::AclData(acl_packet).to_bytes(),
            vec![0x02, 0x40, 0x00, 8, 0, 4, 0, 4, 0, 0x12, 0x1A, 0x00, 0x01]
        );
    }

    #[test]
    fn test_serialize_hci_unknown_packet() {
        let unknown_packet = HciPacket::Unknown(0x99, vec![0x01, 0x02, 0x03]);
        assert_eq!(unknown_packet.to_bytes(), vec![0x99, 0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_serialize_att_pdu() {
        let att_handle = 0x001A_u16;
        let att_value = vec![0x01];
        let att_pdu = AttPdu {
            opcode: 0x12,
            data: [&att_handle.to_le_bytes()[..], &att_value[..]].concat(),
        };
        let expected_bytes: Vec<u8> = vec![0x12, 0x1A, 0x00, 0x01];
        assert_eq!(att_pdu.to_bytes(), expected_bytes);
    }

    #[test]
    fn test_serialize_smp_pdu() {
        let smp_pdu = SmpPdu {
            code: 0x01,
            data: vec![0x02, 0x03, 0x04],
        };
        let expected_bytes: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];
        assert_eq!(smp_pdu.to_bytes(), expected_bytes);
    }

    #[test]
    fn test_serialize_l2cap_packet() {
        let att_handle = 0x001A_u16;
        let att_value = vec![0x01];
        let att_pdu = AttPdu {
            opcode: 0x12,
            data: [&att_handle.to_le_bytes()[..], &att_value[..]].concat(),
        };
        let l2cap_packet = L2capPacket::Att(att_pdu);
        assert_eq!(
            l2cap_packet.to_bytes(),
            vec![4, 0, 4, 0, 0x12, 0x1A, 0x00, 0x01]
        );

        let smp_pdu = SmpPdu {
            code: 0x01,
            data: vec![0x02, 0x03, 0x04],
        };
        let l2cap_packet_smp = L2capPacket::Smp(smp_pdu);
        assert_eq!(
            l2cap_packet_smp.to_bytes(),
            vec![4, 0, 6, 0, 0x01, 0x02, 0x03, 0x04]
        );

        let unknown_data = vec![0xAA, 0xBB, 0xCC];
        let l2cap_packet_unknown = L2capPacket::Unknown(0x00F0, unknown_data.clone());
        assert_eq!(
            l2cap_packet_unknown.to_bytes(),
            vec![3, 0, 0xF0, 0, 0xAA, 0xBB, 0xCC]
        );
    }

    // --- Round trip tests (serialize then parse) ---

    #[test]
    fn test_round_trip_command() {
        let original = HciPacket::Command(HciCommand {
            opcode: 0x1001,
            data: vec![],
        });
        let bytes = original.to_bytes();
        let parsed = HciPacket::from_bytes(&bytes).unwrap();
        assert_eq!(original, parsed);
        assert_eq!(bytes, parsed.to_bytes());
    }

    #[test]
    fn test_round_trip_event() {
        let original = HciPacket::Event(HciEvent {
            event_code: 0x0E,
            data: vec![0x01, 0x01, 0x10, 0x00, 0x01, 0x02, 0x03, 0x04],
        });
        let bytes = original.to_bytes();
        let parsed = HciPacket::from_bytes(&bytes).unwrap();
        assert_eq!(original, parsed);
        assert_eq!(bytes, parsed.to_bytes());
    }

    #[test]
    fn test_round_trip_acl_att() {
        let att_handle = 0x002B_u16;
        let att_value = vec![0x02, 0x03];
        let att_pdu = AttPdu {
            opcode: 0x52,
            data: [&att_handle.to_le_bytes()[..], &att_value[..]].concat(),
        };
        let l2cap_packet = L2capPacket::Att(att_pdu);
        let original = HciPacket::AclData(HciAclData {
            handle: 0x0055,
            pb: PacketBoundaryFlag::FirstNonFlushable,
            bc: BroadcastFlag::PointToPoint,
            data: l2cap_packet,
        });

        let bytes = original.to_bytes();
        let parsed = HciPacket::from_bytes(&bytes).unwrap();
        assert_eq!(&original, &parsed);
        assert_eq!(bytes, parsed.to_bytes());
    }

    #[test]
    fn test_round_trip_acl_smp() {
        let smp_pdu = SmpPdu {
            code: 0x02,
            data: vec![0x11, 0x22, 0x33],
        };
        let l2cap_packet = L2capPacket::Smp(smp_pdu);
        let original = HciPacket::AclData(HciAclData {
            handle: 0x0066,
            pb: PacketBoundaryFlag::FirstNonFlushable,
            bc: BroadcastFlag::PointToPoint,
            data: l2cap_packet,
        });

        let bytes = original.to_bytes();
        let parsed = HciPacket::from_bytes(&bytes).unwrap();
        assert_eq!(&original, &parsed);
        assert_eq!(bytes, parsed.to_bytes());
    }

    #[test]
    fn test_pb_bc_flag_parse() {
        {
            let data: Vec<u8> = vec![0x02, 0x40, 0x00, 8, 0, 4, 0, 4, 0, 0x12, 0x1A, 0x00, 0x00];
            let res = HciPacket::from_bytes(&data).unwrap();
            match res {
                HciPacket::AclData(acl) => {
                    assert_eq!(acl.handle, 0x0040);
                    assert_eq!(acl.pb, PacketBoundaryFlag::FirstNonFlushable);
                    assert_eq!(acl.bc, BroadcastFlag::PointToPoint);
                }
                _ => panic!("Expected ACL Data Packet"),
            }
        }

        {
            let data: Vec<u8> = vec![
                0x2, 0x40, 0x10, 0x7, 0x0, 0x3, 0x0, 0x4, 0x0, 0xa, 0x17, 0x0,
            ];
            let res = HciPacket::from_bytes(&data).unwrap();
            match res {
                HciPacket::AclData(acl) => {
                    assert_eq!(acl.handle, 0x0040);
                    assert_eq!(acl.pb, PacketBoundaryFlag::Continuation);
                    assert_eq!(acl.bc, BroadcastFlag::PointToPoint);
                }
                _ => panic!("Expected ACL Data Packet"),
            }
        }
        {
            let data: Vec<u8> = vec![
                0x2, 0x40, 0x20, 0x7, 0x0, 0x3, 0x0, 0x4, 0x0, 0xa, 0x17, 0x0,
            ];
            let res = HciPacket::from_bytes(&data).unwrap();
            match res {
                HciPacket::AclData(acl) => {
                    assert_eq!(acl.handle, 0x0040);
                    assert_eq!(acl.pb, PacketBoundaryFlag::FirstFlushable);
                    assert_eq!(acl.bc, BroadcastFlag::PointToPoint);
                }
                _ => panic!("Expected ACL Data Packet"),
            }
        }
        {
            let data: Vec<u8> = vec![
                0x2, 0x40, 0x30, 0x7, 0x0, 0x3, 0x0, 0x4, 0x0, 0xa, 0x17, 0x0,
            ];
            let res = HciPacket::from_bytes(&data).unwrap();
            match res {
                HciPacket::AclData(acl) => {
                    assert_eq!(acl.handle, 0x0040);
                    assert_eq!(acl.pb, PacketBoundaryFlag::Deprecated);
                    assert_eq!(acl.bc, BroadcastFlag::PointToPoint);
                }
                _ => panic!("Expected ACL Data Packet"),
            }
        }
        {
            let data: Vec<u8> = vec![0x02, 0x40, 0x40, 8, 0, 4, 0, 4, 0, 0x12, 0x1A, 0x00, 0x00];
            let res = HciPacket::from_bytes(&data).unwrap();
            match res {
                HciPacket::AclData(acl) => {
                    assert_eq!(acl.handle, 0x0040);
                    assert_eq!(acl.pb, PacketBoundaryFlag::FirstNonFlushable);
                    assert_eq!(acl.bc, BroadcastFlag::BdEdrBroadcast);
                }
                _ => panic!("Expected ACL Data Packet"),
            }
        }
    }

    #[test]
    fn test_pb_bc_serialize_flags() {
        {
            assert_eq!(
                HciAclData {
                    handle: 0x0040,
                    pb: PacketBoundaryFlag::FirstNonFlushable,
                    bc: BroadcastFlag::PointToPoint,
                    data: L2capPacket::Unknown(0xBEEF, vec![]),
                }
                .to_bytes(),
                vec![0x40, 0x00, 0x04, 0x00, 0x00, 0x00, 0xEF, 0xBE]
            );
        }
        {
            assert_eq!(
                HciAclData {
                    handle: 0x0040,
                    pb: PacketBoundaryFlag::Continuation,
                    bc: BroadcastFlag::PointToPoint,
                    data: L2capPacket::Unknown(0xBEEF, vec![]),
                }
                .to_bytes(),
                vec![0x40, 0x10, 0x04, 0x00, 0x00, 0x00, 0xEF, 0xBE]
            );
        }
        {
            assert_eq!(
                HciAclData {
                    handle: 0x0040,
                    pb: PacketBoundaryFlag::FirstFlushable,
                    bc: BroadcastFlag::PointToPoint,
                    data: L2capPacket::Unknown(0xBEEF, vec![]),
                }
                .to_bytes(),
                vec![0x40, 0x20, 0x04, 0x00, 0x00, 0x00, 0xEF, 0xBE]
            );
        }
        {
            assert_eq!(
                HciAclData {
                    handle: 0x0040,
                    pb: PacketBoundaryFlag::Deprecated,
                    bc: BroadcastFlag::PointToPoint,
                    data: L2capPacket::Unknown(0xBEEF, vec![]),
                }
                .to_bytes(),
                vec![0x40, 0x30, 0x04, 0x00, 0x00, 0x00, 0xEF, 0xBE]
            );
        }
        {
            assert_eq!(
                HciAclData {
                    handle: 0x0040,
                    pb: PacketBoundaryFlag::FirstNonFlushable,
                    bc: BroadcastFlag::BdEdrBroadcast,
                    data: L2capPacket::Unknown(0xBEEF, vec![]),
                }
                .to_bytes(),
                vec![0x40, 0x40, 0x04, 0x00, 0x00, 0x00, 0xEF, 0xBE]
            );
        }
    }
}
