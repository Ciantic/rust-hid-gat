use crate::packer::FixedSizeUtf8;

/// id_type = u8
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum H4Packet {
    /// id = 0x01
    Command(HciCommand),
    /// id = 0x04
    Event(HciEvent),
    /// id = 0x02
    Acl(HciAcl),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HciAcl {
    /// bits = 12
    pub connection_handle: ConnectionHandle,

    /// bits = 2
    pub pb: PacketBoundaryFlag,

    /// bits = 2
    pub bc: BroadcastFlag,

    /// ACL Data length
    ///
    /// prepend_length = u16
    pub msg: L2CapMessage,
}

/// L2CAP Message
///
/// id_type = u16
/// prepend_length = u16
/// prepend_length_offset = -2
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum L2CapMessage {
    /// id = 0x0006
    Smp(SmpPdu),
    /// id = 0x0004
    Att(AttPdu),
    /// id = _
    Unknown(u16, Vec<u8>),
}

/// ATT Message
///
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/attribute-protocol--att-.html
///
/// id_type = u8
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttPdu {
    /// id = 0x02
    AttExchangeMtuRequest { mtu: u16 },

    /// id = 0x03
    AttExchangeMtuResponse { mtu: u16 },

    /// id = 0x04
    AttFindInformationRequest {
        starting_handle: u16,
        ending_handle: u16,
    },

    /// id = 0x05
    AttFindInformationResponse {
        format: u8,
        information: Vec<(u16, u16)>,
    },

    /// id = 0x06
    AttFindByTypeValueRequest {
        starting_handle: u16,
        ending_handle: u16,
        uuid: u16,
        value: Vec<u8>,
    },

    /// id = 0x07
    AttFindByTypeValueResponse {
        handles_information: Vec<(u16, u16)>,
    },

    /// id = 0x08
    AttReadByTypeRequest {
        starting_handle: u16,
        ending_handle: u16,

        /// 2 or 16 bytes
        uuid: Vec<u8>,
    },

    /// id = 0x09
    AttReadByTypeResponse {
        /// The Length parameter shall be set to the size of one attribute handle-value pair.
        pair_length: u8,

        // Following can't work, because Vec reads to the end of the buffer, falling back to Vec<u8> instead
        // values: Vec<(u16, Vec<u8>)>,
        /// (Handle, value pairs)
        values: Vec<u8>,
    },

    /// id = 0x0A
    AttReadRequest { handle: u16 },

    /// id = 0x0B
    AttReadResponse { value: Vec<u8> },

    /// id = 0x18
    AttExecuteWriteRequest { flags: u8 },

    /// id = 0x19
    AttExecuteWriteResponse,

    /// id = 0x1B
    AttHandleValueNotification { handle: u16, value: Vec<u8> },

    /// id = _
    AttUnknown(u8, Vec<u8>),
}

/// id_type = u8
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmpPdu {
    /// id = 0x01
    SmpPairingRequest {
        io_capability: IOCapability,
        oob_data_flag: OOBDataFlag,
        authentication_requirements: AuthenticationRequirements,
        max_encryption_key_size: u8,
        initiator_key_distribution: KeyDistributionFlags,
        responder_key_distribution: KeyDistributionFlags,
    },

    /// id = 0x02
    SmpPairingResponse {
        io_capability: IOCapability,
        oob_data_flag: OOBDataFlag,
        authentication_requirements: AuthenticationRequirements,
        max_encryption_key_size: u8,
        initiator_key_distribution: KeyDistributionFlags,
        responder_key_distribution: KeyDistributionFlags,
    },

    /// id = 0x03
    SmpPairingConfirmation { confirm_value: u128 },

    /// id = 0x04
    SmpPairingRandom { random_value: u128 },

    /// id = 0x05
    SmpPairingFailed(SmpPairingFailure),

    /// id = 0x06
    SmpEncryptionInformation { long_term_key: u128 },

    /// id = 0x07
    SmpCentralIdentification {
        encrypted_diversifier: u16,
        random_number: u64,
    },
}

/// length_after_id = u8
/// id_type = OpCode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HciCommand {
    /// id = OpCode(0x0006, 0x01)
    Disconnect {
        connection_handle: ConnectionHandle,
        reason: u8,
    },

    /// id = OpCode(0x0003, 0x03)
    Reset,

    /// id = OpCode(0x0001, 0x03)
    SetEventMask { event_mask: u64 },

    /// id = OpCode(0x0002, 0x04)
    ReadLocalSupportedCommands,

    /// id = OpCode(0x0009, 0x04)
    ReadBdAddr,

    /// id = OpCode(0x001a, 0x03)
    WriteScanEnable(ScanEnable),

    /// id = OpCode(0x0016, 0x03)
    WriteConnectionAcceptTimeout(u16),

    /// id = OpCode(0x0018, 0x03)
    WritePageTimeout(u16),

    /// id = OpCode(0x0013, 0x03)
    WriteLocalName(FixedSizeUtf8<248>),

    /// id = OpCode(0x0014, 0x03)
    ReadLocalName {
        status: HciStatus,
        name: FixedSizeUtf8<248>,
    },

    /// id = OpCode(0x0001, 0x08)
    LeSetEventMask { event_mask: u64 },

    /// id = OpCode(0x0002, 0x08)
    LeReadBufferSize,

    /// id = OpCode(0x0005, 0x08)
    LeSetRandomAddress(BdAddr),

    /// id = OpCode(0x0006, 0x08)
    LeSetAdvertisingParameters {
        advertising_interval_min: u16,
        advertising_interval_max: u16,
        advertising_type: u8,
        own_address_type: u8,
        peer_address_type: u8,
        peer_address: BdAddr,
        advertising_channel_map: u8,
        advertising_filter_policy: u8,
    },

    /// id = OpCode(0x0008, 0x08)
    LeSetAdvertisingData {
        advertising_data_length: u8,
        advertising_data: [u8; 31],
    },

    /// id = OpCode(0x0025, 0x08)
    LeReadLocalP256PublicKey,

    /// id = OpCode(0x000A, 0x08)
    LeSetAdvertisingEnable(bool),

    /// id = OpCode(0x0022, 0x08)
    LeSetDataLength {
        connection_handle: ConnectionHandle,
        tx_octets: u16,
        tx_time: u16,
    },

    /// id = OpCode(0x001A, 0x08)
    LeLongTermKeyRequestReply {
        connection_handle: ConnectionHandle,
        long_term_key: u128,
    },
}

/// HCI OpCode
#[derive(Debug, Clone, PartialEq, Eq, Copy, Default, Hash)]
pub struct OpCode(
    /// Command (OCF)
    /// bits = 10
    pub u16,
    /// Group (OGF)
    /// bits = 6
    pub u8,
);

/// id_type = u8
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScanEnable {
    /// id = 0x00
    NoScans,
    /// id = 0x01
    InquiryScanEnabled_PageScanDisabled,
    /// id = 0x02
    InquiryScanDisabled_PageScanEnabled,
    /// id = 0x03
    InquiryScanEnabled_PageScanEnabled,
}

/// id_type = u8
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeMeta {
    /// id = 0x01
    LeConnectionComplete {
        status: HciStatus,
        connection_handle: ConnectionHandle,
        role: Role,
        peer_address_type: AddressType,
        peer_address: BdAddr,
        connection_interval: u16,
        peripheral_latency: u16,
        supervision_timeout: u16,
        central_clock_accuracy: ClockAccuracy,
    },

    /// id = 0x02
    LeAdvertisingReport(Vec<u8>),

    /// id = 0x03
    LeConnectionUpdateComplete {
        status: HciStatus,
        connection_handle: ConnectionHandle,
        interval: u16,
        latency: u16,
        timeout: u16,
    },

    /// id = 0x04
    LeReadRemoteFeaturesPage0Complete {
        status: HciStatus,
        connection_handle: ConnectionHandle,
        le_features: u64,
    },

    /// id = 0x05
    LeLongTermKeyRequest {
        connection_handle: ConnectionHandle,
        random_number: u64,
        encrypted_diversifier: u16,
    },

    /// id = 0x07
    LeDataLengthChange {
        connection_handle: ConnectionHandle,
        max_tx_octets: u16,
        max_tx_time: u16,
        max_rx_octets: u16,
        max_rx_time: u16,
    },

    /// id = 0x08
    LeReadLocalP256PublicKeyComplete {
        status: HciStatus,
        public_key: [u8; 64],
    },
}

/// id_type = u8
/// length_after_id = u8
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HciEvent {
    /// id = 0x05
    DisconnectComplete {
        status: HciStatus,
        connection_handle: ConnectionHandle,
        reason: u8,
    },

    /// id = 0x08
    EncryptionChange {
        status: HciStatus,
        connection_handle: ConnectionHandle,
        encryption_enabled: bool,
    },

    /// id = 0x13
    NumberOfCompletedPackets {
        num_hci_command_packets: u8,
        connection_handle: ConnectionHandle,
        num_completed_packets: u16,
    },

    /// id = 0x3e
    LeMeta(LeMeta),

    /// id = 0x0E
    CommandComplete {
        /// The number of HCI Command packets which are allowed to be sent to
        /// the Controller from the Host.
        num_hci_command_packets: u8,
        command_opcode: OpCode,
        status: HciStatus,
        data: Vec<u8>,
    },
    /// id = 0x0F
    CommandStatus {
        status: HciStatus,
        num_hci_command_packets: u8,
        command_opcode: OpCode,
    },

    /// id = 0xFF
    VendorSpecific(Vec<u8>),
}

/// id_type = u8
#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
pub enum PacketBoundaryFlag {
    #[default]
    FirstNonFlushable = 0b00,
    Continuation = 0b01,
    FirstFlushable = 0b10,
    Deprecated = 0b11,
}

/// id_type = u8
#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
pub enum BroadcastFlag {
    #[default]
    PointToPoint = 0b00,
    BdEdrBroadcast = 0b01,
}

/// id_type = u8
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HciStatus {
    /// id = 0x00
    Success,
    /// id = _
    Failure(u8),
}

/// id_type = u8
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Role {
    Central = 0,
    Peripheral = 1,
}

/// id_type = u8
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AddressType {
    Public = 0,
    Random = 1,
}

/// id_type = u8
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BdAddr(pub [u8; 6]);

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

/// id_type = u8
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IOCapability {
    DisplayOnly = 0x00,
    DisplayYesNo = 0x01,
    KeyboardOnly = 0x02,
    NoInputNoOutput = 0x03,
    KeyboardDisplay = 0x04,
    // Reserved(u8),
}

/// id_type = u8
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OOBDataFlag {
    OobNotAvailable = 0x00,
    OobAvailable = 0x01,
    // Reserved(u8),
}

/// SMP Pairing failures
///
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/security-manager-specification.html#UUID-edc160cf-62e1-c774-f84c-da67aaf4aa50
///
/// id_type = u8
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
