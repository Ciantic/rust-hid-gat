use crate::messages::*;
use crate::packer::*;
impl FromToPacket for H4Packet {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<u8>(&0x01) {
            return Ok(H4Packet::Command(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x04) {
            return Ok(H4Packet::Event(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x02) {
            return Ok(H4Packet::Acl(bytes.unpack()?));
        }
        Err(PacketError::Unspecified(format!(
            "No matching variant found for {}",
            stringify!(H4Packet)
        )))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            H4Packet::Command(m0) => {
                bytes.pack::<u8>(&0x01)?;
                bytes.pack(m0)?;
            }
            H4Packet::Event(m0) => {
                bytes.pack::<u8>(&0x04)?;
                bytes.pack(m0)?;
            }
            H4Packet::Acl(m0) => {
                bytes.pack::<u8>(&0x02)?;
                bytes.pack(m0)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u8> for H4Packet {
    fn get_id(&self) -> u8 {
        match self {
            H4Packet::Command(m0) => 0x01,
            H4Packet::Event(m0) => 0x04,
            H4Packet::Acl(m0) => 0x02,
        }
    }
}
impl FromToPacket for HciCommand {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0006, 0x01)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::Disconnect(bytes.unpack()?));
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0003, 0x03)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::Reset);
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0001, 0x03)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::SetEventMask(bytes.unpack()?));
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0002, 0x04)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::ReadLocalSupportedCommands);
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0009, 0x04)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::ReadBdAddr);
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x001a, 0x03)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::WriteScanEnable(bytes.unpack()?));
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0016, 0x03)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::WriteConnectionAcceptTimeout(bytes.unpack()?));
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0018, 0x03)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::WritePageTimeout(bytes.unpack()?));
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0013, 0x03)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::WriteLocalName(bytes.unpack()?));
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0014, 0x03)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::ReadLocalName(bytes.unpack()?));
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0001, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeSetEventMask(bytes.unpack()?));
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0002, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeReadBufferSize);
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0005, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeSetRandomAddress(bytes.unpack()?));
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0006, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeSetAdvertisingParameters(bytes.unpack()?));
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0008, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeSetAdvertisingData(bytes.unpack()?));
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0025, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeReadLocalP256PublicKey);
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x000A, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeSetAdvertisingEnable(bytes.unpack()?));
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0022, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeSetDataLength(bytes.unpack()?));
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x001A, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeLongTermKeyRequestReply(bytes.unpack()?));
        }
        Err(PacketError::Unspecified(format!(
            "No matching variant found for {}",
            stringify!(HciCommand)
        )))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            HciCommand::Disconnect(m0) => {
                bytes.pack::<OpCode>(&OpCode(0x0006, 0x01))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciCommand::Reset => {
                bytes.pack::<OpCode>(&OpCode(0x0003, 0x03))?;
                bytes.pack_length::<u8>()?;
            }
            HciCommand::SetEventMask(m0) => {
                bytes.pack::<OpCode>(&OpCode(0x0001, 0x03))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciCommand::ReadLocalSupportedCommands => {
                bytes.pack::<OpCode>(&OpCode(0x0002, 0x04))?;
                bytes.pack_length::<u8>()?;
            }
            HciCommand::ReadBdAddr => {
                bytes.pack::<OpCode>(&OpCode(0x0009, 0x04))?;
                bytes.pack_length::<u8>()?;
            }
            HciCommand::WriteScanEnable(m0) => {
                bytes.pack::<OpCode>(&OpCode(0x001a, 0x03))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciCommand::WriteConnectionAcceptTimeout(m0) => {
                bytes.pack::<OpCode>(&OpCode(0x0016, 0x03))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciCommand::WritePageTimeout(m0) => {
                bytes.pack::<OpCode>(&OpCode(0x0018, 0x03))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciCommand::WriteLocalName(m0) => {
                bytes.pack::<OpCode>(&OpCode(0x0013, 0x03))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciCommand::ReadLocalName(m0) => {
                bytes.pack::<OpCode>(&OpCode(0x0014, 0x03))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciCommand::LeSetEventMask(m0) => {
                bytes.pack::<OpCode>(&OpCode(0x0001, 0x08))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciCommand::LeReadBufferSize => {
                bytes.pack::<OpCode>(&OpCode(0x0002, 0x08))?;
                bytes.pack_length::<u8>()?;
            }
            HciCommand::LeSetRandomAddress(m0) => {
                bytes.pack::<OpCode>(&OpCode(0x0005, 0x08))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciCommand::LeSetAdvertisingParameters(m0) => {
                bytes.pack::<OpCode>(&OpCode(0x0006, 0x08))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciCommand::LeSetAdvertisingData(m0) => {
                bytes.pack::<OpCode>(&OpCode(0x0008, 0x08))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciCommand::LeReadLocalP256PublicKey => {
                bytes.pack::<OpCode>(&OpCode(0x0025, 0x08))?;
                bytes.pack_length::<u8>()?;
            }
            HciCommand::LeSetAdvertisingEnable(m0) => {
                bytes.pack::<OpCode>(&OpCode(0x000A, 0x08))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciCommand::LeSetDataLength(m0) => {
                bytes.pack::<OpCode>(&OpCode(0x0022, 0x08))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciCommand::LeLongTermKeyRequestReply(m0) => {
                bytes.pack::<OpCode>(&OpCode(0x001A, 0x08))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<OpCode> for HciCommand {
    fn get_id(&self) -> OpCode {
        match self {
            HciCommand::Disconnect(m0) => OpCode(0x0006, 0x01),
            HciCommand::Reset => OpCode(0x0003, 0x03),
            HciCommand::SetEventMask(m0) => OpCode(0x0001, 0x03),
            HciCommand::ReadLocalSupportedCommands => OpCode(0x0002, 0x04),
            HciCommand::ReadBdAddr => OpCode(0x0009, 0x04),
            HciCommand::WriteScanEnable(m0) => OpCode(0x001a, 0x03),
            HciCommand::WriteConnectionAcceptTimeout(m0) => OpCode(0x0016, 0x03),
            HciCommand::WritePageTimeout(m0) => OpCode(0x0018, 0x03),
            HciCommand::WriteLocalName(m0) => OpCode(0x0013, 0x03),
            HciCommand::ReadLocalName(m0) => OpCode(0x0014, 0x03),
            HciCommand::LeSetEventMask(m0) => OpCode(0x0001, 0x08),
            HciCommand::LeReadBufferSize => OpCode(0x0002, 0x08),
            HciCommand::LeSetRandomAddress(m0) => OpCode(0x0005, 0x08),
            HciCommand::LeSetAdvertisingParameters(m0) => OpCode(0x0006, 0x08),
            HciCommand::LeSetAdvertisingData(m0) => OpCode(0x0008, 0x08),
            HciCommand::LeReadLocalP256PublicKey => OpCode(0x0025, 0x08),
            HciCommand::LeSetAdvertisingEnable(m0) => OpCode(0x000A, 0x08),
            HciCommand::LeSetDataLength(m0) => OpCode(0x0022, 0x08),
            HciCommand::LeLongTermKeyRequestReply(m0) => OpCode(0x001A, 0x08),
        }
    }
}
impl FromToPacket for HciEvent {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<u8>(&0x05) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEvent::DisconnectComplete(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x08) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEvent::EncryptionChange(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x13) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEvent::NumberOfCompletedPackets(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x3e) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEvent::LeMeta(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x0E) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEvent::CommandComplete(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x0F) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEvent::CommandStatus(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0xFF) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEvent::VendorSpecific(bytes.unpack()?));
        }
        Err(PacketError::Unspecified(format!(
            "No matching variant found for {}",
            stringify!(HciEvent)
        )))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            HciEvent::DisconnectComplete(m0) => {
                bytes.pack::<u8>(&0x05)?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciEvent::EncryptionChange(m0) => {
                bytes.pack::<u8>(&0x08)?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciEvent::NumberOfCompletedPackets(m0) => {
                bytes.pack::<u8>(&0x13)?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciEvent::LeMeta(m0) => {
                bytes.pack::<u8>(&0x3e)?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciEvent::CommandComplete(m0) => {
                bytes.pack::<u8>(&0x0E)?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciEvent::CommandStatus(m0) => {
                bytes.pack::<u8>(&0x0F)?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciEvent::VendorSpecific(m0) => {
                bytes.pack::<u8>(&0xFF)?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u8> for HciEvent {
    fn get_id(&self) -> u8 {
        match self {
            HciEvent::DisconnectComplete(m0) => 0x05,
            HciEvent::EncryptionChange(m0) => 0x08,
            HciEvent::NumberOfCompletedPackets(m0) => 0x13,
            HciEvent::LeMeta(m0) => 0x3e,
            HciEvent::CommandComplete(m0) => 0x0E,
            HciEvent::CommandStatus(m0) => 0x0F,
            HciEvent::VendorSpecific(m0) => 0xFF,
        }
    }
}
impl FromToPacket for HciAcl {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(HciAcl {
            connection_handle: bytes.set_bits(12).unpack()?,
            pb: bytes.set_bits(2).unpack()?,
            bc: bytes.set_bits(2).unpack()?,
            msg: bytes.unpack_length::<u16>()?.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            HciAcl {
                connection_handle,
                pb,
                bc,
                msg,
            } => {
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
        bytes.unpack_length::<u16>()?;
        if bytes.next_if_eq::<u16>(&0x0006) {
            return Ok(L2CapMessage::Smp(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u16>(&0x0004) {
            return Ok(L2CapMessage::Att(bytes.unpack()?));
        }
        Ok(L2CapMessage::Unknown(bytes.unpack()?, bytes.unpack()?))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        bytes.pack_length_with_offset::<u16>(-2)?;
        match self {
            L2CapMessage::Smp(m0) => {
                bytes.pack::<u16>(&0x0006)?;
                bytes.pack(m0)?;
            }
            L2CapMessage::Att(m0) => {
                bytes.pack::<u16>(&0x0004)?;
                bytes.pack(m0)?;
            }
            L2CapMessage::Unknown(m0, m1) => {
                bytes.pack(m0)?;
                bytes.pack(m1)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u16> for L2CapMessage {
    fn get_id(&self) -> u16 {
        match self {
            L2CapMessage::Smp(m0) => 0x0006,
            L2CapMessage::Att(m0) => 0x0004,
            L2CapMessage::Unknown(m0, m1) => m0.clone(),
        }
    }
}
impl FromToPacket for AttPdu {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<u8>(&0x02) {
            return Ok(AttPdu::ExchangeMtuRequest(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x03) {
            return Ok(AttPdu::ExchangeMtuResponse(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x04) {
            return Ok(AttPdu::FindInformationRequest(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x05) {
            return Ok(AttPdu::FindInformationResponse(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x06) {
            return Ok(AttPdu::FindByTypeValueRequest(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x07) {
            return Ok(AttPdu::FindByTypeValueResponse(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x08) {
            return Ok(AttPdu::ReadByTypeRequest(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x09) {
            return Ok(AttPdu::ReadByTypeResponse(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x0A) {
            return Ok(AttPdu::ReadRequest(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x0B) {
            return Ok(AttPdu::ReadResponse(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x18) {
            return Ok(AttPdu::ExecuteWriteRequest(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x19) {
            return Ok(AttPdu::ExecuteWriteResponse);
        }
        if bytes.next_if_eq::<u8>(&0x1B) {
            return Ok(AttPdu::HandleValueNotification(bytes.unpack()?));
        }
        Ok(AttPdu::Unknown(bytes.unpack()?, bytes.unpack()?))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            AttPdu::ExchangeMtuRequest(m0) => {
                bytes.pack::<u8>(&0x02)?;
                bytes.pack(m0)?;
            }
            AttPdu::ExchangeMtuResponse(m0) => {
                bytes.pack::<u8>(&0x03)?;
                bytes.pack(m0)?;
            }
            AttPdu::FindInformationRequest(m0) => {
                bytes.pack::<u8>(&0x04)?;
                bytes.pack(m0)?;
            }
            AttPdu::FindInformationResponse(m0) => {
                bytes.pack::<u8>(&0x05)?;
                bytes.pack(m0)?;
            }
            AttPdu::FindByTypeValueRequest(m0) => {
                bytes.pack::<u8>(&0x06)?;
                bytes.pack(m0)?;
            }
            AttPdu::FindByTypeValueResponse(m0) => {
                bytes.pack::<u8>(&0x07)?;
                bytes.pack(m0)?;
            }
            AttPdu::ReadByTypeRequest(m0) => {
                bytes.pack::<u8>(&0x08)?;
                bytes.pack(m0)?;
            }
            AttPdu::ReadByTypeResponse(m0) => {
                bytes.pack::<u8>(&0x09)?;
                bytes.pack(m0)?;
            }
            AttPdu::ReadRequest(m0) => {
                bytes.pack::<u8>(&0x0A)?;
                bytes.pack(m0)?;
            }
            AttPdu::ReadResponse(m0) => {
                bytes.pack::<u8>(&0x0B)?;
                bytes.pack(m0)?;
            }
            AttPdu::ExecuteWriteRequest(m0) => {
                bytes.pack::<u8>(&0x18)?;
                bytes.pack(m0)?;
            }
            AttPdu::ExecuteWriteResponse => {
                bytes.pack::<u8>(&0x19)?;
            }
            AttPdu::HandleValueNotification(m0) => {
                bytes.pack::<u8>(&0x1B)?;
                bytes.pack(m0)?;
            }
            AttPdu::Unknown(m0, m1) => {
                bytes.pack(m0)?;
                bytes.pack(m1)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u8> for AttPdu {
    fn get_id(&self) -> u8 {
        match self {
            AttPdu::ExchangeMtuRequest(m0) => 0x02,
            AttPdu::ExchangeMtuResponse(m0) => 0x03,
            AttPdu::FindInformationRequest(m0) => 0x04,
            AttPdu::FindInformationResponse(m0) => 0x05,
            AttPdu::FindByTypeValueRequest(m0) => 0x06,
            AttPdu::FindByTypeValueResponse(m0) => 0x07,
            AttPdu::ReadByTypeRequest(m0) => 0x08,
            AttPdu::ReadByTypeResponse(m0) => 0x09,
            AttPdu::ReadRequest(m0) => 0x0A,
            AttPdu::ReadResponse(m0) => 0x0B,
            AttPdu::ExecuteWriteRequest(m0) => 0x18,
            AttPdu::ExecuteWriteResponse => 0x19,
            AttPdu::HandleValueNotification(m0) => 0x1B,
            AttPdu::Unknown(m0, m1) => m0.clone(),
        }
    }
}
impl FromToPacket for SmpPdu {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<u8>(&0x01) {
            return Ok(SmpPdu::PairingRequest(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x02) {
            return Ok(SmpPdu::PairingResponse(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x03) {
            return Ok(SmpPdu::PairingConfirmation(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x04) {
            return Ok(SmpPdu::PairingRandom(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x05) {
            return Ok(SmpPdu::PairingFailed(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x06) {
            return Ok(SmpPdu::EncryptionInformation(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x07) {
            return Ok(SmpPdu::CentralIdentification(bytes.unpack()?));
        }
        Err(PacketError::Unspecified(format!(
            "No matching variant found for {}",
            stringify!(SmpPdu)
        )))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            SmpPdu::PairingRequest(m0) => {
                bytes.pack::<u8>(&0x01)?;
                bytes.pack(m0)?;
            }
            SmpPdu::PairingResponse(m0) => {
                bytes.pack::<u8>(&0x02)?;
                bytes.pack(m0)?;
            }
            SmpPdu::PairingConfirmation(m0) => {
                bytes.pack::<u8>(&0x03)?;
                bytes.pack(m0)?;
            }
            SmpPdu::PairingRandom(m0) => {
                bytes.pack::<u8>(&0x04)?;
                bytes.pack(m0)?;
            }
            SmpPdu::PairingFailed(m0) => {
                bytes.pack::<u8>(&0x05)?;
                bytes.pack(m0)?;
            }
            SmpPdu::EncryptionInformation(m0) => {
                bytes.pack::<u8>(&0x06)?;
                bytes.pack(m0)?;
            }
            SmpPdu::CentralIdentification(m0) => {
                bytes.pack::<u8>(&0x07)?;
                bytes.pack(m0)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u8> for SmpPdu {
    fn get_id(&self) -> u8 {
        match self {
            SmpPdu::PairingRequest(m0) => 0x01,
            SmpPdu::PairingResponse(m0) => 0x02,
            SmpPdu::PairingConfirmation(m0) => 0x03,
            SmpPdu::PairingRandom(m0) => 0x04,
            SmpPdu::PairingFailed(m0) => 0x05,
            SmpPdu::EncryptionInformation(m0) => 0x06,
            SmpPdu::CentralIdentification(m0) => 0x07,
        }
    }
}
impl FromToPacket for EvtLeMeta {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<u8>(&0x01) {
            return Ok(EvtLeMeta::LeConnectionComplete(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x02) {
            return Ok(EvtLeMeta::LeAdvertisingReport(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x03) {
            return Ok(EvtLeMeta::LeConnectionUpdateComplete(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x04) {
            return Ok(EvtLeMeta::LeReadRemoteFeaturesPage0Complete(
                bytes.unpack()?,
            ));
        }
        if bytes.next_if_eq::<u8>(&0x05) {
            return Ok(EvtLeMeta::LeLongTermKeyRequest(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x07) {
            return Ok(EvtLeMeta::LeDataLengthChange(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x08) {
            return Ok(EvtLeMeta::LeReadLocalP256PublicKeyComplete(bytes.unpack()?));
        }
        Err(PacketError::Unspecified(format!(
            "No matching variant found for {}",
            stringify!(EvtLeMeta)
        )))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            EvtLeMeta::LeConnectionComplete(m0) => {
                bytes.pack::<u8>(&0x01)?;
                bytes.pack(m0)?;
            }
            EvtLeMeta::LeAdvertisingReport(m0) => {
                bytes.pack::<u8>(&0x02)?;
                bytes.pack(m0)?;
            }
            EvtLeMeta::LeConnectionUpdateComplete(m0) => {
                bytes.pack::<u8>(&0x03)?;
                bytes.pack(m0)?;
            }
            EvtLeMeta::LeReadRemoteFeaturesPage0Complete(m0) => {
                bytes.pack::<u8>(&0x04)?;
                bytes.pack(m0)?;
            }
            EvtLeMeta::LeLongTermKeyRequest(m0) => {
                bytes.pack::<u8>(&0x05)?;
                bytes.pack(m0)?;
            }
            EvtLeMeta::LeDataLengthChange(m0) => {
                bytes.pack::<u8>(&0x07)?;
                bytes.pack(m0)?;
            }
            EvtLeMeta::LeReadLocalP256PublicKeyComplete(m0) => {
                bytes.pack::<u8>(&0x08)?;
                bytes.pack(m0)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u8> for EvtLeMeta {
    fn get_id(&self) -> u8 {
        match self {
            EvtLeMeta::LeConnectionComplete(m0) => 0x01,
            EvtLeMeta::LeAdvertisingReport(m0) => 0x02,
            EvtLeMeta::LeConnectionUpdateComplete(m0) => 0x03,
            EvtLeMeta::LeReadRemoteFeaturesPage0Complete(m0) => 0x04,
            EvtLeMeta::LeLongTermKeyRequest(m0) => 0x05,
            EvtLeMeta::LeDataLengthChange(m0) => 0x07,
            EvtLeMeta::LeReadLocalP256PublicKeyComplete(m0) => 0x08,
        }
    }
}
impl FromToPacket for AttFindInformationRequest {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(AttFindInformationRequest {
            starting_handle: bytes.unpack()?,
            ending_handle: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            AttFindInformationRequest {
                starting_handle,
                ending_handle,
            } => {
                bytes.pack(starting_handle)?;
                bytes.pack(ending_handle)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for AttFindInformationResponse {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(AttFindInformationResponse {
            format: bytes.unpack()?,
            information: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            AttFindInformationResponse {
                format,
                information,
            } => {
                bytes.pack(format)?;
                bytes.pack(information)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for AttFindByTypeValueRequest {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(AttFindByTypeValueRequest {
            starting_handle: bytes.unpack()?,
            ending_handle: bytes.unpack()?,
            uuid: bytes.unpack()?,
            value: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            AttFindByTypeValueRequest {
                starting_handle,
                ending_handle,
                uuid,
                value,
            } => {
                bytes.pack(starting_handle)?;
                bytes.pack(ending_handle)?;
                bytes.pack(uuid)?;
                bytes.pack(value)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for AttFindByTypeValueResponse {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(AttFindByTypeValueResponse {
            handles_information: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            AttFindByTypeValueResponse {
                handles_information,
            } => {
                bytes.pack(handles_information)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for AttReadByTypeRequest {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(AttReadByTypeRequest {
            starting_handle: bytes.unpack()?,
            ending_handle: bytes.unpack()?,
            uuid: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            AttReadByTypeRequest {
                starting_handle,
                ending_handle,
                uuid,
            } => {
                bytes.pack(starting_handle)?;
                bytes.pack(ending_handle)?;
                bytes.pack(uuid)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for AttReadByTypeResponse {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(AttReadByTypeResponse {
            pair_length: bytes.unpack()?,
            values: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            AttReadByTypeResponse {
                pair_length,
                values,
            } => {
                bytes.pack(pair_length)?;
                bytes.pack(values)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for AttReadRequest {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(AttReadRequest {
            handle: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            AttReadRequest { handle } => {
                bytes.pack(handle)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for AttReadResponse {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(AttReadResponse {
            value: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            AttReadResponse { value } => {
                bytes.pack(value)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for AttExecuteWriteRequest {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(AttExecuteWriteRequest {
            flags: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            AttExecuteWriteRequest { flags } => {
                bytes.pack(flags)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for AttHandleValueNotification {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(AttHandleValueNotification {
            handle: bytes.unpack()?,
            value: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            AttHandleValueNotification { handle, value } => {
                bytes.pack(handle)?;
                bytes.pack(value)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for SmpPairingReqRes {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(SmpPairingReqRes {
            io_capability: bytes.unpack()?,
            oob_data_flag: bytes.unpack()?,
            authentication_requirements: bytes.unpack()?,
            max_encryption_key_size: bytes.unpack()?,
            initiator_key_distribution: bytes.unpack()?,
            responder_key_distribution: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            SmpPairingReqRes {
                io_capability,
                oob_data_flag,
                authentication_requirements,
                max_encryption_key_size,
                initiator_key_distribution,
                responder_key_distribution,
            } => {
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
impl FromToPacket for SmpPairingConfirmation {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(SmpPairingConfirmation {
            confirm_value: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            SmpPairingConfirmation { confirm_value } => {
                bytes.pack(confirm_value)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for SmpPairingRandom {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(SmpPairingRandom {
            random_value: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            SmpPairingRandom { random_value } => {
                bytes.pack(random_value)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for SmpEncryptionInformation {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(SmpEncryptionInformation {
            long_term_key: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            SmpEncryptionInformation { long_term_key } => {
                bytes.pack(long_term_key)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for SmpCentralIdentification {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(SmpCentralIdentification {
            encrypted_diversifier: bytes.unpack()?,
            random_number: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            SmpCentralIdentification {
                encrypted_diversifier,
                random_number,
            } => {
                bytes.pack(encrypted_diversifier)?;
                bytes.pack(random_number)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for CmdDisconnect {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(CmdDisconnect {
            connection_handle: bytes.unpack()?,
            reason: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            CmdDisconnect {
                connection_handle,
                reason,
            } => {
                bytes.pack(connection_handle)?;
                bytes.pack(reason)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for CmdReadLocalName {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(CmdReadLocalName {
            status: bytes.unpack()?,
            name: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            CmdReadLocalName { status, name } => {
                bytes.pack(status)?;
                bytes.pack(name)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for LeSetAdvertisingParameters {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(LeSetAdvertisingParameters {
            advertising_interval_min: bytes.unpack()?,
            advertising_interval_max: bytes.unpack()?,
            advertising_type: bytes.unpack()?,
            own_address_type: bytes.unpack()?,
            peer_address_type: bytes.unpack()?,
            peer_address: bytes.unpack()?,
            advertising_channel_map: bytes.unpack()?,
            advertising_filter_policy: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            LeSetAdvertisingParameters {
                advertising_interval_min,
                advertising_interval_max,
                advertising_type,
                own_address_type,
                peer_address_type,
                peer_address,
                advertising_channel_map,
                advertising_filter_policy,
            } => {
                bytes.pack(advertising_interval_min)?;
                bytes.pack(advertising_interval_max)?;
                bytes.pack(advertising_type)?;
                bytes.pack(own_address_type)?;
                bytes.pack(peer_address_type)?;
                bytes.pack(peer_address)?;
                bytes.pack(advertising_channel_map)?;
                bytes.pack(advertising_filter_policy)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for LeSetAdvertisingData {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(LeSetAdvertisingData {
            advertising_data_length: bytes.unpack()?,
            advertising_data: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            LeSetAdvertisingData {
                advertising_data_length,
                advertising_data,
            } => {
                bytes.pack(advertising_data_length)?;
                bytes.pack(advertising_data)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for LeSetDataLength {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(LeSetDataLength {
            connection_handle: bytes.unpack()?,
            tx_octets: bytes.unpack()?,
            tx_time: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            LeSetDataLength {
                connection_handle,
                tx_octets,
                tx_time,
            } => {
                bytes.pack(connection_handle)?;
                bytes.pack(tx_octets)?;
                bytes.pack(tx_time)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for LeLongTermKeyRequestReply {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(LeLongTermKeyRequestReply {
            connection_handle: bytes.unpack()?,
            long_term_key: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            LeLongTermKeyRequestReply {
                connection_handle,
                long_term_key,
            } => {
                bytes.pack(connection_handle)?;
                bytes.pack(long_term_key)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for OpCode {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(OpCode(
            bytes.set_bits(10).unpack()?,
            bytes.set_bits(6).unpack()?,
        ))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            OpCode(m0, m1) => {
                bytes.set_bits(10).pack(m0)?;
                bytes.set_bits(6).pack(m1)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for CmdScanEnable {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<u8>(&0x00) {
            return Ok(CmdScanEnable::NoScans);
        }
        if bytes.next_if_eq::<u8>(&0x01) {
            return Ok(CmdScanEnable::InquiryScanEnabled_PageScanDisabled);
        }
        if bytes.next_if_eq::<u8>(&0x02) {
            return Ok(CmdScanEnable::InquiryScanDisabled_PageScanEnabled);
        }
        if bytes.next_if_eq::<u8>(&0x03) {
            return Ok(CmdScanEnable::InquiryScanEnabled_PageScanEnabled);
        }
        Err(PacketError::Unspecified(format!(
            "No matching variant found for {}",
            stringify!(CmdScanEnable)
        )))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            CmdScanEnable::NoScans => {
                bytes.pack::<u8>(&0x00)?;
            }
            CmdScanEnable::InquiryScanEnabled_PageScanDisabled => {
                bytes.pack::<u8>(&0x01)?;
            }
            CmdScanEnable::InquiryScanDisabled_PageScanEnabled => {
                bytes.pack::<u8>(&0x02)?;
            }
            CmdScanEnable::InquiryScanEnabled_PageScanEnabled => {
                bytes.pack::<u8>(&0x03)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u8> for CmdScanEnable {
    fn get_id(&self) -> u8 {
        match self {
            CmdScanEnable::NoScans => 0x00,
            CmdScanEnable::InquiryScanEnabled_PageScanDisabled => 0x01,
            CmdScanEnable::InquiryScanDisabled_PageScanEnabled => 0x02,
            CmdScanEnable::InquiryScanEnabled_PageScanEnabled => 0x03,
        }
    }
}
impl FromToPacket for LeConnectionComplete {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(LeConnectionComplete {
            status: bytes.unpack()?,
            connection_handle: bytes.unpack()?,
            role: bytes.unpack()?,
            peer_address_type: bytes.unpack()?,
            peer_address: bytes.unpack()?,
            connection_interval: bytes.unpack()?,
            peripheral_latency: bytes.unpack()?,
            supervision_timeout: bytes.unpack()?,
            central_clock_accuracy: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            LeConnectionComplete {
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
        };
        Ok(())
    }
}
impl FromToPacket for LeConnectionUpdateComplete {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(LeConnectionUpdateComplete {
            status: bytes.unpack()?,
            connection_handle: bytes.unpack()?,
            interval: bytes.unpack()?,
            latency: bytes.unpack()?,
            timeout: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            LeConnectionUpdateComplete {
                status,
                connection_handle,
                interval,
                latency,
                timeout,
            } => {
                bytes.pack(status)?;
                bytes.pack(connection_handle)?;
                bytes.pack(interval)?;
                bytes.pack(latency)?;
                bytes.pack(timeout)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for LeReadRemoteFeaturesPage0Complete {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(LeReadRemoteFeaturesPage0Complete {
            status: bytes.unpack()?,
            connection_handle: bytes.unpack()?,
            le_features: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            LeReadRemoteFeaturesPage0Complete {
                status,
                connection_handle,
                le_features,
            } => {
                bytes.pack(status)?;
                bytes.pack(connection_handle)?;
                bytes.pack(le_features)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for LeLongTermKeyRequest {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(LeLongTermKeyRequest {
            connection_handle: bytes.unpack()?,
            random_number: bytes.unpack()?,
            encrypted_diversifier: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            LeLongTermKeyRequest {
                connection_handle,
                random_number,
                encrypted_diversifier,
            } => {
                bytes.pack(connection_handle)?;
                bytes.pack(random_number)?;
                bytes.pack(encrypted_diversifier)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for LeDataLengthChange {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(LeDataLengthChange {
            connection_handle: bytes.unpack()?,
            max_tx_octets: bytes.unpack()?,
            max_tx_time: bytes.unpack()?,
            max_rx_octets: bytes.unpack()?,
            max_rx_time: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            LeDataLengthChange {
                connection_handle,
                max_tx_octets,
                max_tx_time,
                max_rx_octets,
                max_rx_time,
            } => {
                bytes.pack(connection_handle)?;
                bytes.pack(max_tx_octets)?;
                bytes.pack(max_tx_time)?;
                bytes.pack(max_rx_octets)?;
                bytes.pack(max_rx_time)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for LeReadLocalP256PublicKeyComplete {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(LeReadLocalP256PublicKeyComplete {
            status: bytes.unpack()?,
            public_key: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            LeReadLocalP256PublicKeyComplete { status, public_key } => {
                bytes.pack(status)?;
                bytes.pack(public_key)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for EvtDisconnectComplete {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(EvtDisconnectComplete {
            status: bytes.unpack()?,
            connection_handle: bytes.unpack()?,
            reason: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            EvtDisconnectComplete {
                status,
                connection_handle,
                reason,
            } => {
                bytes.pack(status)?;
                bytes.pack(connection_handle)?;
                bytes.pack(reason)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for EvtEncryptionChange {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(EvtEncryptionChange {
            status: bytes.unpack()?,
            connection_handle: bytes.unpack()?,
            encryption_enabled: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            EvtEncryptionChange {
                status,
                connection_handle,
                encryption_enabled,
            } => {
                bytes.pack(status)?;
                bytes.pack(connection_handle)?;
                bytes.pack(encryption_enabled)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for EvtNumberOfCompletedPackets {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(EvtNumberOfCompletedPackets {
            num_hci_command_packets: bytes.unpack()?,
            connection_handle: bytes.unpack()?,
            num_completed_packets: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            EvtNumberOfCompletedPackets {
                num_hci_command_packets,
                connection_handle,
                num_completed_packets,
            } => {
                bytes.pack(num_hci_command_packets)?;
                bytes.pack(connection_handle)?;
                bytes.pack(num_completed_packets)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for EvtCommandComplete {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(EvtCommandComplete {
            num_hci_command_packets: bytes.unpack()?,
            command_opcode: bytes.unpack()?,
            status: bytes.unpack()?,
            data: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            EvtCommandComplete {
                num_hci_command_packets,
                command_opcode,
                status,
                data,
            } => {
                bytes.pack(num_hci_command_packets)?;
                bytes.pack(command_opcode)?;
                bytes.pack(status)?;
                bytes.pack(data)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for EvtCommandStatus {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(EvtCommandStatus {
            status: bytes.unpack()?,
            num_hci_command_packets: bytes.unpack()?,
            command_opcode: bytes.unpack()?,
        })
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            EvtCommandStatus {
                status,
                num_hci_command_packets,
                command_opcode,
            } => {
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
        if bytes.next_if_eq::<u8>(&0b00) {
            return Ok(PacketBoundaryFlag::FirstNonFlushable);
        }
        if bytes.next_if_eq::<u8>(&0b01) {
            return Ok(PacketBoundaryFlag::Continuation);
        }
        if bytes.next_if_eq::<u8>(&0b10) {
            return Ok(PacketBoundaryFlag::FirstFlushable);
        }
        if bytes.next_if_eq::<u8>(&0b11) {
            return Ok(PacketBoundaryFlag::Deprecated);
        }
        Err(PacketError::Unspecified(format!(
            "No matching variant found for {}",
            stringify!(PacketBoundaryFlag)
        )))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            PacketBoundaryFlag::FirstNonFlushable => {
                bytes.pack::<u8>(&0b00)?;
            }
            PacketBoundaryFlag::Continuation => {
                bytes.pack::<u8>(&0b01)?;
            }
            PacketBoundaryFlag::FirstFlushable => {
                bytes.pack::<u8>(&0b10)?;
            }
            PacketBoundaryFlag::Deprecated => {
                bytes.pack::<u8>(&0b11)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u8> for PacketBoundaryFlag {
    fn get_id(&self) -> u8 {
        match self {
            PacketBoundaryFlag::FirstNonFlushable => 0b00,
            PacketBoundaryFlag::Continuation => 0b01,
            PacketBoundaryFlag::FirstFlushable => 0b10,
            PacketBoundaryFlag::Deprecated => 0b11,
        }
    }
}
impl FromToPacket for BroadcastFlag {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<u8>(&0b00) {
            return Ok(BroadcastFlag::PointToPoint);
        }
        if bytes.next_if_eq::<u8>(&0b01) {
            return Ok(BroadcastFlag::BdEdrBroadcast);
        }
        Err(PacketError::Unspecified(format!(
            "No matching variant found for {}",
            stringify!(BroadcastFlag)
        )))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            BroadcastFlag::PointToPoint => {
                bytes.pack::<u8>(&0b00)?;
            }
            BroadcastFlag::BdEdrBroadcast => {
                bytes.pack::<u8>(&0b01)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u8> for BroadcastFlag {
    fn get_id(&self) -> u8 {
        match self {
            BroadcastFlag::PointToPoint => 0b00,
            BroadcastFlag::BdEdrBroadcast => 0b01,
        }
    }
}
impl FromToPacket for HciStatus {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<u8>(&0x00) {
            return Ok(HciStatus::Success);
        }
        Ok(HciStatus::Failure(bytes.unpack()?))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            HciStatus::Success => {
                bytes.pack::<u8>(&0x00)?;
            }
            HciStatus::Failure(m0) => {
                bytes.pack(m0)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u8> for HciStatus {
    fn get_id(&self) -> u8 {
        match self {
            HciStatus::Success => 0x00,
            HciStatus::Failure(m0) => m0.clone(),
        }
    }
}
impl FromToPacket for Role {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<u8>(&0) {
            return Ok(Role::Central);
        }
        if bytes.next_if_eq::<u8>(&1) {
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
                bytes.pack::<u8>(&0)?;
            }
            Role::Peripheral => {
                bytes.pack::<u8>(&1)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u8> for Role {
    fn get_id(&self) -> u8 {
        match self {
            Role::Central => 0,
            Role::Peripheral => 1,
        }
    }
}
impl FromToPacket for AddressType {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<u8>(&0) {
            return Ok(AddressType::Public);
        }
        if bytes.next_if_eq::<u8>(&1) {
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
                bytes.pack::<u8>(&0)?;
            }
            AddressType::Random => {
                bytes.pack::<u8>(&1)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u8> for AddressType {
    fn get_id(&self) -> u8 {
        match self {
            AddressType::Public => 0,
            AddressType::Random => 1,
        }
    }
}
impl FromToPacket for ClockAccuracy {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<u8>(&0) {
            return Ok(ClockAccuracy::Ppm500);
        }
        if bytes.next_if_eq::<u8>(&1) {
            return Ok(ClockAccuracy::Ppm250);
        }
        if bytes.next_if_eq::<u8>(&2) {
            return Ok(ClockAccuracy::Ppm150);
        }
        if bytes.next_if_eq::<u8>(&3) {
            return Ok(ClockAccuracy::Ppm100);
        }
        if bytes.next_if_eq::<u8>(&4) {
            return Ok(ClockAccuracy::Ppm75);
        }
        if bytes.next_if_eq::<u8>(&5) {
            return Ok(ClockAccuracy::Ppm50);
        }
        if bytes.next_if_eq::<u8>(&6) {
            return Ok(ClockAccuracy::Ppm30);
        }
        if bytes.next_if_eq::<u8>(&7) {
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
                bytes.pack::<u8>(&0)?;
            }
            ClockAccuracy::Ppm250 => {
                bytes.pack::<u8>(&1)?;
            }
            ClockAccuracy::Ppm150 => {
                bytes.pack::<u8>(&2)?;
            }
            ClockAccuracy::Ppm100 => {
                bytes.pack::<u8>(&3)?;
            }
            ClockAccuracy::Ppm75 => {
                bytes.pack::<u8>(&4)?;
            }
            ClockAccuracy::Ppm50 => {
                bytes.pack::<u8>(&5)?;
            }
            ClockAccuracy::Ppm30 => {
                bytes.pack::<u8>(&6)?;
            }
            ClockAccuracy::Ppm20 => {
                bytes.pack::<u8>(&7)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u8> for ClockAccuracy {
    fn get_id(&self) -> u8 {
        match self {
            ClockAccuracy::Ppm500 => 0,
            ClockAccuracy::Ppm250 => 1,
            ClockAccuracy::Ppm150 => 2,
            ClockAccuracy::Ppm100 => 3,
            ClockAccuracy::Ppm75 => 4,
            ClockAccuracy::Ppm50 => 5,
            ClockAccuracy::Ppm30 => 6,
            ClockAccuracy::Ppm20 => 7,
        }
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
impl FromToPacket for BdAddr {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(BdAddr(bytes.unpack()?))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            BdAddr(m0) => {
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
            KeyDistributionFlags {
                enc_key,
                id_key,
                sign_key,
                link_key,
                _reserved,
            } => {
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
        if bytes.next_if_eq::<u8>(&0x00) {
            return Ok(IOCapability::DisplayOnly);
        }
        if bytes.next_if_eq::<u8>(&0x01) {
            return Ok(IOCapability::DisplayYesNo);
        }
        if bytes.next_if_eq::<u8>(&0x02) {
            return Ok(IOCapability::KeyboardOnly);
        }
        if bytes.next_if_eq::<u8>(&0x03) {
            return Ok(IOCapability::NoInputNoOutput);
        }
        if bytes.next_if_eq::<u8>(&0x04) {
            return Ok(IOCapability::KeyboardDisplay);
        }
        Err(PacketError::Unspecified(format!(
            "No matching variant found for {}",
            stringify!(IOCapability)
        )))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            IOCapability::DisplayOnly => {
                bytes.pack::<u8>(&0x00)?;
            }
            IOCapability::DisplayYesNo => {
                bytes.pack::<u8>(&0x01)?;
            }
            IOCapability::KeyboardOnly => {
                bytes.pack::<u8>(&0x02)?;
            }
            IOCapability::NoInputNoOutput => {
                bytes.pack::<u8>(&0x03)?;
            }
            IOCapability::KeyboardDisplay => {
                bytes.pack::<u8>(&0x04)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u8> for IOCapability {
    fn get_id(&self) -> u8 {
        match self {
            IOCapability::DisplayOnly => 0x00,
            IOCapability::DisplayYesNo => 0x01,
            IOCapability::KeyboardOnly => 0x02,
            IOCapability::NoInputNoOutput => 0x03,
            IOCapability::KeyboardDisplay => 0x04,
        }
    }
}
impl FromToPacket for OOBDataFlag {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<u8>(&0x00) {
            return Ok(OOBDataFlag::OobNotAvailable);
        }
        if bytes.next_if_eq::<u8>(&0x01) {
            return Ok(OOBDataFlag::OobAvailable);
        }
        Err(PacketError::Unspecified(format!(
            "No matching variant found for {}",
            stringify!(OOBDataFlag)
        )))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            OOBDataFlag::OobNotAvailable => {
                bytes.pack::<u8>(&0x00)?;
            }
            OOBDataFlag::OobAvailable => {
                bytes.pack::<u8>(&0x01)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u8> for OOBDataFlag {
    fn get_id(&self) -> u8 {
        match self {
            OOBDataFlag::OobNotAvailable => 0x00,
            OOBDataFlag::OobAvailable => 0x01,
        }
    }
}
impl FromToPacket for SmpPairingFailure {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<u8>(&0x01) {
            return Ok(SmpPairingFailure::PasskeyEntryFailed);
        }
        if bytes.next_if_eq::<u8>(&0x02) {
            return Ok(SmpPairingFailure::OobNotAvailable);
        }
        if bytes.next_if_eq::<u8>(&0x03) {
            return Ok(SmpPairingFailure::AuthenticationRequirements);
        }
        if bytes.next_if_eq::<u8>(&0x04) {
            return Ok(SmpPairingFailure::ConfirmValueFailed);
        }
        if bytes.next_if_eq::<u8>(&0x05) {
            return Ok(SmpPairingFailure::PairingNotSupported);
        }
        if bytes.next_if_eq::<u8>(&0x06) {
            return Ok(SmpPairingFailure::EncryptionKeySize);
        }
        if bytes.next_if_eq::<u8>(&0x07) {
            return Ok(SmpPairingFailure::CommandNotSupported);
        }
        if bytes.next_if_eq::<u8>(&0x08) {
            return Ok(SmpPairingFailure::UnspecifiedReason);
        }
        if bytes.next_if_eq::<u8>(&0x09) {
            return Ok(SmpPairingFailure::RepeatedAttempts);
        }
        if bytes.next_if_eq::<u8>(&0x0A) {
            return Ok(SmpPairingFailure::InvalidParameters);
        }
        if bytes.next_if_eq::<u8>(&0x0B) {
            return Ok(SmpPairingFailure::DhKeyCheckFailed);
        }
        if bytes.next_if_eq::<u8>(&0x0C) {
            return Ok(SmpPairingFailure::NumericComparisonFailed);
        }
        if bytes.next_if_eq::<u8>(&0x0D) {
            return Ok(SmpPairingFailure::BrEdrPairingInProgress);
        }
        if bytes.next_if_eq::<u8>(&0x0E) {
            return Ok(SmpPairingFailure::CrossTransportKeyDerivationGenerationNotAllowed);
        }
        if bytes.next_if_eq::<u8>(&0x0F) {
            return Ok(SmpPairingFailure::KeyRejected);
        }
        if bytes.next_if_eq::<u8>(&0x10) {
            return Ok(SmpPairingFailure::Busy);
        }
        Err(PacketError::Unspecified(format!(
            "No matching variant found for {}",
            stringify!(SmpPairingFailure)
        )))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            SmpPairingFailure::PasskeyEntryFailed => {
                bytes.pack::<u8>(&0x01)?;
            }
            SmpPairingFailure::OobNotAvailable => {
                bytes.pack::<u8>(&0x02)?;
            }
            SmpPairingFailure::AuthenticationRequirements => {
                bytes.pack::<u8>(&0x03)?;
            }
            SmpPairingFailure::ConfirmValueFailed => {
                bytes.pack::<u8>(&0x04)?;
            }
            SmpPairingFailure::PairingNotSupported => {
                bytes.pack::<u8>(&0x05)?;
            }
            SmpPairingFailure::EncryptionKeySize => {
                bytes.pack::<u8>(&0x06)?;
            }
            SmpPairingFailure::CommandNotSupported => {
                bytes.pack::<u8>(&0x07)?;
            }
            SmpPairingFailure::UnspecifiedReason => {
                bytes.pack::<u8>(&0x08)?;
            }
            SmpPairingFailure::RepeatedAttempts => {
                bytes.pack::<u8>(&0x09)?;
            }
            SmpPairingFailure::InvalidParameters => {
                bytes.pack::<u8>(&0x0A)?;
            }
            SmpPairingFailure::DhKeyCheckFailed => {
                bytes.pack::<u8>(&0x0B)?;
            }
            SmpPairingFailure::NumericComparisonFailed => {
                bytes.pack::<u8>(&0x0C)?;
            }
            SmpPairingFailure::BrEdrPairingInProgress => {
                bytes.pack::<u8>(&0x0D)?;
            }
            SmpPairingFailure::CrossTransportKeyDerivationGenerationNotAllowed => {
                bytes.pack::<u8>(&0x0E)?;
            }
            SmpPairingFailure::KeyRejected => {
                bytes.pack::<u8>(&0x0F)?;
            }
            SmpPairingFailure::Busy => {
                bytes.pack::<u8>(&0x10)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u8> for SmpPairingFailure {
    fn get_id(&self) -> u8 {
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
        }
    }
}
