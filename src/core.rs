use crate::packer::FixedSizeUtf8;

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

        /// ACL Data length
        ///
        /// prepend_length = u16
        msg: L2CapMessage,
    },
}

/// L2CAP Message
///
/// prepend_length = u16
/// prepend_length_offset = -2
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum L2CapMessage {
    /// id = &[0x06, 0x00]
    Smp(SmpPdu),
    /// id = &[0x04, 0x00]
    Att,
    /// id = _
    Unknown(u16, Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmpPdu {
    /// id = &[0x03]
    SmpPairingConfirmation { confirm_value: u128 },
    /// id = &[0x04]
    SmpPairingRandom { random_value: u128 },
    /// id = &[0x01]
    SmpPairingRequest {
        io_capability: IOCapability,
        oob_data_flag: OOBDataFlag,
        authentication_requirements: AuthenticationRequirements,
        max_encryption_key_size: u8,
        initiator_key_distribution: KeyDistributionFlags,
        responder_key_distribution: KeyDistributionFlags,
    },
    /// id = &[0x02]
    SmpPairingResponse {
        io_capability: IOCapability,
        oob_data_flag: OOBDataFlag,
        authentication_requirements: AuthenticationRequirements,
        max_encryption_key_size: u8,
        initiator_key_distribution: KeyDistributionFlags,
        responder_key_distribution: KeyDistributionFlags,
    },
}

/// length_after_id = u8
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HciCommand {
    /// id = &[0x03, 0x0C]
    Reset,

    /// id = &[0x01, 0x0c]
    SetEventMask { event_mask: u64 },

    /// id = &[0x01, 0x20]
    LeSetEventMask { event_mask: u64 },

    /// id = &[0x1a, 0x0C]
    WriteScanEnable(ScanEnable),

    /// id = &[0x16, 0x0C]
    WriteConnectionAcceptTimeout(u16),

    /// id = &[0x18, 0x0C]
    WritePageTimeout(u16),

    /// id = &[0x02, 0x10]
    ReadLocalSupportedCommands,

    /// id = &[0x09, 0x10]
    ReadBdAddr,

    /// id = &[0x02, 0x20]
    LeReadBufferSize,

    /// id = &[0x13, 0x0c]
    WriteLocalName(FixedSizeUtf8<248>),

    /// id = &[0x14, 0x0c]
    ReadLocalName {
        status: HciStatus,
        name: FixedSizeUtf8<248>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScanEnable {
    /// id = &[0x00]
    NoScans,
    /// id = &[0x01]
    InquiryScanEnabled_PageScanDisabled,
    /// id = &[0x02]
    InquiryScanDisabled_PageScanEnabled,
    /// id = &[0x03]
    InquiryScanEnabled_PageScanEnabled,
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
    /// bits = 1
    pub enc_key: bool,

    /// bits = 1
    pub id_key: bool,

    /// bits = 1
    pub sign_key: bool,

    /// bits = 1
    pub link_key: bool,

    /// bits = 4
    pub _reserved: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AuthenticationRequirements {
    /// bits = 2
    pub bonding: bool,
    /// bits = 1
    pub mitm_protection: bool,
    /// bits = 1
    pub secure_connections: bool,
    /// bits = 1
    pub keypress_notification: bool,
    /// bits = 1
    pub ct2: bool,

    /// bits = 2
    pub _reserved: u8,
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
    fn deserialize_write_name() {
        let mut packet = Packet::from_slice(&[
            0x13, 0xc, 0xf8, 0x4d, 0x79, 0x20, 0x50, 0x69, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ]);
        let msg = packet.unpack::<HciCommand>().unwrap();
        if let HciCommand::WriteLocalName(name) = msg {
            assert_eq!(name.get(), "My Pi");
        } else {
            panic!("Expected WriteLocalName");
        }
    }

    #[test]
    fn serialize_write_name() {
        let mut packet = Packet::new();
        packet
            .pack(&HciCommand::WriteLocalName(FixedSizeUtf8::<248>::new(
                "My Pi",
            )))
            .unwrap();
        assert_eq!(
            &[
                0x13, 0xc, 0xf8, 0x4d, 0x79, 0x20, 0x50, 0x69, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            packet.get_bytes()
        );
    }

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
    fn test_hci_command_reset() {
        let mut packet = Packet::from_slice(&[0x03, 0x0C, 0x00]);
        let handle = HciCommand::from_packet(&mut packet).unwrap();
        assert_eq!(handle, HciCommand::Reset);

        let mut packet = Packet::new();
        packet.pack(&HciCommand::Reset).unwrap();
        assert_eq!(packet.get_bytes(), &[0x03, 0x0C, 0x00]);
    }

    #[test]
    fn test_hci_command_set_event_mask() {
        let mut packet =
            Packet::from_slice(&[0x1, 0xc, 0x8, 0xff, 0xff, 0xfb, 0xff, 0x7, 0xf8, 0xbf, 0x3d]);
        let handle = HciCommand::from_packet(&mut packet).unwrap();
        assert_eq!(
            handle,
            HciCommand::SetEventMask {
                event_mask: 4449547670108504063
            }
        );

        let mut packet = Packet::new();
        packet
            .pack(&HciCommand::SetEventMask {
                event_mask: 4449547670108504063,
            })
            .unwrap();
        assert_eq!(
            packet.get_bytes(),
            &[
                0x01, 0x0c, 0x08, 0xff, 0xff, 0xfb, 0xff, 0x7, 0xf8, 0xbf, 0x3d
            ]
        );
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
    fn test_auth_req() {
        let m = AuthenticationRequirements {
            bonding: true,
            mitm_protection: true,
            secure_connections: true,
            keypress_notification: false,
            ct2: true,
            _reserved: 0,
        };

        let mut packet = Packet::new();
        packet.pack(&m).unwrap();
        assert_eq!(packet.get_bytes(), vec![0x2d]);

        let mut packet = Packet::from_slice(&[0x2d]);
        let res_msg = packet.unpack::<AuthenticationRequirements>();
        assert_eq!(res_msg, Ok(m));
    }

    #[test]
    fn test_keydistribution_flags() {
        let v = KeyDistributionFlags {
            enc_key: true,
            id_key: false,
            sign_key: true,
            link_key: false,
            _reserved: 0,
        };

        let mut packet = Packet::new();
        packet.pack(&v).unwrap();
        assert_eq!(packet.get_bytes(), vec![0b0000_0101]);
    }

    #[test]
    fn test_pairing_request() {
        // 02 40 20 0b 00 07 00 06 00 01 04 00 2d 10 0e 0f
        const DATA: [u8; 16] = [
            0x02, 0x40, 0x20, 0x0b, 0x00, 0x07, 0x00, 0x06, 0x00, 0x01, 0x04, 0x00, 0x2d, 0x10,
            0x0e, 0x0f,
        ];
        let mut packet = Packet::from_slice(&DATA);
        let res_msg = packet.unpack::<H4Packet>();
        let msg = H4Packet::HciAcl {
            connection_handle: ConnectionHandle(64),
            pb: PacketBoundaryFlag::FirstFlushable,
            bc: BroadcastFlag::PointToPoint,
            msg: L2CapMessage::Smp(SmpPdu::SmpPairingRequest {
                io_capability: IOCapability::KeyboardDisplay,
                oob_data_flag: OOBDataFlag::OobNotAvailable,
                authentication_requirements: AuthenticationRequirements {
                    bonding: true,
                    mitm_protection: true,
                    secure_connections: true,
                    keypress_notification: false,
                    ct2: true,
                    _reserved: 0,
                },
                max_encryption_key_size: 16,
                initiator_key_distribution: KeyDistributionFlags {
                    enc_key: false,
                    id_key: true,
                    sign_key: true,
                    link_key: true,
                    _reserved: 0,
                },
                responder_key_distribution: KeyDistributionFlags {
                    enc_key: true,
                    id_key: true,
                    sign_key: true,
                    link_key: true,
                    _reserved: 0,
                },
            }),
        };
        assert_eq!(res_msg.as_ref(), Ok(&msg));

        // Serializes back to the original bytes
        let mut packer2 = Packet::new();
        packer2.pack(&msg).unwrap();
        let serialized_bytes = packer2.get_bytes();
        assert_eq!(DATA.to_vec(), serialized_bytes);
    }

    #[test]
    fn test_pairing_request_2() {
        const DATA: [u8; 15] = [
            0x40, 0x20, 0x0b, 0x00, 0x07, 0x00, 0x06, 0x00, 0x01, 0x04, 0x00, 0x2d, 0x10, 0x0e,
            0x0f,
        ];
        let mut packet = Packet::from_slice(&DATA);
        let h: ConnectionHandle = packet.set_bits(12).unpack().unwrap();
        println!("{:x?}", h.0);

        let v = packet.set_bits(2).next_if_eq(&[0b10]);
        println!("{:?}", v);

        // let p: u8 = packet.set_bits(2).unpack().unwrap();
        // Print binary
        // println!("{:?}", p);
        // println!("{:08b}", p);
        // packet.unpack()
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
