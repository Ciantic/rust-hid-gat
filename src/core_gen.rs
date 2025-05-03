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
                bytes.pack(&[0x01])?;
                bytes.pack(m0)?;
            }
            H4Packet::HciEvent(m0) => {
                bytes.pack(&[0x04])?;
                bytes.pack(m0)?;
            }
            H4Packet::HciAcl { connection_handle, pb, bc, msg } => {
                bytes.pack(&[0x02])?;
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
        if bytes.next_if_eq(&[0x06, 0x00]) {
            return Ok(L2CapMessage::Smp(bytes.unpack()?));
        }
        if bytes.next_if_eq(&[0x04, 0x00]) {
            return Ok(L2CapMessage::Att(bytes.unpack()?));
        }
        Ok(L2CapMessage::Unknown(bytes.unpack()?, bytes.unpack()?))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        bytes.pack_length_with_offset::<u16>(-2)?;
        match self {
            L2CapMessage::Smp(m0) => {
                bytes.pack(&[0x06, 0x00])?;
                bytes.pack(m0)?;
            }
            L2CapMessage::Att(m0) => {
                bytes.pack(&[0x04, 0x00])?;
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
impl FromToPacket for AttPdu {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0x02]) {
            return Ok(AttPdu::AttExchangeMtuRequest {
                mtu: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x03]) {
            return Ok(AttPdu::AttExchangeMtuResponse {
                mtu: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x06]) {
            return Ok(AttPdu::AttFindByTypeValueRequest {
                starting_handle: bytes.unpack()?,
                ending_handle: bytes.unpack()?,
                uuid: bytes.unpack()?,
                value: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x07]) {
            return Ok(AttPdu::AttFindByTypeValueResponse {
                handles_information: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x18]) {
            return Ok(AttPdu::AttExecuteWriteRequest {
                flags: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x19]) {
            return Ok(AttPdu::AttExecuteWriteResponse);
        }
        if bytes.next_if_eq(&[0x1B]) {
            return Ok(AttPdu::AttHandleValueNotification {
                handle: bytes.unpack()?,
                value: bytes.unpack()?,
            });
        }
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(AttPdu)),
            ),
        )
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            AttPdu::AttExchangeMtuRequest { mtu } => {
                bytes.pack(&[0x02])?;
                bytes.pack(mtu)?;
            }
            AttPdu::AttExchangeMtuResponse { mtu } => {
                bytes.pack(&[0x03])?;
                bytes.pack(mtu)?;
            }
            AttPdu::AttFindByTypeValueRequest {
                starting_handle,
                ending_handle,
                uuid,
                value,
            } => {
                bytes.pack(&[0x06])?;
                bytes.pack(starting_handle)?;
                bytes.pack(ending_handle)?;
                bytes.pack(uuid)?;
                bytes.pack(value)?;
            }
            AttPdu::AttFindByTypeValueResponse { handles_information } => {
                bytes.pack(&[0x07])?;
                bytes.pack(handles_information)?;
            }
            AttPdu::AttExecuteWriteRequest { flags } => {
                bytes.pack(&[0x18])?;
                bytes.pack(flags)?;
            }
            AttPdu::AttExecuteWriteResponse => {
                bytes.pack(&[0x19])?;
            }
            AttPdu::AttHandleValueNotification { handle, value } => {
                bytes.pack(&[0x1B])?;
                bytes.pack(handle)?;
                bytes.pack(value)?;
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
                bytes.pack(&[0x03])?;
                bytes.pack(confirm_value)?;
            }
            SmpPdu::SmpPairingRandom { random_value } => {
                bytes.pack(&[0x04])?;
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
                bytes.pack(&[0x01])?;
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
                bytes.pack(&[0x02])?;
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
        if bytes.next_if_eq(&OpCode(0x0006, 0x01)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::Disconnect {
                connection_handle: bytes.unpack()?,
                reason: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&OpCode(0x0003, 0x03)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::Reset);
        }
        if bytes.next_if_eq(&OpCode(0x0001, 0x03)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::SetEventMask {
                event_mask: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&OpCode(0x0002, 0x04)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::ReadLocalSupportedCommands);
        }
        if bytes.next_if_eq(&OpCode(0x0009, 0x04)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::ReadBdAddr);
        }
        if bytes.next_if_eq(&OpCode(0x001a, 0x03)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::WriteScanEnable(bytes.unpack()?));
        }
        if bytes.next_if_eq(&OpCode(0x0016, 0x03)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::WriteConnectionAcceptTimeout(bytes.unpack()?));
        }
        if bytes.next_if_eq(&OpCode(0x0018, 0x03)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::WritePageTimeout(bytes.unpack()?));
        }
        if bytes.next_if_eq(&OpCode(0x0013, 0x03)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::WriteLocalName(bytes.unpack()?));
        }
        if bytes.next_if_eq(&OpCode(0x0014, 0x03)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::ReadLocalName {
                status: bytes.unpack()?,
                name: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&OpCode(0x0001, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeSetEventMask {
                event_mask: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&OpCode(0x0002, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeReadBufferSize);
        }
        if bytes.next_if_eq(&OpCode(0x0005, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeSetRandomAddress(bytes.unpack()?));
        }
        if bytes.next_if_eq(&OpCode(0x0006, 0x08)) {
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
        if bytes.next_if_eq(&OpCode(0x0008, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeSetAdvertisingData {
                advertising_data_length: bytes.unpack()?,
                advertising_data: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&OpCode(0x0025, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeReadLocalP256PublicKey);
        }
        if bytes.next_if_eq(&OpCode(0x000A, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeSetAdvertisingEnable(bytes.unpack()?));
        }
        if bytes.next_if_eq(&OpCode(0x0022, 0x08)) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciCommand::LeSetDataLength {
                connection_handle: bytes.unpack()?,
                tx_octets: bytes.unpack()?,
                tx_time: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&OpCode(0x001A, 0x08)) {
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
                bytes.pack(&OpCode(0x0006, 0x01))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(connection_handle)?;
                bytes.pack(reason)?;
            }
            HciCommand::Reset => {
                bytes.pack(&OpCode(0x0003, 0x03))?;
                bytes.pack_length::<u8>()?;
            }
            HciCommand::SetEventMask { event_mask } => {
                bytes.pack(&OpCode(0x0001, 0x03))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(event_mask)?;
            }
            HciCommand::ReadLocalSupportedCommands => {
                bytes.pack(&OpCode(0x0002, 0x04))?;
                bytes.pack_length::<u8>()?;
            }
            HciCommand::ReadBdAddr => {
                bytes.pack(&OpCode(0x0009, 0x04))?;
                bytes.pack_length::<u8>()?;
            }
            HciCommand::WriteScanEnable(m0) => {
                bytes.pack(&OpCode(0x001a, 0x03))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciCommand::WriteConnectionAcceptTimeout(m0) => {
                bytes.pack(&OpCode(0x0016, 0x03))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciCommand::WritePageTimeout(m0) => {
                bytes.pack(&OpCode(0x0018, 0x03))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciCommand::WriteLocalName(m0) => {
                bytes.pack(&OpCode(0x0013, 0x03))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciCommand::ReadLocalName { status, name } => {
                bytes.pack(&OpCode(0x0014, 0x03))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(status)?;
                bytes.pack(name)?;
            }
            HciCommand::LeSetEventMask { event_mask } => {
                bytes.pack(&OpCode(0x0001, 0x08))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(event_mask)?;
            }
            HciCommand::LeReadBufferSize => {
                bytes.pack(&OpCode(0x0002, 0x08))?;
                bytes.pack_length::<u8>()?;
            }
            HciCommand::LeSetRandomAddress(m0) => {
                bytes.pack(&OpCode(0x0005, 0x08))?;
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
                bytes.pack(&OpCode(0x0006, 0x08))?;
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
                bytes.pack(&OpCode(0x0008, 0x08))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(advertising_data_length)?;
                bytes.pack(advertising_data)?;
            }
            HciCommand::LeReadLocalP256PublicKey => {
                bytes.pack(&OpCode(0x0025, 0x08))?;
                bytes.pack_length::<u8>()?;
            }
            HciCommand::LeSetAdvertisingEnable(m0) => {
                bytes.pack(&OpCode(0x000A, 0x08))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciCommand::LeSetDataLength { connection_handle, tx_octets, tx_time } => {
                bytes.pack(&OpCode(0x0022, 0x08))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(connection_handle)?;
                bytes.pack(tx_octets)?;
                bytes.pack(tx_time)?;
            }
            HciCommand::LeLongTermKeyRequestReply {
                connection_handle,
                long_term_key,
            } => {
                bytes.pack(&OpCode(0x001A, 0x08))?;
                bytes.pack_length::<u8>()?;
                bytes.pack(connection_handle)?;
                bytes.pack(long_term_key)?;
            }
        };
        Ok(())
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
        if bytes.next_if_eq(&[0x00]) {
            return Ok(ScanEnable::NoScans);
        }
        if bytes.next_if_eq(&[0x01]) {
            return Ok(ScanEnable::InquiryScanEnabled_PageScanDisabled);
        }
        if bytes.next_if_eq(&[0x02]) {
            return Ok(ScanEnable::InquiryScanDisabled_PageScanEnabled);
        }
        if bytes.next_if_eq(&[0x03]) {
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
                bytes.pack(&[0x00])?;
            }
            ScanEnable::InquiryScanEnabled_PageScanDisabled => {
                bytes.pack(&[0x01])?;
            }
            ScanEnable::InquiryScanDisabled_PageScanEnabled => {
                bytes.pack(&[0x02])?;
            }
            ScanEnable::InquiryScanEnabled_PageScanEnabled => {
                bytes.pack(&[0x03])?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for LeMeta {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0x01]) {
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
        if bytes.next_if_eq(&[0x02]) {
            return Ok(LeMeta::LeAdvertisingReport(bytes.unpack()?));
        }
        if bytes.next_if_eq(&[0x03]) {
            return Ok(LeMeta::LeConnectionUpdateComplete {
                status: bytes.unpack()?,
                connection_handle: bytes.unpack()?,
                interval: bytes.unpack()?,
                latency: bytes.unpack()?,
                timeout: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x05]) {
            return Ok(LeMeta::LeLongTermKeyRequest {
                connection_handle: bytes.unpack()?,
                random_number: bytes.unpack()?,
                encrypted_diversifier: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x07]) {
            return Ok(LeMeta::LeDataLengthChange {
                connection_handle: bytes.unpack()?,
                max_tx_octets: bytes.unpack()?,
                max_tx_time: bytes.unpack()?,
                max_rx_octets: bytes.unpack()?,
                max_rx_time: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x08]) {
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
                bytes.pack(&[0x01])?;
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
                bytes.pack(&[0x02])?;
                bytes.pack(m0)?;
            }
            LeMeta::LeConnectionUpdateComplete {
                status,
                connection_handle,
                interval,
                latency,
                timeout,
            } => {
                bytes.pack(&[0x03])?;
                bytes.pack(status)?;
                bytes.pack(connection_handle)?;
                bytes.pack(interval)?;
                bytes.pack(latency)?;
                bytes.pack(timeout)?;
            }
            LeMeta::LeLongTermKeyRequest {
                connection_handle,
                random_number,
                encrypted_diversifier,
            } => {
                bytes.pack(&[0x05])?;
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
                bytes.pack(&[0x07])?;
                bytes.pack(connection_handle)?;
                bytes.pack(max_tx_octets)?;
                bytes.pack(max_tx_time)?;
                bytes.pack(max_rx_octets)?;
                bytes.pack(max_rx_time)?;
            }
            LeMeta::LeReadLocalP256PublicKeyComplete { status, public_key } => {
                bytes.pack(&[0x08])?;
                bytes.pack(status)?;
                bytes.pack(public_key)?;
            }
        };
        Ok(())
    }
}
impl FromToPacket for HciEventMsg {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        if bytes.next_if_eq(&[0x05]) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEventMsg::DisconnectComplete {
                status: bytes.unpack()?,
                connection_handle: bytes.unpack()?,
                reason: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x08]) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEventMsg::EncryptionChange {
                status: bytes.unpack()?,
                connection_handle: bytes.unpack()?,
                encryption_enabled: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x13]) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEventMsg::NumberOfCompletedPackets {
                num_hci_command_packets: bytes.unpack()?,
                connection_handle: bytes.unpack()?,
                num_completed_packets: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x3e]) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEventMsg::LeMeta(bytes.unpack()?));
        }
        if bytes.next_if_eq(&[0x0E]) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEventMsg::CommandComplete {
                num_hci_command_packets: bytes.unpack()?,
                command_opcode: bytes.unpack()?,
                status: bytes.unpack()?,
                data: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0x0F]) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEventMsg::CommandStatus {
                status: bytes.unpack()?,
                num_hci_command_packets: bytes.unpack()?,
                command_opcode: bytes.unpack()?,
            });
        }
        if bytes.next_if_eq(&[0xFF]) {
            bytes.unpack_length::<u8>()?;
            return Ok(HciEventMsg::VendorSpecific(bytes.unpack()?));
        }
        Err(
            PacketError::Unspecified(
                format!("No matching variant found for {}", stringify!(HciEventMsg)),
            ),
        )
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        match self {
            HciEventMsg::DisconnectComplete { status, connection_handle, reason } => {
                bytes.pack(&[0x05])?;
                bytes.pack_length::<u8>()?;
                bytes.pack(status)?;
                bytes.pack(connection_handle)?;
                bytes.pack(reason)?;
            }
            HciEventMsg::EncryptionChange {
                status,
                connection_handle,
                encryption_enabled,
            } => {
                bytes.pack(&[0x08])?;
                bytes.pack_length::<u8>()?;
                bytes.pack(status)?;
                bytes.pack(connection_handle)?;
                bytes.pack(encryption_enabled)?;
            }
            HciEventMsg::NumberOfCompletedPackets {
                num_hci_command_packets,
                connection_handle,
                num_completed_packets,
            } => {
                bytes.pack(&[0x13])?;
                bytes.pack_length::<u8>()?;
                bytes.pack(num_hci_command_packets)?;
                bytes.pack(connection_handle)?;
                bytes.pack(num_completed_packets)?;
            }
            HciEventMsg::LeMeta(m0) => {
                bytes.pack(&[0x3e])?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
            }
            HciEventMsg::CommandComplete {
                num_hci_command_packets,
                command_opcode,
                status,
                data,
            } => {
                bytes.pack(&[0x0E])?;
                bytes.pack_length::<u8>()?;
                bytes.pack(num_hci_command_packets)?;
                bytes.pack(command_opcode)?;
                bytes.pack(status)?;
                bytes.pack(data)?;
            }
            HciEventMsg::CommandStatus {
                status,
                num_hci_command_packets,
                command_opcode,
            } => {
                bytes.pack(&[0x0F])?;
                bytes.pack_length::<u8>()?;
                bytes.pack(status)?;
                bytes.pack(num_hci_command_packets)?;
                bytes.pack(command_opcode)?;
            }
            HciEventMsg::VendorSpecific(m0) => {
                bytes.pack(&[0xFF])?;
                bytes.pack_length::<u8>()?;
                bytes.pack(m0)?;
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
                bytes.pack(&[0b00])?;
            }
            PacketBoundaryFlag::Continuation => {
                bytes.pack(&[0b01])?;
            }
            PacketBoundaryFlag::FirstFlushable => {
                bytes.pack(&[0b10])?;
            }
            PacketBoundaryFlag::Deprecated => {
                bytes.pack(&[0b11])?;
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
                bytes.pack(&[0b00])?;
            }
            BroadcastFlag::BdEdrBroadcast => {
                bytes.pack(&[0b01])?;
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
                bytes.pack(&[0x00])?;
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
                bytes.pack(&[0])?;
            }
            Role::Peripheral => {
                bytes.pack(&[1])?;
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
                bytes.pack(&[0])?;
            }
            AddressType::Random => {
                bytes.pack(&[1])?;
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
                bytes.pack(&[0])?;
            }
            ClockAccuracy::Ppm250 => {
                bytes.pack(&[1])?;
            }
            ClockAccuracy::Ppm150 => {
                bytes.pack(&[2])?;
            }
            ClockAccuracy::Ppm100 => {
                bytes.pack(&[3])?;
            }
            ClockAccuracy::Ppm75 => {
                bytes.pack(&[4])?;
            }
            ClockAccuracy::Ppm50 => {
                bytes.pack(&[5])?;
            }
            ClockAccuracy::Ppm30 => {
                bytes.pack(&[6])?;
            }
            ClockAccuracy::Ppm20 => {
                bytes.pack(&[7])?;
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
                bytes.pack(&[0x00])?;
            }
            IOCapability::DisplayYesNo => {
                bytes.pack(&[0x01])?;
            }
            IOCapability::KeyboardOnly => {
                bytes.pack(&[0x02])?;
            }
            IOCapability::NoInputNoOutput => {
                bytes.pack(&[0x03])?;
            }
            IOCapability::KeyboardDisplay => {
                bytes.pack(&[0x04])?;
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
                bytes.pack(&[0x00])?;
            }
            OOBDataFlag::OobAvailable => {
                bytes.pack(&[0x01])?;
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
                bytes.pack(&[0x01])?;
            }
            SmpPairingFailure::OobNotAvailable => {
                bytes.pack(&[0x02])?;
            }
            SmpPairingFailure::AuthenticationRequirements => {
                bytes.pack(&[0x03])?;
            }
            SmpPairingFailure::ConfirmValueFailed => {
                bytes.pack(&[0x04])?;
            }
            SmpPairingFailure::PairingNotSupported => {
                bytes.pack(&[0x05])?;
            }
            SmpPairingFailure::EncryptionKeySize => {
                bytes.pack(&[0x06])?;
            }
            SmpPairingFailure::CommandNotSupported => {
                bytes.pack(&[0x07])?;
            }
            SmpPairingFailure::UnspecifiedReason => {
                bytes.pack(&[0x08])?;
            }
            SmpPairingFailure::RepeatedAttempts => {
                bytes.pack(&[0x09])?;
            }
            SmpPairingFailure::InvalidParameters => {
                bytes.pack(&[0x0A])?;
            }
            SmpPairingFailure::DhKeyCheckFailed => {
                bytes.pack(&[0x0B])?;
            }
            SmpPairingFailure::NumericComparisonFailed => {
                bytes.pack(&[0x0C])?;
            }
            SmpPairingFailure::BrEdrPairingInProgress => {
                bytes.pack(&[0x0D])?;
            }
            SmpPairingFailure::CrossTransportKeyDerivationGenerationNotAllowed => {
                bytes.pack(&[0x0E])?;
            }
            SmpPairingFailure::KeyRejected => {
                bytes.pack(&[0x0F])?;
            }
            SmpPairingFailure::Busy => {
                bytes.pack(&[0x10])?;
            }
        };
        Ok(())
    }
}
