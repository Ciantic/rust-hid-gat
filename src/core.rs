#[derive(Debug, Clone, PartialEq, Eq)]
pub enum H4Packet {
    /// id = &[0x01]
    HciCommand(HciCommand),
    /// id = &[0x04]
    HciEvent(HciEventMsg),
    /// id = &[0x02]
    HciAcl {
        /// bits = 12
        connection_handle: ConnectionHandle,
        /// bits = 2
        pb: PacketBoundaryFlag,
        /// bits = 2
        bc: BroadcastFlag,
        msg: AclMessage,
        // len: u16,
        // data: L2CapMessage,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AclMessage {
    // Id is:
    //        +-------------------------------------------+
    //        | ACL LEN  | L2CAP LEN | L2CAP CID | OPCODE |
    //        | 2 bytes  | 2 bytes   | 2 bytes   | 1 byte |
    //        +-------------------------------------------+
    //
    /// id = &[0x15, 0x00, 0x11, 0x00, 0x06, 0x00, 0x03]
    SmpPairingConfirmation { confirm_value: u128 },
    /// id = &[0x15, 0x00, 0x11, 0x00, 0x06, 0x00, 0x04]
    SmpPairingRandom { random_value: u128 },
    /// id = &[0x0b, 0x00, 0x07, 0x00, 0x06, 0x00, 0x01]
    SmpPairingRequest {
        io_capability: IOCapability,
        oob_data_flag: OOBDataFlag,
        authentication_requirements: AuthenticationRequirements,
        max_encryption_key_size: u8,
        initiator_key_distribution: KeyDistributionFlags,
        responder_key_distribution: KeyDistributionFlags,
    },
    /// id = &[0x0b, 0x00, 0x07, 0x00, 0x06, 0x00, 0x02]
    SmpPairingResponse {
        io_capability: IOCapability,
        oob_data_flag: OOBDataFlag,
        authentication_requirements: AuthenticationRequirements,
        max_encryption_key_size: u8,
        initiator_key_distribution: KeyDistributionFlags,
        responder_key_distribution: KeyDistributionFlags,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HciCommand {
    /// id = &[0x01, 0x0c]
    SetEventMask {
        event_mask: u64,
        // bit = 0
        // size = 64
        // inquire_complete_event: bool,
        // inquiry_result_event: bool,
        // .. nearly 64 flags .. not worth it
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HciEventMsg {
    /// id = &[0x3e, 0x13, 0x01]
    LeConnectionComplete {
        status: HciStatus,
        connection_handle: u16,
        role: Role,
        peer_address_type: AddressType,
        peer_address: [u8; 6],
        connection_interval: u16,
        peripheral_latency: u16,
        supervision_timeout: u16,
        central_clock_accuracy: ClockAccuracy,
    },
    /// id = &[0x0E, 0x04]
    CommandComplete {
        num_hci_command_packets: u8,
        command_opcode: u16,
        status: HciStatus,
    },
    /// id = &[0x0F, 0x04]
    CommandStatus {
        status: HciStatus,
        num_hci_command_packets: u8,
        command_opcode: u16,
    },
    // Other messages...
    // #[deku(id_pat = "_")]
    // Unreachable,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
pub enum PacketBoundaryFlag {
    #[default]
    FirstNonFlushable = 0b00,
    Continuation = 0b01,
    FirstFlushable = 0b10,
    Deprecated = 0b11,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
pub enum BroadcastFlag {
    #[default]
    PointToPoint = 0b00,
    BdEdrBroadcast = 0b01,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HciStatus {
    /// id = &[0x00]
    Success,
    /// id = _
    Failure(u8),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Role {
    Central = 0,
    Peripheral = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AddressType {
    Public = 0,
    Random = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ClockAccuracy {
    Ppm500 = 0,
    Ppm250 = 1,
    Ppm150 = 2,
    Ppm100 = 3,
    Ppm75 = 4,
    Ppm50 = 5,
    Ppm30 = 6,
    Ppm20 = 7,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConnectionHandle(pub u16); // max value 0x0EFF

// SMP

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct KeyDistributionFlags {
    pub enc_key: bool,
    pub id_key: bool,
    pub sign_key: bool,
    pub link_key: bool,
    // pub reserved: (bool, bool, bool, bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AuthenticationRequirements {
    pub bonding: bool,
    pub mitm_protection: bool,
    pub secure_connections: bool,
    pub keypress_notification: bool,
    pub ct2: bool,
    // pub reserved: (bool, bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IOCapability {
    DisplayOnly = 0x00,
    DisplayYesNo = 0x01,
    KeyboardOnly = 0x02,
    NoInputNoOutput = 0x03,
    KeyboardDisplay = 0x04,
    // Reserved(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OOBDataFlag {
    OobNotAvailable = 0x00,
    OobAvailable = 0x01,
    // Reserved(u8),
}

/// SMP Pairing failures
///
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/security-manager-specification.html#UUID-edc160cf-62e1-c774-f84c-da67aaf4aa50
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmpPairingFailure {
    PasskeyEntryFailed = 0x01,
    OobNotAvailable = 0x02,
    AuthenticationRequirements = 0x03,
    ConfirmValueFailed = 0x04,
    PairingNotSupported = 0x05,
    EncryptionKeySize = 0x06,
    CommandNotSupported = 0x07,
    UnspecifiedReason = 0x08,
    RepeatedAttempts = 0x09,
    InvalidParameters = 0x0A,
    DhKeyCheckFailed = 0x0B,
    NumericComparisonFailed = 0x0C,
    BrEdrPairingInProgress = 0x0D,
    CrossTransportKeyDerivationGenerationNotAllowed = 0x0E,
    KeyRejected = 0x0F,
    Busy = 0x10,
}

#[cfg(test)]
mod tests {
    use crate::packer::*;

    use super::*;

    #[test]
    fn deserialize_connection_handle() {
        let mut packet = Packet::from_slice(&[0xEF, 0xBE]);
        let handle = ConnectionHandle::from_packet(&mut packet).unwrap();
        assert_eq!(handle, ConnectionHandle(0xBEEF));
    }

    #[test]
    fn deserialize_role() {
        let mut packet = Packet::from_slice(&[0x00]);
        let handle = Role::from_packet(&mut packet).unwrap();
        assert_eq!(handle, Role::Central);
    }

    #[test]
    fn hci_status() {
        let mut packet = Packet::from_slice(&[0x00]);
        let handle = HciStatus::from_packet(&mut packet).unwrap();
        assert_eq!(handle, HciStatus::Success);

        let mut packet = Packet::from_slice(&[0x05]);
        let handle = HciStatus::from_packet(&mut packet).unwrap();
        assert_eq!(handle, HciStatus::Failure(0x05));
    }
    #[test]
    fn serialize_hci_status() {
        let mut packet = Packet::from_slice(&[0x05]);

        HciStatus::Success.to_packet(&mut packet).unwrap();
        assert_eq!(packet.get_bytes(), &[0x00]);

        HciStatus::Failure(0x05).to_packet(&mut packet).unwrap();
        assert_eq!(packet.get_bytes(), &[0x00, 0x05]);
    }

    #[test]
    fn test_event() {
        const DATA: [u8; 21] = [
            0x3e, 0x13, 0x1, 0x0, 0x40, 0x0, 0x1, 0x0, 0x26, 0xe, 0xd6, 0xe8, 0xc2, 0x50, 0x30,
            0x0, 0x0, 0x0, 0xc0, 0x3, 0x1,
        ];
        let mut packet = Packet::from_slice(&DATA);
        let msg = HciEventMsg::from_packet(&mut packet).unwrap();

        let expected = HciEventMsg::LeConnectionComplete {
            status: HciStatus::Success,
            connection_handle: 0x0040,
            role: Role::Peripheral,
            peer_address_type: AddressType::Public,
            peer_address: [0x26, 0xe, 0xd6, 0xe8, 0xc2, 0x50],
            connection_interval: 48,
            peripheral_latency: 0,
            supervision_timeout: 960,
            central_clock_accuracy: ClockAccuracy::Ppm250,
        };
        assert_eq!(msg, expected);

        // Ensure it can be serialized back to the original bytes
        // assert_eq!(msg.to_bytes().unwrap(), DATA.to_vec());
    }

    #[test]
    fn test_pairing_request() {
        // 02 40 20 0b 00 07 00 06 00 01 04 00 2d 10 0e 0f
        const DATA: [u8; 16] = [
            0x02, 0x40, 0x20, 0x0b, 0x00, 0x07, 0x00, 0x06, 0x00, 0x01, 0x04, 0x00, 0x2d, 0x10,
            0x0e, 0x0f,
        ];
        let mut packet = Packet::from_slice(&DATA);
        let msg = packet.unpack::<H4Packet>();
        packet.dump_state();
        println!("{:?}", msg);
    }

    /*
    #[test]
    fn test_role() {
        assert_eq!(Role::Central.to_bytes().unwrap(), vec![0x00]);
        assert_eq!(Role::Peripheral.to_bytes().unwrap(), vec![0x01]);

        let ((rest, offset), v) = Role::from_bytes((&[0x00, 0xFF], 0)).unwrap();
        assert_eq!(v, Role::Central);
        assert_eq!(offset, 0);
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0], 0xFF);

        let ((rest, offset), v) = Role::from_bytes((&[0x01, 0xFF], 0)).unwrap();
        assert_eq!(v, Role::Peripheral);
        assert_eq!(offset, 0);
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0], 0xFF);

        // Error
        let err = Role::from_bytes((&[0x02, 0xFF], 0)).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Parse error: Could not match enum variant id = 2 on enum `Role`"
        );
    }

    #[test]
    fn test_address_type() {
        assert_eq!(AddressType::Public.to_bytes().unwrap(), vec![0x00]);
        assert_eq!(AddressType::Random.to_bytes().unwrap(), vec![0x01]);

        let ((rest, offset), v) = AddressType::from_bytes((&[0x00, 0xFF], 0)).unwrap();
        assert_eq!(v, AddressType::Public);
        assert_eq!(offset, 0);
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0], 0xFF);

        let ((rest, offset), v) = AddressType::from_bytes((&[0x01, 0xFF], 0)).unwrap();
        assert_eq!(v, AddressType::Random);
        assert_eq!(offset, 0);
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0], 0xFF);
    }

    #[test]
    fn test_hci_status() {
        assert_eq!(HciStatus::Success.to_bytes().unwrap(), vec![0x00]);
        assert_eq!(HciStatus::Failure(0x05).to_bytes().unwrap(), vec![0x05]);

        let ((rest, offset), v) = HciStatus::from_bytes((&[0x00, 0xFF], 0)).unwrap();
        assert_eq!(v, HciStatus::Success);
        assert_eq!(offset, 0);
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0], 0xFF);

        let ((rest, offset), v) = HciStatus::from_bytes((&[0x05, 0xFF], 0)).unwrap();
        assert_eq!(v, HciStatus::Failure(0x05));
        assert_eq!(offset, 0);
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0], 0xFF);
    }

    #[test]
    fn test_connection_handle() {
        assert_eq!(
            ConnectionHandle(0xBEEF).to_bytes().unwrap(),
            vec![0xEF, 0xBE]
        );

        let ((rest, offset), v) = ConnectionHandle::from_bytes((&[0xEF, 0xBE, 0xCA], 0)).unwrap();
        assert_eq!(v, ConnectionHandle(0xBEEF));
        assert_eq!(offset, 0);
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0], 0xCA);
    }
    */
}
