use crate::core::*;
use crate::packer::*;
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
                bytes.pack(status)?;
                bytes.pack(connection_handle)?;
                bytes.pack(role)?;
                bytes.pack(peer_address_type)?;
                bytes.pack(peer_address)?;
                bytes.pack(connection_interval)?;
                bytes.pack(peripheral_latency)?;
                bytes.pack(supervision_timeout)?;
                bytes.pack(central_clock_accuracy)?;
                Ok(())
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
                Ok(())
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
        if true {
            return Ok(HciStatus::Failure(bytes.unpack()?));
        }
        Err(PacketError::Unspecified(format!(
            "No matching variant found for {}",
            stringify!(HciStatus)
        )))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            HciStatus::Success => bytes.pack_bytes(&[0x00]),
            HciStatus::Failure(m0) => {
                Ok(())?;
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
            Role::Central => bytes.pack_bytes(&[0]),
            Role::Peripheral => bytes.pack_bytes(&[1]),
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
            AddressType::Public => bytes.pack_bytes(&[0]),
            AddressType::Random => bytes.pack_bytes(&[1]),
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
            ClockAccuracy::Ppm500 => bytes.pack_bytes(&[0]),
            ClockAccuracy::Ppm250 => bytes.pack_bytes(&[1]),
            ClockAccuracy::Ppm150 => bytes.pack_bytes(&[2]),
            ClockAccuracy::Ppm100 => bytes.pack_bytes(&[3]),
            ClockAccuracy::Ppm75 => bytes.pack_bytes(&[4]),
            ClockAccuracy::Ppm50 => bytes.pack_bytes(&[5]),
            ClockAccuracy::Ppm30 => bytes.pack_bytes(&[6]),
            ClockAccuracy::Ppm20 => bytes.pack_bytes(&[7]),
        }
    }
}
impl FromToPacket for ConnectionHandle {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(Self(bytes.unpack()?))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        self.0.to_packet(bytes)?;
        Ok(())
    }
}
