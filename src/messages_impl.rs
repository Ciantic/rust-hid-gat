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
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(H4Packet)),
            ),
        )
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
            HciAcl { connection_handle, pb, bc, msg } => {
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
            return Ok(AttPdu::AttExchangeMtuRequest {
                mtu: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x03) {
            return Ok(AttPdu::AttExchangeMtuResponse {
                mtu: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x04) {
            return Ok(AttPdu::AttFindInformationRequest {
                starting_handle: bytes.unpack()?,
                ending_handle: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x05) {
            return Ok(AttPdu::AttFindInformationResponse {
                format: bytes.unpack()?,
                information: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x06) {
            return Ok(AttPdu::AttFindByTypeValueRequest {
                starting_handle: bytes.unpack()?,
                ending_handle: bytes.unpack()?,
                uuid: bytes.unpack()?,
                value: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x07) {
            return Ok(AttPdu::AttFindByTypeValueResponse {
                handles_information: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x08) {
            return Ok(AttPdu::AttReadByTypeRequest {
                starting_handle: bytes.unpack()?,
                ending_handle: bytes.unpack()?,
                uuid: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x09) {
            return Ok(AttPdu::AttReadByTypeResponse {
                pair_length: bytes.unpack()?,
                values: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x0A) {
            return Ok(AttPdu::AttReadRequest {
                handle: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x0B) {
            return Ok(AttPdu::AttReadResponse {
                value: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x18) {
            return Ok(AttPdu::AttExecuteWriteRequest {
                flags: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x19) {
            return Ok(AttPdu::AttExecuteWriteResponse);
        }
        if bytes.next_if_eq::<u8>(&0x1B) {
            return Ok(AttPdu::AttHandleValueNotification {
                handle: bytes.unpack()?,
                value: bytes.unpack()?,
            });
        }
        Ok(AttPdu::AttUnknown(bytes.unpack()?, bytes.unpack()?))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            AttPdu::AttExchangeMtuRequest { mtu } => {
                bytes.pack::<u8>(&0x02)?;
                bytes.pack(mtu)?;
            }
            AttPdu::AttExchangeMtuResponse { mtu } => {
                bytes.pack::<u8>(&0x03)?;
                bytes.pack(mtu)?;
            }
            AttPdu::AttFindInformationRequest { starting_handle, ending_handle } => {
                bytes.pack::<u8>(&0x04)?;
                bytes.pack(starting_handle)?;
                bytes.pack(ending_handle)?;
            }
            AttPdu::AttFindInformationResponse { format, information } => {
                bytes.pack::<u8>(&0x05)?;
                bytes.pack(format)?;
                bytes.pack(information)?;
            }
            AttPdu::AttFindByTypeValueRequest {
                starting_handle,
                ending_handle,
                uuid,
                value,
            } => {
                bytes.pack::<u8>(&0x06)?;
                bytes.pack(starting_handle)?;
                bytes.pack(ending_handle)?;
                bytes.pack(uuid)?;
                bytes.pack(value)?;
            }
            AttPdu::AttFindByTypeValueResponse { handles_information } => {
                bytes.pack::<u8>(&0x07)?;
                bytes.pack(handles_information)?;
            }
            AttPdu::AttReadByTypeRequest { starting_handle, ending_handle, uuid } => {
                bytes.pack::<u8>(&0x08)?;
                bytes.pack(starting_handle)?;
                bytes.pack(ending_handle)?;
                bytes.pack(uuid)?;
            }
            AttPdu::AttReadByTypeResponse { pair_length, values } => {
                bytes.pack::<u8>(&0x09)?;
                bytes.pack(pair_length)?;
                bytes.pack(values)?;
            }
            AttPdu::AttReadRequest { handle } => {
                bytes.pack::<u8>(&0x0A)?;
                bytes.pack(handle)?;
            }
            AttPdu::AttReadResponse { value } => {
                bytes.pack::<u8>(&0x0B)?;
                bytes.pack(value)?;
            }
            AttPdu::AttExecuteWriteRequest { flags } => {
                bytes.pack::<u8>(&0x18)?;
                bytes.pack(flags)?;
            }
            AttPdu::AttExecuteWriteResponse => {
                bytes.pack::<u8>(&0x19)?;
            }
            AttPdu::AttHandleValueNotification { handle, value } => {
                bytes.pack::<u8>(&0x1B)?;
                bytes.pack(handle)?;
                bytes.pack(value)?;
            }
            AttPdu::AttUnknown(m0, m1) => {
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
            AttPdu::AttExchangeMtuRequest { mtu } => 0x02,
            AttPdu::AttExchangeMtuResponse { mtu } => 0x03,
            AttPdu::AttFindInformationRequest { starting_handle, ending_handle } => 0x04,
            AttPdu::AttFindInformationResponse { format, information } => 0x05,
            AttPdu::AttFindByTypeValueRequest {
                starting_handle,
                ending_handle,
                uuid,
                value,
            } => 0x06,
            AttPdu::AttFindByTypeValueResponse { handles_information } => 0x07,
            AttPdu::AttReadByTypeRequest { starting_handle, ending_handle, uuid } => 0x08,
            AttPdu::AttReadByTypeResponse { pair_length, values } => 0x09,
            AttPdu::AttReadRequest { handle } => 0x0A,
            AttPdu::AttReadResponse { value } => 0x0B,
            AttPdu::AttExecuteWriteRequest { flags } => 0x18,
            AttPdu::AttExecuteWriteResponse => 0x19,
            AttPdu::AttHandleValueNotification { handle, value } => 0x1B,
            AttPdu::AttUnknown(m0, m1) => m0.clone(),
        }
    }
}
impl FromToPacket for SmpPdu {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<u8>(&0x01) {
            return Ok(SmpPdu::SmpPairingRequest {
                io_capability: bytes.unpack()?,
                oob_data_flag: bytes.unpack()?,
                authentication_requirements: bytes.unpack()?,
                max_encryption_key_size: bytes.unpack()?,
                initiator_key_distribution: bytes.unpack()?,
                responder_key_distribution: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x02) {
            return Ok(SmpPdu::SmpPairingResponse {
                io_capability: bytes.unpack()?,
                oob_data_flag: bytes.unpack()?,
                authentication_requirements: bytes.unpack()?,
                max_encryption_key_size: bytes.unpack()?,
                initiator_key_distribution: bytes.unpack()?,
                responder_key_distribution: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x03) {
            return Ok(SmpPdu::SmpPairingConfirmation {
                confirm_value: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x04) {
            return Ok(SmpPdu::SmpPairingRandom {
                random_value: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x05) {
            return Ok(SmpPdu::SmpPairingFailed(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x06) {
            return Ok(SmpPdu::SmpEncryptionInformation {
                long_term_key: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x07) {
            return Ok(SmpPdu::SmpCentralIdentification {
                encrypted_diversifier: bytes.unpack()?,
                random_number: bytes.unpack()?,
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
            SmpPdu::SmpPairingRequest {
                io_capability,
                oob_data_flag,
                authentication_requirements,
                max_encryption_key_size,
                initiator_key_distribution,
                responder_key_distribution,
            } => {
                bytes.pack::<u8>(&0x01)?;
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
                bytes.pack::<u8>(&0x02)?;
                bytes.pack(io_capability)?;
                bytes.pack(oob_data_flag)?;
                bytes.pack(authentication_requirements)?;
                bytes.pack(max_encryption_key_size)?;
                bytes.pack(initiator_key_distribution)?;
                bytes.pack(responder_key_distribution)?;
            }
            SmpPdu::SmpPairingConfirmation { confirm_value } => {
                bytes.pack::<u8>(&0x03)?;
                bytes.pack(confirm_value)?;
            }
            SmpPdu::SmpPairingRandom { random_value } => {
                bytes.pack::<u8>(&0x04)?;
                bytes.pack(random_value)?;
            }
            SmpPdu::SmpPairingFailed(m0) => {
                bytes.pack::<u8>(&0x05)?;
                bytes.pack(m0)?;
            }
            SmpPdu::SmpEncryptionInformation { long_term_key } => {
                bytes.pack::<u8>(&0x06)?;
                bytes.pack(long_term_key)?;
            }
            SmpPdu::SmpCentralIdentification {
                encrypted_diversifier,
                random_number,
            } => {
                bytes.pack::<u8>(&0x07)?;
                bytes.pack(encrypted_diversifier)?;
                bytes.pack(random_number)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u8> for SmpPdu {
    fn get_id(&self) -> u8 {
        match self {
            SmpPdu::SmpPairingRequest {
                io_capability,
                oob_data_flag,
                authentication_requirements,
                max_encryption_key_size,
                initiator_key_distribution,
                responder_key_distribution,
            } => 0x01,
            SmpPdu::SmpPairingResponse {
                io_capability,
                oob_data_flag,
                authentication_requirements,
                max_encryption_key_size,
                initiator_key_distribution,
                responder_key_distribution,
            } => 0x02,
            SmpPdu::SmpPairingConfirmation { confirm_value } => 0x03,
            SmpPdu::SmpPairingRandom { random_value } => 0x04,
            SmpPdu::SmpPairingFailed(m0) => 0x05,
            SmpPdu::SmpEncryptionInformation { long_term_key } => 0x06,
            SmpPdu::SmpCentralIdentification { encrypted_diversifier, random_number } => {
                0x07
            }
        }
    }
}
impl FromToPacket for HciCommand {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0006, 0x01)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::Disconnect {
                connection_handle: bytes.unpack()?,
                reason: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0003, 0x03)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::Reset);
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0001, 0x03)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::SetEventMask {
                event_mask: bytes.unpack()?,
            });
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
            return Ok(HciCommand::ReadLocalName {
                status: bytes.unpack()?,
                name: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0001, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeSetEventMask {
                event_mask: bytes.unpack()?,
            });
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
            return Ok(HciCommand::LeSetAdvertisingParameters {
                advertising_interval_min: bytes.unpack()?,
                advertising_interval_max: bytes.unpack()?,
                advertising_type: bytes.unpack()?,
                own_address_type: bytes.unpack()?,
                peer_address_type: bytes.unpack()?,
                peer_address: bytes.unpack()?,
                advertising_channel_map: bytes.unpack()?,
                advertising_filter_policy: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x0008, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeSetAdvertisingData {
                advertising_data_length: bytes.unpack()?,
                advertising_data: bytes.unpack()?,
            });
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
            return Ok(HciCommand::LeSetDataLength {
                connection_handle: bytes.unpack()?,
                tx_octets: bytes.unpack()?,
                tx_time: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<OpCode>(&OpCode(0x001A, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeLongTermKeyRequestReply {
                connection_handle: bytes.unpack()?,
                long_term_key: bytes.unpack()?,
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
            HciCommand::Disconnect { connection_handle, reason } => {
                bytes.pack::<OpCode>(&OpCode(0x0006, 0x01))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(connection_handle)?;
                bytes.pack(reason)?;
            }
            HciCommand::Reset => {
                bytes.pack::<OpCode>(&OpCode(0x0003, 0x03))?;
                bytes.pack_length::<u8>()?;
            }
            HciCommand::SetEventMask { event_mask } => {
                bytes.pack::<OpCode>(&OpCode(0x0001, 0x03))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(event_mask)?;
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
            HciCommand::ReadLocalName { status, name } => {
                bytes.pack::<OpCode>(&OpCode(0x0014, 0x03))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(status)?;
                bytes.pack(name)?;
            }
            HciCommand::LeSetEventMask { event_mask } => {
                bytes.pack::<OpCode>(&OpCode(0x0001, 0x08))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(event_mask)?;
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
            HciCommand::LeSetAdvertisingParameters {
                advertising_interval_min,
                advertising_interval_max,
                advertising_type,
                own_address_type,
                peer_address_type,
                peer_address,
                advertising_channel_map,
                advertising_filter_policy,
            } => {
                bytes.pack::<OpCode>(&OpCode(0x0006, 0x08))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(advertising_interval_min)?;
                bytes.pack(advertising_interval_max)?;
                bytes.pack(advertising_type)?;
                bytes.pack(own_address_type)?;
                bytes.pack(peer_address_type)?;
                bytes.pack(peer_address)?;
                bytes.pack(advertising_channel_map)?;
                bytes.pack(advertising_filter_policy)?;
            }
            HciCommand::LeSetAdvertisingData {
                advertising_data_length,
                advertising_data,
            } => {
                bytes.pack::<OpCode>(&OpCode(0x0008, 0x08))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(advertising_data_length)?;
                bytes.pack(advertising_data)?;
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
            HciCommand::LeSetDataLength { connection_handle, tx_octets, tx_time } => {
                bytes.pack::<OpCode>(&OpCode(0x0022, 0x08))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(connection_handle)?;
                bytes.pack(tx_octets)?;
                bytes.pack(tx_time)?;
            }
            HciCommand::LeLongTermKeyRequestReply {
                connection_handle,
                long_term_key,
            } => {
                bytes.pack::<OpCode>(&OpCode(0x001A, 0x08))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(connection_handle)?;
                bytes.pack(long_term_key)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<OpCode> for HciCommand {
    fn get_id(&self) -> OpCode {
        match self {
            HciCommand::Disconnect { connection_handle, reason } => OpCode(0x0006, 0x01),
            HciCommand::Reset => OpCode(0x0003, 0x03),
            HciCommand::SetEventMask { event_mask } => OpCode(0x0001, 0x03),
            HciCommand::ReadLocalSupportedCommands => OpCode(0x0002, 0x04),
            HciCommand::ReadBdAddr => OpCode(0x0009, 0x04),
            HciCommand::WriteScanEnable(m0) => OpCode(0x001a, 0x03),
            HciCommand::WriteConnectionAcceptTimeout(m0) => OpCode(0x0016, 0x03),
            HciCommand::WritePageTimeout(m0) => OpCode(0x0018, 0x03),
            HciCommand::WriteLocalName(m0) => OpCode(0x0013, 0x03),
            HciCommand::ReadLocalName { status, name } => OpCode(0x0014, 0x03),
            HciCommand::LeSetEventMask { event_mask } => OpCode(0x0001, 0x08),
            HciCommand::LeReadBufferSize => OpCode(0x0002, 0x08),
            HciCommand::LeSetRandomAddress(m0) => OpCode(0x0005, 0x08),
            HciCommand::LeSetAdvertisingParameters {
                advertising_interval_min,
                advertising_interval_max,
                advertising_type,
                own_address_type,
                peer_address_type,
                peer_address,
                advertising_channel_map,
                advertising_filter_policy,
            } => OpCode(0x0006, 0x08),
            HciCommand::LeSetAdvertisingData {
                advertising_data_length,
                advertising_data,
            } => OpCode(0x0008, 0x08),
            HciCommand::LeReadLocalP256PublicKey => OpCode(0x0025, 0x08),
            HciCommand::LeSetAdvertisingEnable(m0) => OpCode(0x000A, 0x08),
            HciCommand::LeSetDataLength { connection_handle, tx_octets, tx_time } => {
                OpCode(0x0022, 0x08)
            }
            HciCommand::LeLongTermKeyRequestReply {
                connection_handle,
                long_term_key,
            } => OpCode(0x001A, 0x08),
        }
    }
}
impl FromToPacket for OpCode {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(OpCode(bytes.set_bits(10).unpack()?, bytes.set_bits(6).unpack()?))
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
impl FromToPacket for ScanEnable {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<u8>(&0x00) {
            return Ok(ScanEnable::NoScans);
        }
        if bytes.next_if_eq::<u8>(&0x01) {
            return Ok(ScanEnable::InquiryScanEnabled_PageScanDisabled);
        }
        if bytes.next_if_eq::<u8>(&0x02) {
            return Ok(ScanEnable::InquiryScanDisabled_PageScanEnabled);
        }
        if bytes.next_if_eq::<u8>(&0x03) {
            return Ok(ScanEnable::InquiryScanEnabled_PageScanEnabled);
        }
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(ScanEnable)),
            ),
        )
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            ScanEnable::NoScans => {
                bytes.pack::<u8>(&0x00)?;
            }
            ScanEnable::InquiryScanEnabled_PageScanDisabled => {
                bytes.pack::<u8>(&0x01)?;
            }
            ScanEnable::InquiryScanDisabled_PageScanEnabled => {
                bytes.pack::<u8>(&0x02)?;
            }
            ScanEnable::InquiryScanEnabled_PageScanEnabled => {
                bytes.pack::<u8>(&0x03)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u8> for ScanEnable {
    fn get_id(&self) -> u8 {
        match self {
            ScanEnable::NoScans => 0x00,
            ScanEnable::InquiryScanEnabled_PageScanDisabled => 0x01,
            ScanEnable::InquiryScanDisabled_PageScanEnabled => 0x02,
            ScanEnable::InquiryScanEnabled_PageScanEnabled => 0x03,
        }
    }
}
impl FromToPacket for LeMeta {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<u8>(&0x01) {
            return Ok(LeMeta::LeConnectionComplete {
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
        if bytes.next_if_eq::<u8>(&0x02) {
            return Ok(LeMeta::LeAdvertisingReport(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x03) {
            return Ok(LeMeta::LeConnectionUpdateComplete {
                status: bytes.unpack()?,
                connection_handle: bytes.unpack()?,
                interval: bytes.unpack()?,
                latency: bytes.unpack()?,
                timeout: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x04) {
            return Ok(LeMeta::LeReadRemoteFeaturesPage0Complete {
                status: bytes.unpack()?,
                connection_handle: bytes.unpack()?,
                le_features: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x05) {
            return Ok(LeMeta::LeLongTermKeyRequest {
                connection_handle: bytes.unpack()?,
                random_number: bytes.unpack()?,
                encrypted_diversifier: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x07) {
            return Ok(LeMeta::LeDataLengthChange {
                connection_handle: bytes.unpack()?,
                max_tx_octets: bytes.unpack()?,
                max_tx_time: bytes.unpack()?,
                max_rx_octets: bytes.unpack()?,
                max_rx_time: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x08) {
            return Ok(LeMeta::LeReadLocalP256PublicKeyComplete {
                status: bytes.unpack()?,
                public_key: bytes.unpack()?,
            });
        }
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(LeMeta)),
            ),
        )
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            LeMeta::LeConnectionComplete {
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
                bytes.pack::<u8>(&0x01)?;
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
            LeMeta::LeAdvertisingReport(m0) => {
                bytes.pack::<u8>(&0x02)?;
                bytes.pack(m0)?;
            }
            LeMeta::LeConnectionUpdateComplete {
                status,
                connection_handle,
                interval,
                latency,
                timeout,
            } => {
                bytes.pack::<u8>(&0x03)?;
                bytes.pack(status)?;
                bytes.pack(connection_handle)?;
                bytes.pack(interval)?;
                bytes.pack(latency)?;
                bytes.pack(timeout)?;
            }
            LeMeta::LeReadRemoteFeaturesPage0Complete {
                status,
                connection_handle,
                le_features,
            } => {
                bytes.pack::<u8>(&0x04)?;
                bytes.pack(status)?;
                bytes.pack(connection_handle)?;
                bytes.pack(le_features)?;
            }
            LeMeta::LeLongTermKeyRequest {
                connection_handle,
                random_number,
                encrypted_diversifier,
            } => {
                bytes.pack::<u8>(&0x05)?;
                bytes.pack(connection_handle)?;
                bytes.pack(random_number)?;
                bytes.pack(encrypted_diversifier)?;
            }
            LeMeta::LeDataLengthChange {
                connection_handle,
                max_tx_octets,
                max_tx_time,
                max_rx_octets,
                max_rx_time,
            } => {
                bytes.pack::<u8>(&0x07)?;
                bytes.pack(connection_handle)?;
                bytes.pack(max_tx_octets)?;
                bytes.pack(max_tx_time)?;
                bytes.pack(max_rx_octets)?;
                bytes.pack(max_rx_time)?;
            }
            LeMeta::LeReadLocalP256PublicKeyComplete { status, public_key } => {
                bytes.pack::<u8>(&0x08)?;
                bytes.pack(status)?;
                bytes.pack(public_key)?;
            }
        };
        Ok(())
    }
}
impl PacketIdentifier<u8> for LeMeta {
    fn get_id(&self) -> u8 {
        match self {
            LeMeta::LeConnectionComplete {
                status,
                connection_handle,
                role,
                peer_address_type,
                peer_address,
                connection_interval,
                peripheral_latency,
                supervision_timeout,
                central_clock_accuracy,
            } => 0x01,
            LeMeta::LeAdvertisingReport(m0) => 0x02,
            LeMeta::LeConnectionUpdateComplete {
                status,
                connection_handle,
                interval,
                latency,
                timeout,
            } => 0x03,
            LeMeta::LeReadRemoteFeaturesPage0Complete {
                status,
                connection_handle,
                le_features,
            } => 0x04,
            LeMeta::LeLongTermKeyRequest {
                connection_handle,
                random_number,
                encrypted_diversifier,
            } => 0x05,
            LeMeta::LeDataLengthChange {
                connection_handle,
                max_tx_octets,
                max_tx_time,
                max_rx_octets,
                max_rx_time,
            } => 0x07,
            LeMeta::LeReadLocalP256PublicKeyComplete { status, public_key } => 0x08,
        }
    }
}
impl FromToPacket for HciEvent {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq::<u8>(&0x05) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEvent::DisconnectComplete {
                status: bytes.unpack()?,
                connection_handle: bytes.unpack()?,
                reason: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x08) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEvent::EncryptionChange {
                status: bytes.unpack()?,
                connection_handle: bytes.unpack()?,
                encryption_enabled: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x13) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEvent::NumberOfCompletedPackets {
                num_hci_command_packets: bytes.unpack()?,
                connection_handle: bytes.unpack()?,
                num_completed_packets: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x3e) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEvent::LeMeta(bytes.unpack()?));
        }
        if bytes.next_if_eq::<u8>(&0x0E) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEvent::CommandComplete {
                num_hci_command_packets: bytes.unpack()?,
                command_opcode: bytes.unpack()?,
                status: bytes.unpack()?,
                data: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0x0F) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEvent::CommandStatus {
                status: bytes.unpack()?,
                num_hci_command_packets: bytes.unpack()?,
                command_opcode: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq::<u8>(&0xFF) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEvent::VendorSpecific(bytes.unpack()?));
        }
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(HciEvent)),
            ),
        )
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            HciEvent::DisconnectComplete { status, connection_handle, reason } => {
                bytes.pack::<u8>(&0x05)?;
                bytes.pack_length::<u8>()?;
                bytes.pack(status)?;
                bytes.pack(connection_handle)?;
                bytes.pack(reason)?;
            }
            HciEvent::EncryptionChange {
                status,
                connection_handle,
                encryption_enabled,
            } => {
                bytes.pack::<u8>(&0x08)?;
                bytes.pack_length::<u8>()?;
                bytes.pack(status)?;
                bytes.pack(connection_handle)?;
                bytes.pack(encryption_enabled)?;
            }
            HciEvent::NumberOfCompletedPackets {
                num_hci_command_packets,
                connection_handle,
                num_completed_packets,
            } => {
                bytes.pack::<u8>(&0x13)?;
                bytes.pack_length::<u8>()?;
                bytes.pack(num_hci_command_packets)?;
                bytes.pack(connection_handle)?;
                bytes.pack(num_completed_packets)?;
            }
            HciEvent::LeMeta(m0) => {
                bytes.pack::<u8>(&0x3e)?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciEvent::CommandComplete {
                num_hci_command_packets,
                command_opcode,
                status,
                data,
            } => {
                bytes.pack::<u8>(&0x0E)?;
                bytes.pack_length::<u8>()?;
                bytes.pack(num_hci_command_packets)?;
                bytes.pack(command_opcode)?;
                bytes.pack(status)?;
                bytes.pack(data)?;
            }
            HciEvent::CommandStatus {
                status,
                num_hci_command_packets,
                command_opcode,
            } => {
                bytes.pack::<u8>(&0x0F)?;
                bytes.pack_length::<u8>()?;
                bytes.pack(status)?;
                bytes.pack(num_hci_command_packets)?;
                bytes.pack(command_opcode)?;
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
            HciEvent::DisconnectComplete { status, connection_handle, reason } => 0x05,
            HciEvent::EncryptionChange {
                status,
                connection_handle,
                encryption_enabled,
            } => 0x08,
            HciEvent::NumberOfCompletedPackets {
                num_hci_command_packets,
                connection_handle,
                num_completed_packets,
            } => 0x13,
            HciEvent::LeMeta(m0) => 0x3e,
            HciEvent::CommandComplete {
                num_hci_command_packets,
                command_opcode,
                status,
                data,
            } => 0x0E,
            HciEvent::CommandStatus {
                status,
                num_hci_command_packets,
                command_opcode,
            } => 0x0F,
            HciEvent::VendorSpecific(m0) => 0xFF,
        }
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
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(BroadcastFlag)),
            ),
        )
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
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(Role)),
            ),
        )
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
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(AddressType)),
            ),
        )
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
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(ClockAccuracy)),
            ),
        )
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
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(IOCapability)),
            ),
        )
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
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(OOBDataFlag)),
            ),
        )
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
            return Ok(
                SmpPairingFailure::CrossTransportKeyDerivationGenerationNotAllowed,
            );
        }
        if bytes.next_if_eq::<u8>(&0x0F) {
            return Ok(SmpPairingFailure::KeyRejected);
        }
        if bytes.next_if_eq::<u8>(&0x10) {
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
