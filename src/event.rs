#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HciEventMsg {
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
    CommandComplete {
        num_hci_command_packets: u8,
        command_opcode: u16,
        return_parameters: Vec<u8>,
    },
    CommandStatus {
        status: HciStatus,
        num_hci_command_packets: u8,
        command_opcode: u16,
    },
    // Other messages...
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HciStatus {
    Success,
    Error(u8),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Role {
    Central = 0x00,
    Peripheral = 0x01,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum AddressType {
    Public = 0x00,
    Random = 0x01,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockAccuracy {
    Ppm500 = 0x00,
    Ppm250 = 0x01,
    Ppm150 = 0x02,
    Ppm100 = 0x03,
    Ppm75 = 0x04,
    Ppm50 = 0x05,
    Ppm30 = 0x06,
    Ppm20 = 0x07,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerializationError {
    InternalError(&'static str),
    DataTooLarge(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InsufficientData { expected: usize, actual: usize },
    InvalidEventCode(u8),
    InvalidLength { expected: u8, actual: u8 },
    InvalidSubeventCode(u8),
    InvalidValue { field: &'static str, value: u8 },
    SliceToArrayError, // Store the original error
}

// --- Constants ---
const HCI_EVENT_CODE_LE_META: u8 = 0x3E;
const HCI_EVENT_CODE_COMMAND_COMPLETE: u8 = 0x0E;
const HCI_EVENT_CODE_COMMAND_STATUS: u8 = 0x0F; // New

const HCI_SUBEVENT_CODE_LE_CONNECTION_COMPLETE: u8 = 0x01;

// --- Lengths ---
const LE_CONN_COMPLETE_PARAM_LEN: u8 = 19;
const LE_CONN_COMPLETE_TOTAL_LEN: usize = 2 + LE_CONN_COMPLETE_PARAM_LEN as usize;

const COMMAND_COMPLETE_MIN_PARAM_LEN: u8 = 3; // NumPkts(1) + OpCode(2)

const COMMAND_STATUS_PARAM_LEN: u8 = 4; // Status(1) + NumPkts(1) + Opcode(2)
const COMMAND_STATUS_TOTAL_LEN: usize = 2 + COMMAND_STATUS_PARAM_LEN as usize; // Header(2) + Params(4)

// --- HciStatus ---
impl From<u8> for HciStatus {
    fn from(val: u8) -> Self {
        match val {
            0x00 => HciStatus::Success,
            err => HciStatus::Error(err),
        }
    }
}

impl HciStatus {
    pub fn into_u8(self) -> u8 {
        match self {
            HciStatus::Success => 0x00,
            HciStatus::Error(err) => err,
        }
    }
}

// --- Role ---
impl TryFrom<u8> for Role {
    type Error = ParseError;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0x00 => Ok(Role::Central),
            0x01 => Ok(Role::Peripheral),
            _ => Err(ParseError::InvalidValue {
                field: "Role",
                value: val,
            }),
        }
    }
}

// --- AddressType ---
impl TryFrom<u8> for AddressType {
    type Error = ParseError;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0x00 => Ok(AddressType::Public),
            0x01 => Ok(AddressType::Random),
            _ => Err(ParseError::InvalidValue {
                field: "AddressType",
                value: val,
            }),
        }
    }
}

// --- ClockAccuracy ---
impl TryFrom<u8> for ClockAccuracy {
    type Error = ParseError;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0x00 => Ok(ClockAccuracy::Ppm500),
            0x01 => Ok(ClockAccuracy::Ppm250),
            0x02 => Ok(ClockAccuracy::Ppm150),
            0x03 => Ok(ClockAccuracy::Ppm100),
            0x04 => Ok(ClockAccuracy::Ppm75),
            0x05 => Ok(ClockAccuracy::Ppm50),
            0x06 => Ok(ClockAccuracy::Ppm30),
            0x07 => Ok(ClockAccuracy::Ppm20),
            _ => Err(ParseError::InvalidValue {
                field: "ClockAccuracy",
                value: val,
            }),
        }
    }
}

// --- Error Conversion ---
impl From<std::array::TryFromSliceError> for ParseError {
    fn from(e: std::array::TryFromSliceError) -> Self {
        ParseError::SliceToArrayError
    }
}

// --- Implementation ---

impl HciEventMsg {
    pub fn to_bytes(&self) -> Result<Vec<u8>, SerializationError> {
        match self {
            HciEventMsg::LeConnectionComplete {
                status,
                connection_handle,
                role,
                peer_address_type,
                peer_address,
                connection_interval,
                peripheral_latency,
                supervision_timeout,
                central_clock_accuracy,
            } => {
                let mut bytes = Vec::with_capacity(LE_CONN_COMPLETE_TOTAL_LEN);
                bytes.push(HCI_EVENT_CODE_LE_META);
                bytes.push(LE_CONN_COMPLETE_PARAM_LEN);
                bytes.push(HCI_SUBEVENT_CODE_LE_CONNECTION_COMPLETE);
                bytes.push(status.into_u8());
                bytes.extend_from_slice(&connection_handle.to_le_bytes());
                bytes.push(*role as u8);
                bytes.push(*peer_address_type as u8);
                bytes.extend_from_slice(peer_address);
                bytes.extend_from_slice(&connection_interval.to_le_bytes());
                bytes.extend_from_slice(&peripheral_latency.to_le_bytes());
                bytes.extend_from_slice(&supervision_timeout.to_le_bytes());
                bytes.push(*central_clock_accuracy as u8);
                Ok(bytes)
            }
            HciEventMsg::CommandComplete {
                num_hci_command_packets,
                command_opcode,
                return_parameters,
            } => {
                let return_params_len = return_parameters.len();
                let param_len_usize = 1usize + 2usize + return_params_len;
                let param_len = u8::try_from(param_len_usize).map_err(|_| {
                    SerializationError::DataTooLarge(
                        "CommandComplete return parameters too large for HCI length field",
                    )
                })?;

                let total_len = 2 + param_len_usize;
                let mut bytes = Vec::with_capacity(total_len);

                bytes.push(HCI_EVENT_CODE_COMMAND_COMPLETE);
                bytes.push(param_len);
                bytes.push(*num_hci_command_packets);
                bytes.extend_from_slice(&command_opcode.to_le_bytes());
                bytes.extend_from_slice(return_parameters);

                Ok(bytes)
            }
            HciEventMsg::CommandStatus {
                status,
                num_hci_command_packets,
                command_opcode,
            } => {
                let mut bytes = Vec::with_capacity(COMMAND_STATUS_TOTAL_LEN);
                bytes.push(HCI_EVENT_CODE_COMMAND_STATUS);
                bytes.push(COMMAND_STATUS_PARAM_LEN);
                bytes.push(status.into_u8());
                bytes.push(*num_hci_command_packets);
                bytes.extend_from_slice(&command_opcode.to_le_bytes());
                Ok(bytes)
            } // Add other event types here...
              // _ => Err(SerializationError::InternalError("Serialization not implemented for this event type")),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
        // 1. Check minimum length for *any* HCI event header
        if bytes.len() < 2 {
            return Err(ParseError::InsufficientData {
                expected: 2,
                actual: bytes.len(),
            });
        }

        // 2. Read header fields
        let event_code = bytes[0];
        let param_len = bytes[1];
        let expected_total_len = 2 + param_len as usize;

        // 3. Check overall length based on header
        if bytes.len() < expected_total_len {
            return Err(ParseError::InsufficientData {
                expected: expected_total_len,
                actual: bytes.len(),
            });
        }

        // 4. Dispatch based on Event Code
        match event_code {
            HCI_EVENT_CODE_LE_META => {
                // --- LE Meta Event Parsing (as before) ---
                if param_len < 1 {
                    return Err(ParseError::InvalidLength {
                        expected: 1, // At least subevent code
                        actual: param_len,
                    });
                }
                let subevent_code = bytes[2];

                match subevent_code {
                    HCI_SUBEVENT_CODE_LE_CONNECTION_COMPLETE => {
                        if param_len != LE_CONN_COMPLETE_PARAM_LEN {
                            return Err(ParseError::InvalidLength {
                                expected: LE_CONN_COMPLETE_PARAM_LEN,
                                actual: param_len,
                            });
                        }
                        // Redundant total length check (already done above), but safe
                        if bytes.len() < LE_CONN_COMPLETE_TOTAL_LEN {
                            return Err(ParseError::InsufficientData {
                                expected: LE_CONN_COMPLETE_TOTAL_LEN,
                                actual: bytes.len(),
                            });
                        }

                        // Parse parameters (offset 3: after Event, Len, Subevent)
                        let status = HciStatus::from(bytes[3]);
                        let connection_handle = u16::from_le_bytes(bytes[4..6].try_into()?);
                        let role = Role::try_from(bytes[6])?;
                        let peer_address_type = AddressType::try_from(bytes[7])?;
                        let peer_address: [u8; 6] = bytes[8..14].try_into()?;
                        let connection_interval = u16::from_le_bytes(bytes[14..16].try_into()?);
                        let peripheral_latency = u16::from_le_bytes(bytes[16..18].try_into()?);
                        let supervision_timeout = u16::from_le_bytes(bytes[18..20].try_into()?);
                        let central_clock_accuracy = ClockAccuracy::try_from(bytes[20])?;

                        Ok(HciEventMsg::LeConnectionComplete {
                            status,
                            connection_handle,
                            role,
                            peer_address_type,
                            peer_address,
                            connection_interval,
                            peripheral_latency,
                            supervision_timeout,
                            central_clock_accuracy,
                        })
                    }
                    // Add other LE subevent codes here...
                    _ => Err(ParseError::InvalidSubeventCode(subevent_code)),
                }
            } // End HCI_EVENT_CODE_LE_META

            HCI_EVENT_CODE_COMMAND_COMPLETE => {
                // Check minimum length for Command Complete parameters
                if param_len < COMMAND_COMPLETE_MIN_PARAM_LEN {
                    return Err(ParseError::InvalidLength {
                        expected: COMMAND_COMPLETE_MIN_PARAM_LEN, // Or perhaps just check >= ? Spec implies exact. Let's require >=
                        actual: param_len,
                    });
                }
                // Parse parameters (offset 2: after Event, Len)
                let num_hci_command_packets = bytes[2];
                let command_opcode = u16::from_le_bytes(bytes[3..5].try_into()?);
                let return_parameters = bytes[5..expected_total_len].to_vec(); // Use expected_total_len derived from param_len

                Ok(HciEventMsg::CommandComplete {
                    num_hci_command_packets,
                    command_opcode,
                    return_parameters,
                })
            } // End HCI_EVENT_CODE_COMMAND_COMPLETE

            // --- NEW PARSING LOGIC ---
            HCI_EVENT_CODE_COMMAND_STATUS => {
                // Check specific length for Command Status
                if param_len != COMMAND_STATUS_PARAM_LEN {
                    return Err(ParseError::InvalidLength {
                        expected: COMMAND_STATUS_PARAM_LEN,
                        actual: param_len,
                    });
                }
                // Redundant total length check (already done above), but safe
                if bytes.len() < COMMAND_STATUS_TOTAL_LEN {
                    return Err(ParseError::InsufficientData {
                        expected: COMMAND_STATUS_TOTAL_LEN,
                        actual: bytes.len(),
                    });
                }

                // Parse parameters (offset 2: after Event, Len)
                let status = HciStatus::from(bytes[2]);
                let num_hci_command_packets = bytes[3];
                let command_opcode = u16::from_le_bytes(bytes[4..6].try_into()?);

                Ok(HciEventMsg::CommandStatus {
                    status,
                    num_hci_command_packets,
                    command_opcode,
                })
            } // End HCI_EVENT_CODE_COMMAND_STATUS

            // Add other top-level event codes here...
            _ => Err(ParseError::InvalidEventCode(event_code)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_byte_array_complete_event() {
        const LE_COMPLETE_EVENT_EX1: [u8; 6] = [0xe, 0x4, 0x1, 0x3, 0xc, 0x0];
        let value = HciEventMsg::from_bytes(&LE_COMPLETE_EVENT_EX1).unwrap();
        let expected = HciEventMsg::CommandComplete {
            num_hci_command_packets: 0x1,
            command_opcode: 0x0c03,
            return_parameters: vec![0x0],
        };
        assert_eq!(value, expected);

        // Serialize back to bytes and check if it matches the original
        let serialized = value.to_bytes().unwrap();
        assert_eq!(&serialized, &LE_COMPLETE_EVENT_EX1);
    }

    #[test]
    fn test_known_byte_array_le_connection_complete() {
        const LE_CONNECTION_COMPLETE_EX1: [u8; 21] = [
            0x3e, 0x13, 0x1, 0x0, 0x40, 0x0, 0x1, 0x0, 0x26, 0xe, 0xd6, 0xe8, 0xc2, 0x50, 0x30,
            0x0, 0x0, 0x0, 0xc0, 0x3, 0x1,
        ];

        let value = HciEventMsg::from_bytes(&LE_CONNECTION_COMPLETE_EX1).unwrap();
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
        assert_eq!(value, expected);

        // Serialize back to bytes and check if it matches the original
        let serialized = value.to_bytes().unwrap();
        assert_eq!(&serialized, &LE_CONNECTION_COMPLETE_EX1);
    }

    #[test]
    fn test_known_byte_array_command_status() {
        const COMMAND_STATUS_EX1: [u8; 6] = [0xf, 0x4, 0x0, 0x1, 0x25, 0x20];
        let value = HciEventMsg::from_bytes(&COMMAND_STATUS_EX1).unwrap();
        let expected = HciEventMsg::CommandStatus {
            status: HciStatus::Success,
            num_hci_command_packets: 0x1,
            command_opcode: 0x2025,
        };
        assert_eq!(value, expected);

        // Serialize back to bytes and check if it matches the original
        let serialized = value.to_bytes().unwrap();
        assert_eq!(&serialized, &COMMAND_STATUS_EX1);
    }
}
