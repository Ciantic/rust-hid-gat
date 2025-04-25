use crate::core::*;
use crate::packer::*;
impl FromToPacket for HciEventMsg {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0x3e, 0x13, 0x01]) {
            return Ok(HciEventMsg::LeConnectionComplete {
                status: <HciStatus>::from_packet(bytes)?,
                connection_handle: <u16>::from_packet(bytes)?,
                role: <Role>::from_packet(bytes)?,
                peer_address_type: <AddressType>::from_packet(bytes)?,
                peer_address: <[u8; 6]>::from_packet(bytes)?,
                connection_interval: <u16>::from_packet(bytes)?,
                peripheral_latency: <u16>::from_packet(bytes)?,
                supervision_timeout: <u16>::from_packet(bytes)?,
                central_clock_accuracy: <ClockAccuracy>::from_packet(bytes)?,
            });
        }
        if bytes.next_if_eq(&[0x0E, 0x04]) {
            return Ok(HciEventMsg::CommandComplete {
                num_hci_command_packets: <u8>::from_packet(bytes)?,
                command_opcode: <u16>::from_packet(bytes)?,
                status: <HciStatus>::from_packet(bytes)?,
            });
        }
        if bytes.next_if_eq(&[0x0F, 0x04]) {
            return Ok(HciEventMsg::CommandStatus {
                status: <HciStatus>::from_packet(bytes)?,
                num_hci_command_packets: <u8>::from_packet(bytes)?,
                command_opcode: <u16>::from_packet(bytes)?,
            });
        }
        Err(PacketError::Unspecified(format!(
            "No matching variant found for {}",
            stringify!(HciEventMsg)
        )))
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
                status.to_packet(bytes)?;
                connection_handle.to_packet(bytes)?;
                role.to_packet(bytes)?;
                peer_address_type.to_packet(bytes)?;
                peer_address.to_packet(bytes)?;
                connection_interval.to_packet(bytes)?;
                peripheral_latency.to_packet(bytes)?;
                supervision_timeout.to_packet(bytes)?;
                central_clock_accuracy.to_packet(bytes)?;
                Ok(())
            }
            HciEventMsg::CommandComplete {
                num_hci_command_packets,
                command_opcode,
                status,
            } => {
                bytes.pack_bytes(&[0x0E, 0x04])?;
                num_hci_command_packets.to_packet(bytes)?;
                command_opcode.to_packet(bytes)?;
                status.to_packet(bytes)?;
                Ok(())
            }
            HciEventMsg::CommandStatus {
                status,
                num_hci_command_packets,
                command_opcode,
            } => {
                bytes.pack_bytes(&[0x0F, 0x04])?;
                status.to_packet(bytes)?;
                num_hci_command_packets.to_packet(bytes)?;
                command_opcode.to_packet(bytes)?;
                Ok(())
            }
        }
    }
}
impl FromToPacket for HciStatus {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0x00]) {
            return Ok(HciStatus::Success);
        }
        if bytes.next_if_eq(&[0x01]) {
            return Ok(HciStatus::Failure(<u8>::from_packet(bytes)?));
        }
        Err(PacketError::Unspecified(format!(
            "No matching variant found for {}",
            stringify!(HciStatus)
        )))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            HciStatus::Success => {
                bytes.pack_bytes(&[0x00])?;
                Ok(())
            }
            HciStatus::Failure(m0) => {
                bytes.pack_bytes(&[0x01])?;
                m0.to_packet(bytes)?;
                Ok(())
            }
        }
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
        Err(PacketError::Unspecified(format!(
            "No matching variant found for {}",
            stringify!(Role)
        )))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            Role::Central => {
                bytes.pack_bytes(&[0])?;
                Ok(())
            }
            Role::Peripheral => {
                bytes.pack_bytes(&[1])?;
                Ok(())
            }
        }
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
        Err(PacketError::Unspecified(format!(
            "No matching variant found for {}",
            stringify!(AddressType)
        )))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            AddressType::Public => {
                bytes.pack_bytes(&[0])?;
                Ok(())
            }
            AddressType::Random => {
                bytes.pack_bytes(&[1])?;
                Ok(())
            }
        }
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
        Err(PacketError::Unspecified(format!(
            "No matching variant found for {}",
            stringify!(ClockAccuracy)
        )))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            ClockAccuracy::Ppm500 => {
                bytes.pack_bytes(&[0])?;
                Ok(())
            }
            ClockAccuracy::Ppm250 => {
                bytes.pack_bytes(&[1])?;
                Ok(())
            }
            ClockAccuracy::Ppm150 => {
                bytes.pack_bytes(&[2])?;
                Ok(())
            }
            ClockAccuracy::Ppm100 => {
                bytes.pack_bytes(&[3])?;
                Ok(())
            }
            ClockAccuracy::Ppm75 => {
                bytes.pack_bytes(&[4])?;
                Ok(())
            }
            ClockAccuracy::Ppm50 => {
                bytes.pack_bytes(&[5])?;
                Ok(())
            }
            ClockAccuracy::Ppm30 => {
                bytes.pack_bytes(&[6])?;
                Ok(())
            }
            ClockAccuracy::Ppm20 => {
                bytes.pack_bytes(&[7])?;
                Ok(())
            }
        }
    }
}
impl FromToPacket for ConnectionHandle {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(Self(<u16>::from_packet(bytes)?))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        self.0.to_packet(bytes)?;
        Ok(())
    }
}
