use crate::packets::SmpPdu;

// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/security-manager-specification.html

pub struct ParseError(String);
pub struct SerializationError(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmpMsg {
    /// Pairing Request
    ///
    /// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/security-manager-specification.html#UUID-717b311a-0264-5837-3902-fc17dbe94572
    PairingRequest {
        io_capability: IOCapability,
        oob_data_flag: OOBDataFlag,
        authentication_requirements: AuthenticationRequirements,
        max_encryption_key_size: u8,
        initiator_key_distribution: KeyDistributionFlags,
        responder_key_distribution: KeyDistributionFlags,
    },
    PairingResponse {
        io_capability: IOCapability,
        oob_data_flag: OOBDataFlag,
        authentication_requirements: AuthenticationRequirements,
        max_encryption_key_size: u8,
        initiator_key_distribution: KeyDistributionFlags,
        responder_key_distribution: KeyDistributionFlags,
    },
    PairingConfirm(u128),
    PairingRandom(u128),
    PairingFailed(SmpPairingFailure),
    EncryptionInformation(u128),
    CentralIdentification {
        encrypted_diversifier: u8,
        random_value: u64,
    },
    Unknown(u8, Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct KeyDistributionFlags {
    pub enc_key: bool,
    pub id_key: bool,
    pub sign_key: bool,
    pub link_key: bool,
    pub reserved: (bool, bool, bool, bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AuthenticationRequirements {
    pub bonding: bool,
    pub mitm_protection: bool,
    pub secure_connections: bool,
    pub keypress_notification: bool,
    pub ct2: bool,
    pub reserved: (bool, bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IOCapability {
    DisplayOnly,
    DisplayYesNo,
    KeyboardOnly,
    NoInputNoOutput,
    KeyboardDisplay,
    Reserved(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OOBDataFlag {
    OobNotAvailable,
    OobAvailable,
    Reserved(u8),
}

/// SMP Pairing failures
///
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/security-manager-specification.html#UUID-edc160cf-62e1-c774-f84c-da67aaf4aa50
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmpPairingFailure {
    PasskeyEntryFailed,
    OobNotAvailable,
    AuthenticationRequirements,
    ConfirmValueFailed,
    PairingNotSupported,
    EncryptionKeySize,
    CommandNotSupported,
    UnspecifiedReason,
    RepeatedAttempts,
    InvalidParameters,
    DhKeyCheckFailed,
    NumericComparisonFailed,
    BrEdrPairingInProgress,
    CrossTransportKeyDerivationGenerationNotAllowed,
    KeyRejected,
    Busy,
    Reserved(u8),
}

// -- Serialization and Deserialization ---------------------------------------

impl SmpMsg {
    pub fn get_opcode(&self) -> u8 {
        match self {
            SmpMsg::PairingRequest { .. } => 0x01,
            SmpMsg::PairingResponse { .. } => 0x02,
            SmpMsg::PairingConfirm(_) => 0x03,
            SmpMsg::PairingRandom(_) => 0x04,
            SmpMsg::PairingFailed(_) => 0x05,
            SmpMsg::EncryptionInformation(_) => 0x06,
            SmpMsg::CentralIdentification { .. } => 0x07,
            SmpMsg::Unknown(code, _) => *code,
        }
    }

    pub fn to_smp_pdu(&self) -> Result<SmpPdu, SerializationError> {
        Ok(match self {
            SmpMsg::PairingRequest {
                io_capability,
                oob_data_flag,
                authentication_requirements,
                max_encryption_key_size,
                initiator_key_distribution,
                responder_key_distribution,
            } => SmpPdu {
                code: 0x01,
                data: vec![
                    io_capability.to_code(),
                    oob_data_flag.to_code(),
                    authentication_requirements.to_byte(),
                    *max_encryption_key_size,
                    initiator_key_distribution.to_byte(),
                    responder_key_distribution.to_byte(),
                ],
            },
            SmpMsg::PairingResponse {
                io_capability,
                oob_data_flag,
                authentication_requirements,
                max_encryption_key_size,
                initiator_key_distribution,
                responder_key_distribution,
            } => SmpPdu {
                code: 0x02,
                data: vec![
                    io_capability.to_code(),
                    oob_data_flag.to_code(),
                    authentication_requirements.to_byte(),
                    *max_encryption_key_size,
                    initiator_key_distribution.to_byte(),
                    responder_key_distribution.to_byte(),
                ],
            },
            SmpMsg::PairingConfirm(confirm) => SmpPdu {
                code: 0x03,
                data: confirm.to_le_bytes().to_vec(),
            },
            SmpMsg::PairingRandom(random) => SmpPdu {
                code: 0x04,
                data: random.to_le_bytes().to_vec(),
            },
            SmpMsg::PairingFailed(reason) => SmpPdu {
                code: 0x05,
                data: vec![reason.to_code()],
            },
            SmpMsg::EncryptionInformation(info) => SmpPdu {
                code: 0x06,
                data: info.to_le_bytes().to_vec(),
            },
            SmpMsg::CentralIdentification {
                encrypted_diversifier,
                random_value,
            } => SmpPdu {
                code: 0x07,
                data: [
                    vec![*encrypted_diversifier],
                    random_value.to_le_bytes().to_vec(),
                ]
                .concat(),
            },
            SmpMsg::Unknown(code, data) => SmpPdu {
                code: *code,
                data: data.clone(),
            },
        })
    }

    pub fn from_smp_pdu(smp: SmpPdu) -> Result<Self, ParseError> {
        Ok(match smp.code {
            0x01 => SmpMsg::PairingRequest {
                io_capability: IOCapability::from_code(smp.data[0]),
                oob_data_flag: OOBDataFlag::from_code(smp.data[1]),
                authentication_requirements: AuthenticationRequirements::from_byte(smp.data[2]),
                max_encryption_key_size: smp.data[3],
                initiator_key_distribution: KeyDistributionFlags::from_byte(smp.data[4]),
                responder_key_distribution: KeyDistributionFlags::from_byte(smp.data[5]),
            },
            0x02 => SmpMsg::PairingResponse {
                io_capability: IOCapability::from_code(smp.data[0]),
                oob_data_flag: OOBDataFlag::from_code(smp.data[1]),
                authentication_requirements: AuthenticationRequirements::from_byte(smp.data[2]),
                max_encryption_key_size: smp.data[3],
                initiator_key_distribution: KeyDistributionFlags::from_byte(smp.data[4]),
                responder_key_distribution: KeyDistributionFlags::from_byte(smp.data[5]),
            },
            0x03 => SmpMsg::PairingConfirm(u128::from_le_bytes(smp.data.try_into().unwrap())),
            0x04 => SmpMsg::PairingRandom(u128::from_le_bytes(smp.data.try_into().unwrap())),
            0x05 => SmpMsg::PairingFailed(SmpPairingFailure::from_code(smp.data[0])),
            0x06 => {
                SmpMsg::EncryptionInformation(u128::from_le_bytes(smp.data.try_into().unwrap()))
            }
            0x07 => SmpMsg::CentralIdentification {
                encrypted_diversifier: smp.data[0],
                random_value: u64::from_le_bytes(smp.data[1..9].try_into().unwrap()),
            },
            _ => SmpMsg::Unknown(smp.code, smp.data),
        })
    }
}

impl SmpPairingFailure {
    pub fn from_code(code: u8) -> Self {
        match code {
            0x01 => SmpPairingFailure::PasskeyEntryFailed,
            0x02 => SmpPairingFailure::OobNotAvailable,
            0x03 => SmpPairingFailure::AuthenticationRequirements,
            0x04 => SmpPairingFailure::ConfirmValueFailed,
            0x05 => SmpPairingFailure::PairingNotSupported,
            0x06 => SmpPairingFailure::EncryptionKeySize,
            0x07 => SmpPairingFailure::CommandNotSupported,
            0x08 => SmpPairingFailure::UnspecifiedReason,
            0x09 => SmpPairingFailure::RepeatedAttempts,
            0x0A => SmpPairingFailure::InvalidParameters,
            0x0B => SmpPairingFailure::DhKeyCheckFailed,
            0x0C => SmpPairingFailure::NumericComparisonFailed,
            0x0D => SmpPairingFailure::BrEdrPairingInProgress,
            0x0E => SmpPairingFailure::CrossTransportKeyDerivationGenerationNotAllowed,
            0x0F => SmpPairingFailure::KeyRejected,
            0x10 => SmpPairingFailure::Busy,
            _ => SmpPairingFailure::Reserved(code),
        }
    }
    pub fn to_code(&self) -> u8 {
        match self {
            SmpPairingFailure::PasskeyEntryFailed => 0x01,
            SmpPairingFailure::OobNotAvailable => 0x02,
            SmpPairingFailure::AuthenticationRequirements => 0x03,
            SmpPairingFailure::ConfirmValueFailed => 0x04,
            SmpPairingFailure::PairingNotSupported => 0x05,
            SmpPairingFailure::EncryptionKeySize => 0x06,
            SmpPairingFailure::CommandNotSupported => 0x07,
            SmpPairingFailure::UnspecifiedReason => 0x08,
            SmpPairingFailure::RepeatedAttempts => 0x09,
            SmpPairingFailure::InvalidParameters => 0x0A,
            SmpPairingFailure::DhKeyCheckFailed => 0x0B,
            SmpPairingFailure::NumericComparisonFailed => 0x0C,
            SmpPairingFailure::BrEdrPairingInProgress => 0x0D,
            SmpPairingFailure::CrossTransportKeyDerivationGenerationNotAllowed => 0x0E,
            SmpPairingFailure::KeyRejected => 0x0F,
            SmpPairingFailure::Busy => 0x10,
            SmpPairingFailure::Reserved(code) => *code,
        }
    }
}

impl IOCapability {
    pub fn to_code(&self) -> u8 {
        match self {
            IOCapability::DisplayOnly => 0x00,
            IOCapability::DisplayYesNo => 0x01,
            IOCapability::KeyboardOnly => 0x02,
            IOCapability::NoInputNoOutput => 0x03,
            IOCapability::KeyboardDisplay => 0x04,
            IOCapability::Reserved(code) => *code,
        }
    }

    pub fn from_code(code: u8) -> Self {
        match code {
            0x00 => IOCapability::DisplayOnly,
            0x01 => IOCapability::DisplayYesNo,
            0x02 => IOCapability::KeyboardOnly,
            0x03 => IOCapability::NoInputNoOutput,
            0x04 => IOCapability::KeyboardDisplay,
            _ => IOCapability::Reserved(code),
        }
    }
}

impl OOBDataFlag {
    pub fn to_code(&self) -> u8 {
        match self {
            OOBDataFlag::OobNotAvailable => 0x00,
            OOBDataFlag::OobAvailable => 0x01,
            OOBDataFlag::Reserved(code) => *code,
        }
    }

    pub fn from_code(code: u8) -> Self {
        match code {
            0x00 => OOBDataFlag::OobNotAvailable,
            0x01 => OOBDataFlag::OobAvailable,
            _ => OOBDataFlag::Reserved(code),
        }
    }
}

impl AuthenticationRequirements {
    pub fn from_byte(byte: u8) -> Self {
        AuthenticationRequirements {
            bonding: (byte & 0x01) != 0,
            mitm_protection: (byte & 0x04) != 0,
            secure_connections: (byte & 0x08) != 0,
            keypress_notification: (byte & 0x10) != 0,
            ct2: (byte & 0x20) != 0,
            reserved: ((byte & 0x40) != 0, (byte & 0x80) != 0),
        }
    }
    pub fn to_byte(&self) -> u8 {
        let mut byte = 0x00;
        if self.bonding {
            byte |= 0x01;
        }
        if self.mitm_protection {
            byte |= 0x04;
        }
        if self.secure_connections {
            byte |= 0x08;
        }
        if self.keypress_notification {
            byte |= 0x10;
        }
        if self.ct2 {
            byte |= 0x20;
        }
        if self.reserved.0 {
            byte |= 0x40;
        }
        if self.reserved.1 {
            byte |= 0x80;
        }
        byte
    }
}

impl KeyDistributionFlags {
    pub fn from_byte(byte: u8) -> Self {
        KeyDistributionFlags {
            enc_key: (byte & 0x01) != 0,
            id_key: (byte & 0x02) != 0,
            sign_key: (byte & 0x04) != 0,
            link_key: (byte & 0x08) != 0,
            reserved: (
                (byte & 0x10) != 0,
                (byte & 0x20) != 0,
                (byte & 0x40) != 0,
                (byte & 0x80) != 0,
            ),
        }
    }
    pub fn to_byte(&self) -> u8 {
        let mut byte = 0x00;
        if self.enc_key {
            byte |= 0x01;
        }
        if self.id_key {
            byte |= 0x02;
        }
        if self.sign_key {
            byte |= 0x04;
        }
        if self.link_key {
            byte |= 0x08;
        }
        if self.reserved.0 {
            byte |= 0x10;
        }
        if self.reserved.1 {
            byte |= 0x20;
        }
        if self.reserved.2 {
            byte |= 0x40;
        }
        if self.reserved.3 {
            byte |= 0x80;
        }
        byte
    }
}
