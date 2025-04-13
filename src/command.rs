#[derive(Debug, Clone, PartialEq, Eq)]
enum HciCommandMsg {
    SetEventMask([u8; 2]),
    WriteScanEnable(u8),
    WriteConnectionAcceptTimeout([u8; 2]),
    WritePageTimeout([u8; 2]),
    ReadLocalSupportedCommands,
    ReadBdAddr,
    ReadBufferSize,
    WriteLocalName(String),
    // ReadLocalName,
    LeSetEventMask([u8; 2]),
    LeSetRandomAddress([u8; 6]),
    LeSetAdvertisingParameters([u8; 16]),
    LeSetAdvertisingData([u8; 16]),
    LeSetAdvertisingEnable(bool),
    LeReadLocalP256PublicKey,

    // Code, Parameter Total Length, Rest of the data
    Unknown(u8, u8, Vec<u8>),
}
