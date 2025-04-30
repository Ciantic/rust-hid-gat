use crate::core::*;
use crate::packer::*;
impl FromToPacket for H4Packet {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0x01]) {
            return Ok(H4Packet::HciCommand(bytes.unpack()?));
        }
        if bytes.next_if_eq(&[0x04]) {
            return Ok(H4Packet::HciEvent(bytes.unpack()?));
        }
        if bytes.next_if_eq(&[0x02]) {
            return Ok(H4Packet::HciAcl {
                connection_handle: bytes.set_bits(12).unpack()?,
                pb: bytes.set_bits(2).unpack()?,
                bc: bytes.set_bits(2).unpack()?,
                msg: bytes.unpack_length::<u16>()?.unpack()?,
            });
        }
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(H4Packet)),
            ),
        )
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            H4Packet::HciCommand(m0) => {
                bytes.pack_bytes(&[0x01])?;
                bytes.pack(m0)?;
            }
            H4Packet::HciEvent(m0) => {
                bytes.pack_bytes(&[0x04])?;
                bytes.pack(m0)?;
            }
            H4Packet::HciAcl { connection_handle, pb, bc, msg } => {
                bytes.pack_bytes(&[0x02])?;
                bytes.set_bits(12).pack(connection_handle)?;
                bytes.set_bits(2).pack(pb)?;
                bytes.set_bits(2).pack(bc)?;
                bytes.pack_length::<u16>()?.pack(msg)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for L2CapMessage {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0x06, 0x00]) {
            return Ok(L2CapMessage::Smp(bytes.unpack()?));
        }
        if bytes.next_if_eq(&[0x04, 0x00]) {
            return Ok(L2CapMessage::Att);
        }
        Ok(L2CapMessage::Unknown(bytes.unpack()?, bytes.unpack()?))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            L2CapMessage::Smp(m0) => {
                bytes.pack_bytes(&[0x06, 0x00])?;
                bytes.pack(m0)?;
            }
            L2CapMessage::Att => {
                bytes.pack_bytes(&[0x04, 0x00])?;
            }
            L2CapMessage::Unknown(m0, m1) => {
                bytes.pack(m0)?;
                bytes.pack(m1)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for SmpPdu {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0x03]) {
            return Ok(SmpPdu::SmpPairingConfirmation {
                confirm_value: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x04]) {
            return Ok(SmpPdu::SmpPairingRandom {
                random_value: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x01]) {
            return Ok(SmpPdu::SmpPairingRequest {
                io_capability: bytes.unpack()?,
                oob_data_flag: bytes.unpack()?,
                authentication_requirements: bytes.unpack()?,
                max_encryption_key_size: bytes.unpack()?,
                initiator_key_distribution: bytes.unpack()?,
                responder_key_distribution: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x02]) {
            return Ok(SmpPdu::SmpPairingResponse {
                io_capability: bytes.unpack()?,
                oob_data_flag: bytes.unpack()?,
                authentication_requirements: bytes.unpack()?,
                max_encryption_key_size: bytes.unpack()?,
                initiator_key_distribution: bytes.unpack()?,
                responder_key_distribution: bytes.unpack()?,
            });
        }
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(SmpPdu)),
            ),
        )
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            SmpPdu::SmpPairingConfirmation { confirm_value } => {
                bytes.pack_bytes(&[0x03])?;
                bytes.pack(confirm_value)?;
            }
            SmpPdu::SmpPairingRandom { random_value } => {
                bytes.pack_bytes(&[0x04])?;
                bytes.pack(random_value)?;
            }
            SmpPdu::SmpPairingRequest {
                io_capability,
                oob_data_flag,
                authentication_requirements,
                max_encryption_key_size,
                initiator_key_distribution,
                responder_key_distribution,
            } => {
                bytes.pack_bytes(&[0x01])?;
                bytes.pack(io_capability)?;
                bytes.pack(oob_data_flag)?;
                bytes.pack(authentication_requirements)?;
                bytes.pack(max_encryption_key_size)?;
                bytes.pack(initiator_key_distribution)?;
                bytes.pack(responder_key_distribution)?;
            }
            SmpPdu::SmpPairingResponse {
                io_capability,
                oob_data_flag,
                authentication_requirements,
                max_encryption_key_size,
                initiator_key_distribution,
                responder_key_distribution,
            } => {
                bytes.pack_bytes(&[0x02])?;
                bytes.pack(io_capability)?;
                bytes.pack(oob_data_flag)?;
                bytes.pack(authentication_requirements)?;
                bytes.pack(max_encryption_key_size)?;
                bytes.pack(initiator_key_distribution)?;
                bytes.pack(responder_key_distribution)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for AclMessage {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0x15, 0x00, 0x11, 0x00, 0x06, 0x00, 0x03]) {
            return Ok(AclMessage::SmpPairingConfirmation {
                confirm_value: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x15, 0x00, 0x11, 0x00, 0x06, 0x00, 0x04]) {
            return Ok(AclMessage::SmpPairingRandom {
                random_value: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x0b, 0x00, 0x07, 0x00, 0x06, 0x00, 0x01]) {
            return Ok(AclMessage::SmpPairingRequest {
                io_capability: bytes.unpack()?,
                oob_data_flag: bytes.unpack()?,
                authentication_requirements: bytes.unpack()?,
                max_encryption_key_size: bytes.unpack()?,
                initiator_key_distribution: bytes.unpack()?,
                responder_key_distribution: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x0b, 0x00, 0x07, 0x00, 0x06, 0x00, 0x02]) {
            return Ok(AclMessage::SmpPairingResponse {
                io_capability: bytes.unpack()?,
                oob_data_flag: bytes.unpack()?,
                authentication_requirements: bytes.unpack()?,
                max_encryption_key_size: bytes.unpack()?,
                initiator_key_distribution: bytes.unpack()?,
                responder_key_distribution: bytes.unpack()?,
            });
        }
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(AclMessage)),
            ),
        )
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            AclMessage::SmpPairingConfirmation { confirm_value } => {
                bytes.pack_bytes(&[0x15, 0x00, 0x11, 0x00, 0x06, 0x00, 0x03])?;
                bytes.pack(confirm_value)?;
            }
            AclMessage::SmpPairingRandom { random_value } => {
                bytes.pack_bytes(&[0x15, 0x00, 0x11, 0x00, 0x06, 0x00, 0x04])?;
                bytes.pack(random_value)?;
            }
            AclMessage::SmpPairingRequest {
                io_capability,
                oob_data_flag,
                authentication_requirements,
                max_encryption_key_size,
                initiator_key_distribution,
                responder_key_distribution,
            } => {
                bytes.pack_bytes(&[0x0b, 0x00, 0x07, 0x00, 0x06, 0x00, 0x01])?;
                bytes.pack(io_capability)?;
                bytes.pack(oob_data_flag)?;
                bytes.pack(authentication_requirements)?;
                bytes.pack(max_encryption_key_size)?;
                bytes.pack(initiator_key_distribution)?;
                bytes.pack(responder_key_distribution)?;
            }
            AclMessage::SmpPairingResponse {
                io_capability,
                oob_data_flag,
                authentication_requirements,
                max_encryption_key_size,
                initiator_key_distribution,
                responder_key_distribution,
            } => {
                bytes.pack_bytes(&[0x0b, 0x00, 0x07, 0x00, 0x06, 0x00, 0x02])?;
                bytes.pack(io_capability)?;
                bytes.pack(oob_data_flag)?;
                bytes.pack(authentication_requirements)?;
                bytes.pack(max_encryption_key_size)?;
                bytes.pack(initiator_key_distribution)?;
                bytes.pack(responder_key_distribution)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for HciCommand {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0x03, 0x0C, 0x00]) {
            return Ok(HciCommand::Reset);
        }
        if bytes.next_if_eq(&[0x01, 0x0c]) {
            return Ok(HciCommand::SetEventMask {
                event_mask: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x01, 0x20, 0x08]) {
            return Ok(HciCommand::LeSetEventMask {
                event_mask: bytes.unpack()?,
            });
        }
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(HciCommand)),
            ),
        )
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            HciCommand::Reset => {
                bytes.pack_bytes(&[0x03, 0x0C, 0x00])?;
            }
            HciCommand::SetEventMask { event_mask } => {
                bytes.pack_bytes(&[0x01, 0x0c])?;
                bytes.pack(event_mask)?;
            }
            HciCommand::LeSetEventMask { event_mask } => {
                bytes.pack_bytes(&[0x01, 0x20, 0x08])?;
                bytes.pack(event_mask)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for HciEventMsg {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0x3e, 0x13, 0x01]) {
            return Ok(HciEventMsg::LeConnectionComplete {
                status: bytes.unpack()?,
                connection_handle: bytes.unpack()?,
                role: bytes.unpack()?,
                peer_address_type: bytes.unpack()?,
                peer_address: bytes.unpack()?,
                connection_interval: bytes.unpack()?,
                peripheral_latency: bytes.unpack()?,
                supervision_timeout: bytes.unpack()?,
                central_clock_accuracy: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x0E, 0x04]) {
            return Ok(HciEventMsg::CommandComplete {
                num_hci_command_packets: bytes.unpack()?,
                command_opcode: bytes.unpack()?,
                status: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x0F, 0x04]) {
            return Ok(HciEventMsg::CommandStatus {
                status: bytes.unpack()?,
                num_hci_command_packets: bytes.unpack()?,
                command_opcode: bytes.unpack()?,
            });
        }
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(HciEventMsg)),
            ),
        )
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
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
                bytes.pack_bytes(&[0x3e, 0x13, 0x01])?;
                bytes.pack(status)?;
                bytes.pack(connection_handle)?;
                bytes.pack(role)?;
                bytes.pack(peer_address_type)?;
                bytes.pack(peer_address)?;
                bytes.pack(connection_interval)?;
                bytes.pack(peripheral_latency)?;
                bytes.pack(supervision_timeout)?;
                bytes.pack(central_clock_accuracy)?;
            }
            HciEventMsg::CommandComplete {
                num_hci_command_packets,
                command_opcode,
                status,
            } => {
                bytes.pack_bytes(&[0x0E, 0x04])?;
                bytes.pack(num_hci_command_packets)?;
                bytes.pack(command_opcode)?;
                bytes.pack(status)?;
            }
            HciEventMsg::CommandStatus {
                status,
                num_hci_command_packets,
                command_opcode,
            } => {
                bytes.pack_bytes(&[0x0F, 0x04])?;
                bytes.pack(status)?;
                bytes.pack(num_hci_command_packets)?;
                bytes.pack(command_opcode)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for PacketBoundaryFlag {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0b00]) {
            return Ok(PacketBoundaryFlag::FirstNonFlushable);
        }
        if bytes.next_if_eq(&[0b01]) {
            return Ok(PacketBoundaryFlag::Continuation);
        }
        if bytes.next_if_eq(&[0b10]) {
            return Ok(PacketBoundaryFlag::FirstFlushable);
        }
        if bytes.next_if_eq(&[0b11]) {
            return Ok(PacketBoundaryFlag::Deprecated);
        }
        Err(
            PacketError::Unspecified(
                format!(
                    "No matching variant found for {}", stringify!(PacketBoundaryFlag)
                ),
            ),
        )
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            PacketBoundaryFlag::FirstNonFlushable => {
                bytes.pack_bytes(&[0b00])?;
            }
            PacketBoundaryFlag::Continuation => {
                bytes.pack_bytes(&[0b01])?;
            }
            PacketBoundaryFlag::FirstFlushable => {
                bytes.pack_bytes(&[0b10])?;
            }
            PacketBoundaryFlag::Deprecated => {
                bytes.pack_bytes(&[0b11])?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for BroadcastFlag {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0b00]) {
            return Ok(BroadcastFlag::PointToPoint);
        }
        if bytes.next_if_eq(&[0b01]) {
            return Ok(BroadcastFlag::BdEdrBroadcast);
        }
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(BroadcastFlag)),
            ),
        )
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            BroadcastFlag::PointToPoint => {
                bytes.pack_bytes(&[0b00])?;
            }
            BroadcastFlag::BdEdrBroadcast => {
                bytes.pack_bytes(&[0b01])?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for HciStatus {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0x00]) {
            return Ok(HciStatus::Success);
        }
        Ok(HciStatus::Failure(bytes.unpack()?))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            HciStatus::Success => {
                bytes.pack_bytes(&[0x00])?;
            }
            HciStatus::Failure(m0) => {
                bytes.pack(m0)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for Role {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0]) {
            return Ok(Role::Central);
        }
        if bytes.next_if_eq(&[1]) {
            return Ok(Role::Peripheral);
        }
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(Role)),
            ),
        )
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            Role::Central => {
                bytes.pack_bytes(&[0])?;
            }
            Role::Peripheral => {
                bytes.pack_bytes(&[1])?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for AddressType {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0]) {
            return Ok(AddressType::Public);
        }
        if bytes.next_if_eq(&[1]) {
            return Ok(AddressType::Random);
        }
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(AddressType)),
            ),
        )
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            AddressType::Public => {
                bytes.pack_bytes(&[0])?;
            }
            AddressType::Random => {
                bytes.pack_bytes(&[1])?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for ClockAccuracy {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0]) {
            return Ok(ClockAccuracy::Ppm500);
        }
        if bytes.next_if_eq(&[1]) {
            return Ok(ClockAccuracy::Ppm250);
        }
        if bytes.next_if_eq(&[2]) {
            return Ok(ClockAccuracy::Ppm150);
        }
        if bytes.next_if_eq(&[3]) {
            return Ok(ClockAccuracy::Ppm100);
        }
        if bytes.next_if_eq(&[4]) {
            return Ok(ClockAccuracy::Ppm75);
        }
        if bytes.next_if_eq(&[5]) {
            return Ok(ClockAccuracy::Ppm50);
        }
        if bytes.next_if_eq(&[6]) {
            return Ok(ClockAccuracy::Ppm30);
        }
        if bytes.next_if_eq(&[7]) {
            return Ok(ClockAccuracy::Ppm20);
        }
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(ClockAccuracy)),
            ),
        )
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            ClockAccuracy::Ppm500 => {
                bytes.pack_bytes(&[0])?;
            }
            ClockAccuracy::Ppm250 => {
                bytes.pack_bytes(&[1])?;
            }
            ClockAccuracy::Ppm150 => {
                bytes.pack_bytes(&[2])?;
            }
            ClockAccuracy::Ppm100 => {
                bytes.pack_bytes(&[3])?;
            }
            ClockAccuracy::Ppm75 => {
                bytes.pack_bytes(&[4])?;
            }
            ClockAccuracy::Ppm50 => {
                bytes.pack_bytes(&[5])?;
            }
            ClockAccuracy::Ppm30 => {
                bytes.pack_bytes(&[6])?;
            }
            ClockAccuracy::Ppm20 => {
                bytes.pack_bytes(&[7])?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for ConnectionHandle {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(ConnectionHandle(bytes.unpack()?))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            ConnectionHandle(m0) => {
                bytes.pack(m0)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for KeyDistributionFlags {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(KeyDistributionFlags {
            enc_key: bytes.set_bits(1).unpack()?,
            id_key: bytes.set_bits(1).unpack()?,
            sign_key: bytes.set_bits(1).unpack()?,
            link_key: bytes.set_bits(1).unpack()?,
            _reserved: bytes.set_bits(4).unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            KeyDistributionFlags { enc_key, id_key, sign_key, link_key, _reserved } => {
                bytes.set_bits(1).pack(enc_key)?;
                bytes.set_bits(1).pack(id_key)?;
                bytes.set_bits(1).pack(sign_key)?;
                bytes.set_bits(1).pack(link_key)?;
                bytes.set_bits(4).pack(_reserved)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for AuthenticationRequirements {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(AuthenticationRequirements {
            bonding: bytes.set_bits(2).unpack()?,
            mitm_protection: bytes.set_bits(1).unpack()?,
            secure_connections: bytes.set_bits(1).unpack()?,
            keypress_notification: bytes.set_bits(1).unpack()?,
            ct2: bytes.set_bits(1).unpack()?,
            _reserved: bytes.set_bits(2).unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            AuthenticationRequirements {
                bonding,
                mitm_protection,
                secure_connections,
                keypress_notification,
                ct2,
                _reserved,
            } => {
                bytes.set_bits(2).pack(bonding)?;
                bytes.set_bits(1).pack(mitm_protection)?;
                bytes.set_bits(1).pack(secure_connections)?;
                bytes.set_bits(1).pack(keypress_notification)?;
                bytes.set_bits(1).pack(ct2)?;
                bytes.set_bits(2).pack(_reserved)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for IOCapability {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0x00]) {
            return Ok(IOCapability::DisplayOnly);
        }
        if bytes.next_if_eq(&[0x01]) {
            return Ok(IOCapability::DisplayYesNo);
        }
        if bytes.next_if_eq(&[0x02]) {
            return Ok(IOCapability::KeyboardOnly);
        }
        if bytes.next_if_eq(&[0x03]) {
            return Ok(IOCapability::NoInputNoOutput);
        }
        if bytes.next_if_eq(&[0x04]) {
            return Ok(IOCapability::KeyboardDisplay);
        }
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(IOCapability)),
            ),
        )
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            IOCapability::DisplayOnly => {
                bytes.pack_bytes(&[0x00])?;
            }
            IOCapability::DisplayYesNo => {
                bytes.pack_bytes(&[0x01])?;
            }
            IOCapability::KeyboardOnly => {
                bytes.pack_bytes(&[0x02])?;
            }
            IOCapability::NoInputNoOutput => {
                bytes.pack_bytes(&[0x03])?;
            }
            IOCapability::KeyboardDisplay => {
                bytes.pack_bytes(&[0x04])?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for OOBDataFlag {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0x00]) {
            return Ok(OOBDataFlag::OobNotAvailable);
        }
        if bytes.next_if_eq(&[0x01]) {
            return Ok(OOBDataFlag::OobAvailable);
        }
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(OOBDataFlag)),
            ),
        )
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            OOBDataFlag::OobNotAvailable => {
                bytes.pack_bytes(&[0x00])?;
            }
            OOBDataFlag::OobAvailable => {
                bytes.pack_bytes(&[0x01])?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for SmpPairingFailure {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0x01]) {
            return Ok(SmpPairingFailure::PasskeyEntryFailed);
        }
        if bytes.next_if_eq(&[0x02]) {
            return Ok(SmpPairingFailure::OobNotAvailable);
        }
        if bytes.next_if_eq(&[0x03]) {
            return Ok(SmpPairingFailure::AuthenticationRequirements);
        }
        if bytes.next_if_eq(&[0x04]) {
            return Ok(SmpPairingFailure::ConfirmValueFailed);
        }
        if bytes.next_if_eq(&[0x05]) {
            return Ok(SmpPairingFailure::PairingNotSupported);
        }
        if bytes.next_if_eq(&[0x06]) {
            return Ok(SmpPairingFailure::EncryptionKeySize);
        }
        if bytes.next_if_eq(&[0x07]) {
            return Ok(SmpPairingFailure::CommandNotSupported);
        }
        if bytes.next_if_eq(&[0x08]) {
            return Ok(SmpPairingFailure::UnspecifiedReason);
        }
        if bytes.next_if_eq(&[0x09]) {
            return Ok(SmpPairingFailure::RepeatedAttempts);
        }
        if bytes.next_if_eq(&[0x0A]) {
            return Ok(SmpPairingFailure::InvalidParameters);
        }
        if bytes.next_if_eq(&[0x0B]) {
            return Ok(SmpPairingFailure::DhKeyCheckFailed);
        }
        if bytes.next_if_eq(&[0x0C]) {
            return Ok(SmpPairingFailure::NumericComparisonFailed);
        }
        if bytes.next_if_eq(&[0x0D]) {
            return Ok(SmpPairingFailure::BrEdrPairingInProgress);
        }
        if bytes.next_if_eq(&[0x0E]) {
            return Ok(
                SmpPairingFailure::CrossTransportKeyDerivationGenerationNotAllowed,
            );
        }
        if bytes.next_if_eq(&[0x0F]) {
            return Ok(SmpPairingFailure::KeyRejected);
        }
        if bytes.next_if_eq(&[0x10]) {
            return Ok(SmpPairingFailure::Busy);
        }
        Err(
            PacketError::Unspecified(
                format!(
                    "No matching variant found for {}", stringify!(SmpPairingFailure)
                ),
            ),
        )
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            SmpPairingFailure::PasskeyEntryFailed => {
                bytes.pack_bytes(&[0x01])?;
            }
            SmpPairingFailure::OobNotAvailable => {
                bytes.pack_bytes(&[0x02])?;
            }
            SmpPairingFailure::AuthenticationRequirements => {
                bytes.pack_bytes(&[0x03])?;
            }
            SmpPairingFailure::ConfirmValueFailed => {
                bytes.pack_bytes(&[0x04])?;
            }
            SmpPairingFailure::PairingNotSupported => {
                bytes.pack_bytes(&[0x05])?;
            }
            SmpPairingFailure::EncryptionKeySize => {
                bytes.pack_bytes(&[0x06])?;
            }
            SmpPairingFailure::CommandNotSupported => {
                bytes.pack_bytes(&[0x07])?;
            }
            SmpPairingFailure::UnspecifiedReason => {
                bytes.pack_bytes(&[0x08])?;
            }
            SmpPairingFailure::RepeatedAttempts => {
                bytes.pack_bytes(&[0x09])?;
            }
            SmpPairingFailure::InvalidParameters => {
                bytes.pack_bytes(&[0x0A])?;
            }
            SmpPairingFailure::DhKeyCheckFailed => {
                bytes.pack_bytes(&[0x0B])?;
            }
            SmpPairingFailure::NumericComparisonFailed => {
                bytes.pack_bytes(&[0x0C])?;
            }
            SmpPairingFailure::BrEdrPairingInProgress => {
                bytes.pack_bytes(&[0x0D])?;
            }
            SmpPairingFailure::CrossTransportKeyDerivationGenerationNotAllowed => {
                bytes.pack_bytes(&[0x0E])?;
            }
            SmpPairingFailure::KeyRejected => {
                bytes.pack_bytes(&[0x0F])?;
            }
            SmpPairingFailure::Busy => {
                bytes.pack_bytes(&[0x10])?;
            }
        };
        Ok(())
    }
}
