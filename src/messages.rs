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

/// length_after_id = u8
/// id_type = OpCode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HciCommand {
    /// id = OpCode(0x0006, 0x01)
    Disconnect(CmdDisconnect),

    /// id = OpCode(0x0003, 0x03)
    Reset,

    /// id = OpCode(0x0001, 0x03)
    SetEventMask(u64),

    /// id = OpCode(0x0002, 0x04)
    ReadLocalSupportedCommands,

    /// id = OpCode(0x0009, 0x04)
    ReadBdAddr,

    /// id = OpCode(0x001a, 0x03)
    WriteScanEnable(CmdScanEnable),

    /// id = OpCode(0x0016, 0x03)
    WriteConnectionAcceptTimeout(u16),

    /// id = OpCode(0x0018, 0x03)
    WritePageTimeout(u16),

    /// id = OpCode(0x0013, 0x03)
    WriteLocalName(FixedSizeUtf8<248>),

    /// id = OpCode(0x0014, 0x03)
    ReadLocalName(CmdReadLocalName),

    /// id = OpCode(0x0001, 0x08)
    LeSetEventMask(u64),

    /// id = OpCode(0x0002, 0x08)
    LeReadBufferSize,

    /// id = OpCode(0x0005, 0x08)
    LeSetRandomAddress(BdAddr),

    /// id = OpCode(0x0006, 0x08)
    LeSetAdvertisingParameters(LeSetAdvertisingParameters),

    /// id = OpCode(0x0008, 0x08)
    LeSetAdvertisingData(LeSetAdvertisingData),

    /// id = OpCode(0x0025, 0x08)
    LeReadLocalP256PublicKey,

    /// id = OpCode(0x000A, 0x08)
    LeSetAdvertisingEnable(bool),

    /// id = OpCode(0x0022, 0x08)
    LeSetDataLength(LeSetDataLength),

    /// id = OpCode(0x001A, 0x08)
    LeLongTermKeyRequestReply(LeLongTermKeyRequestReply),
}

/// id_type = u8
/// length_after_id = u8
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HciEvent {
    /// id = 0x05
    DisconnectComplete(EvtDisconnectComplete),

    /// id = 0x08
    EncryptionChange(EvtEncryptionChange),

    /// id = 0x13
    NumberOfCompletedPackets(EvtNumberOfCompletedPackets),

    /// id = 0x3e
    LeMeta(LeMeta),

    /// id = 0x0E
    CommandComplete(EvtCommandComplete),

    /// id = 0x0F
    CommandStatus(EvtCommandStatus),

    /// id = 0xFF
    VendorSpecific(Vec<u8>),
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
    ExchangeMtuRequest(u16),

    /// id = 0x03
    ExchangeMtuResponse(u16),

    /// id = 0x04
    FindInformationRequest(AttFindInformationRequest),

    /// id = 0x05
    FindInformationResponse(AttFindInformationResponse),

    /// id = 0x06
    FindByTypeValueRequest(AttFindByTypeValueRequest),

    /// id = 0x07
    FindByTypeValueResponse(AttFindByTypeValueResponse),

    /// id = 0x08
    ReadByTypeRequest(AttReadByTypeRequest),

    /// id = 0x09
    ReadByTypeResponse(AttReadByTypeResponse),

    /// id = 0x0A
    ReadRequest(AttReadRequest),

    /// id = 0x0B
    ReadResponse(AttReadResponse),

    /// id = 0x18
    ExecuteWriteRequest(AttExecuteWriteRequest),

    /// id = 0x19
    ExecuteWriteResponse,

    /// id = 0x1B
    HandleValueNotification(AttHandleValueNotification),

    /// id = _
    Unknown(u8, Vec<u8>),
}

/// id_type = u8
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmpPdu {
    /// id = 0x01
    PairingRequest(SmpPairingReqRes),

    /// id = 0x02
    PairingResponse(SmpPairingReqRes),

    /// id = 0x03
    PairingConfirmation(SmpPairingConfirmation),

    /// id = 0x04
    PairingRandom(SmpPairingRandom),

    /// id = 0x05
    PairingFailed(SmpPairingFailure),

    /// id = 0x06
    EncryptionInformation(SmpEncryptionInformation),

    /// id = 0x07
    CentralIdentification(SmpCentralIdentification),
}

/// id_type = u8
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeMeta {
    /// id = 0x01
    LeConnectionComplete(LeConnectionComplete),

    /// id = 0x02
    LeAdvertisingReport(Vec<u8>),

    /// id = 0x03
    LeConnectionUpdateComplete(LeConnectionUpdateComplete),

    /// id = 0x04
    LeReadRemoteFeaturesPage0Complete(LeReadRemoteFeaturesPage0Complete),

    /// id = 0x05
    LeLongTermKeyRequest(LeLongTermKeyRequest),

    /// id = 0x07
    LeDataLengthChange(LeDataLengthChange),

    /// id = 0x08
    LeReadLocalP256PublicKeyComplete(LeReadLocalP256PublicKeyComplete),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttFindInformationRequest {
    pub starting_handle: u16,
    pub ending_handle: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttFindInformationResponse {
    pub format: u8,
    pub information: Vec<(u16, u16)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttFindByTypeValueRequest {
    pub starting_handle: u16,
    pub ending_handle: u16,
    pub uuid: u16,
    pub value: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttFindByTypeValueResponse {
    pub handles_information: Vec<(u16, u16)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttReadByTypeRequest {
    pub starting_handle: u16,
    pub ending_handle: u16,

    /// 2 or 16 bytes
    pub uuid: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttReadByTypeResponse {
    /// The Length parameter shall be set to the size of one attribute handle-value pair.
    pub pair_length: u8,

    // Following can't work, because Vec reads to the end of the buffer, falling back to Vec<u8> instead
    // values: Vec<(u16, Vec<u8>)>,
    /// (Handle, value pairs)
    pub values: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttReadRequest {
    pub handle: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttReadResponse {
    pub value: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttExecuteWriteRequest {
    pub flags: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttHandleValueNotification {
    pub handle: u16,
    pub value: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmpPairingReqRes {
    pub io_capability: IOCapability,
    pub oob_data_flag: OOBDataFlag,
    pub authentication_requirements: AuthenticationRequirements,
    pub max_encryption_key_size: u8,
    pub initiator_key_distribution: KeyDistributionFlags,
    pub responder_key_distribution: KeyDistributionFlags,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmpPairingConfirmation {
    pub confirm_value: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmpPairingRandom {
    pub random_value: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmpEncryptionInformation {
    pub long_term_key: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmpCentralIdentification {
    pub encrypted_diversifier: u16,
    pub random_number: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmdDisconnect {
    pub connection_handle: ConnectionHandle,
    pub reason: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmdReadLocalName {
    pub status: HciStatus,
    pub name: FixedSizeUtf8<248>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeSetAdvertisingParameters {
    pub advertising_interval_min: u16,
    pub advertising_interval_max: u16,
    pub advertising_type: u8,
    pub own_address_type: u8,
    pub peer_address_type: u8,
    pub peer_address: BdAddr,
    pub advertising_channel_map: u8,
    pub advertising_filter_policy: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeSetAdvertisingData {
    pub advertising_data_length: u8,
    pub advertising_data: [u8; 31],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeSetDataLength {
    pub connection_handle: ConnectionHandle,
    pub tx_octets: u16,
    pub tx_time: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeLongTermKeyRequestReply {
    pub connection_handle: ConnectionHandle,
    pub long_term_key: u128,
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
pub enum CmdScanEnable {
    /// id = 0x00
    NoScans,
    /// id = 0x01
    InquiryScanEnabled_PageScanDisabled,
    /// id = 0x02
    InquiryScanDisabled_PageScanEnabled,
    /// id = 0x03
    InquiryScanEnabled_PageScanEnabled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeConnectionComplete {
    pub status: HciStatus,
    pub connection_handle: ConnectionHandle,
    pub role: Role,
    pub peer_address_type: AddressType,
    pub peer_address: BdAddr,
    pub connection_interval: u16,
    pub peripheral_latency: u16,
    pub supervision_timeout: u16,
    pub central_clock_accuracy: ClockAccuracy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeConnectionUpdateComplete {
    pub status: HciStatus,
    pub connection_handle: ConnectionHandle,
    pub interval: u16,
    pub latency: u16,
    pub timeout: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeReadRemoteFeaturesPage0Complete {
    pub status: HciStatus,
    pub connection_handle: ConnectionHandle,
    pub le_features: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeLongTermKeyRequest {
    pub connection_handle: ConnectionHandle,
    pub random_number: u64,
    pub encrypted_diversifier: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeDataLengthChange {
    pub connection_handle: ConnectionHandle,
    pub max_tx_octets: u16,
    pub max_tx_time: u16,
    pub max_rx_octets: u16,
    pub max_rx_time: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeReadLocalP256PublicKeyComplete {
    pub status: HciStatus,
    pub public_key: [u8; 64],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvtDisconnectComplete {
    pub status: HciStatus,
    pub connection_handle: ConnectionHandle,
    pub reason: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvtEncryptionChange {
    pub status: HciStatus,
    pub connection_handle: ConnectionHandle,
    pub encryption_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvtNumberOfCompletedPackets {
    pub num_hci_command_packets: u8,
    pub connection_handle: ConnectionHandle,
    pub num_completed_packets: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvtCommandComplete {
    /// The number of HCI Command packets which are allowed to be sent to
    /// the Controller from the Host.
    pub num_hci_command_packets: u8,
    pub command_opcode: OpCode,
    pub status: HciStatus,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvtCommandStatus {
    pub status: HciStatus,
    pub num_hci_command_packets: u8,
    pub command_opcode: OpCode,
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
