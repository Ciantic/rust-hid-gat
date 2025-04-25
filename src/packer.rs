/// Set bits from a byte array into a target byte array based on the specified
/// bit range, assumes little-endian byte order.
fn set_bits_le(val: &mut [u8], bits: (usize, usize), value: &[u8]) {
    let (start, end) = bits;
    let num_bits = end - start + 1;

    for i in 0..num_bits {
        let val_bit_pos = start + i;

        let byte_index = val_bit_pos / 8;
        let bit_in_byte = val_bit_pos % 8;

        if byte_index >= val.len() {
            continue;
        }

        // Get the bit from the value
        let value_bit = {
            let bit_index = i;
            let byte = value.get(bit_index / 8).unwrap_or(&0);
            (byte >> (bit_index % 8)) & 1
        };

        // Clear the bit
        val[byte_index] &= !(1 << bit_in_byte);

        // Set if needed
        if value_bit == 1 {
            val[byte_index] |= 1 << bit_in_byte;
        }
    }
}

/// Extract bits from a byte array based on the specified bit range, assumes
/// little-endian byte order.
fn get_bits_le(val: &[u8], bits: (usize, usize)) -> Vec<u8> {
    let (start, end) = bits;
    let num_bits = end - start + 1;

    let mut result = vec![0u8; val.len()]; // same length as input

    for i in 0..num_bits {
        let val_bit_pos = start + i;
        let val_byte = val_bit_pos / 8;
        let val_bit = val_bit_pos % 8;

        if val_byte >= val.len() {
            continue;
        }

        let bit = (val[val_byte] >> val_bit) & 1;

        let res_byte = i / 8;
        let res_bit = i % 8;

        if res_byte < result.len() {
            result[res_byte] |= bit << res_bit;
        }
    }

    result
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn test_set_bits_example1() {
        let mut bytes = vec![0x00, 0x00];
        set_bits_le(&mut bytes, (12, 13), &[0b10_u8, 0x00]);
        set_bits_le(&mut bytes, (0, 11), &0x040_u16.to_le_bytes());
        assert_eq!(bytes, &[0x40, 0x20]);
    }

    #[test]
    fn test_set_bits_u16_le() {
        let mut bytes = 0xBEEFu16.to_le_bytes();
        set_bits_le(&mut bytes, (0, 15), &[0xFE, 0xCA]);
        let res = u16::from_le_bytes(bytes);
        assert_eq!(res, 0xCAFE);
    }

    #[test]
    fn test_set_bits_u16_le_2() {
        let val = 0xBEEF_u16;
        let mut bytes = val.to_le_bytes();
        set_bits_le(&mut bytes, (0, 7), &[0xAA]);
        let res = u16::from_le_bytes(bytes);
        assert_eq!(res, 0xBEAA);
    }

    #[test]
    fn test_get_bits_u16_le() {
        // 0xbeefu16 in little endian is [0xef, 0xbe]
        let bytes = 0xBEEFu16.to_le_bytes();
        // Extract all 16 bits
        assert_eq!(get_bits_le(&bytes, (0, 15)), vec![0xef, 0xbe]);
        // Extract lower 8 bits (should be [0xef])
        assert_eq!(get_bits_le(&bytes, (0, 7)), vec![0xef, 0x00]);
        // Extract upper 8 bits (should be [0xbe])
        assert_eq!(get_bits_le(&bytes, (8, 15)), vec![0xbe, 0x00]);
        // Extract bits 4..11 (should be 0xee, i.e. 0b11101110)
        assert_eq!(get_bits_le(&bytes, (4, 11)), vec![0xee, 0x00]);
    }

    #[test]
    fn test_get_bits_u128_le() {
        let bytes = 0x123456789abcdef0123456789abcdef0_u128.to_le_bytes();
        // [f0, de, bc, 9a, 78, 56, 34, 12, f0, de, bc, 9a, 78, 56, 34, 12]
        assert_eq!(
            u128::from_le_bytes(get_bits_le(&bytes, (0, 64)).try_into().unwrap()),
            0x123456789abcdef0_u128
        );
        assert_eq!(
            u128::from_le_bytes(get_bits_le(&bytes, (64, 127)).try_into().unwrap()),
            0x123456789abcdef0_u128
        );
        assert_eq!(
            u128::from_le_bytes(get_bits_le(&bytes, (0, 127)).try_into().unwrap()),
            0x123456789abcdef0123456789abcdef0_u128
        );
        assert_eq!(
            u128::from_le_bytes(
                get_bits_le(&bytes, (8 * 8 + 4, 8 * 11 + 3))
                    .try_into()
                    .unwrap()
            ),
            0xabcdef
        );
    }

    #[test]
    fn test_bit_range_bigger_than_input() {
        let bytes = 0xFF_u8.to_le_bytes();
        let result = get_bits_le(&bytes, (0, 14));
        assert_eq!(result, vec![0xFF]);
    }

    #[test]
    fn test_bit_range_smaller_than_input() {
        let bytes = 0x1234_u16.to_le_bytes();
        let result = get_bits_le(&bytes, (0, 3));
        assert_eq!(result, vec![0x04, 0x00]);
    }
}

pub trait FromToPacket {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError>
    where
        Self: Sized;
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError>;

    // Helpers

    // fn from_packet_with(
    //     bytes: &mut Packet,
    //     instruction: BitsetInstruction,
    // ) -> Result<Self, PacketError>
    // where
    //     Self: Sized,
    // {
    //     bytes.push_instruction(instruction);
    //     Self::from_packet(bytes)
    // }

    // fn from_packet_bits(bytes: &mut Packet, bits: (usize, usize)) -> Result<Self, PacketError>
    // where
    //     Self: Sized,
    // {
    //     let instruction = BitsetInstruction { bits, advance: 0 };
    //     Self::from_packet_with(bytes, instruction)
    // }

    // fn to_packet_with(
    //     &self,
    //     bytes: &mut Packet,
    //     instruction: BitsetInstruction,
    // ) -> Result<(), PacketError> {
    //     bytes.push_instruction(instruction);
    //     self.to_packet(bytes)
    // }

    // fn to_packet_bits(&self, bytes: &mut Packet, bits: (usize, usize)) -> Result<(), PacketError> {
    //     let instruction = BitsetInstruction { bits, advance: 0 };
    //     self.to_packet_with(bytes, instruction)
    // }
}

// pub trait FromToBytes {
//     type ByteArray: AsRef<[u8]>; // = [u8; 8], etc.
//     fn to_bytes(&self) -> Self::ByteArray;
//     fn from_bytes(bytes: &[u8]) -> Result<Self, PacketError>
//     where
//         Self: Sized;
// }

// impl FromToBytes for bool {
//     type ByteArray = [u8; 1];
//     fn to_bytes(&self) -> Self::ByteArray {
//         [if *self { 1 } else { 0 }]
//     }
//     fn from_bytes(bytes: &[u8]) -> Result<Self, PacketError> {
//         Ok(*bytes.get(0).ok_or(PacketError::InvalidBytes)? == 1_u8)
//     }
// }

impl FromToPacket for bool {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        let mut result = [0u8; 1];
        bytes.unpack_bytes(1)?.copy_from_slice(&mut result);
        Ok(result[0] == 1_u8)
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        bytes.pack_bytes(&[*self as u8])
    }
}

impl<const T: usize> FromToPacket for [u8; T] {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        let mut result = [0u8; T];
        let output = bytes.unpack_bytes(T)?;
        result.copy_from_slice(&output);
        Ok(result)
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        bytes.pack_bytes(&self)
    }
}

macro_rules! impl_from_to_bytes {
    ($type:ty, $size:expr) => {
        // impl FromToBytes for $type {
        //     type ByteArray = [u8; $size];
        //     fn to_bytes(&self) -> Self::ByteArray {
        //         self.to_le_bytes()
        //     }
        //     fn from_bytes(bytes: &[u8]) -> Result<Self, PacketError> {
        //         <$type>::from_le_bytes(bytes.try_into().map_err(|_| PacketError::NotEnoughBytes)?)
        //             .try_into()
        //             .map_err(|_| PacketError::InvalidBytes)
        //     }
        // }
        impl FromToPacket for $type {
            fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
                let bytes = bytes.unpack_bytes($size)?;
                Ok(Self::from_le_bytes(
                    bytes
                        .as_slice()
                        .try_into()
                        .map_err(|_| PacketError::NotEnoughBytes)?,
                ))
            }
            fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                bytes.pack_bytes(self.to_le_bytes())
            }
        }
    };
}

impl_from_to_bytes!(u8, 1);
impl_from_to_bytes!(u16, 2);
impl_from_to_bytes!(u32, 4);
impl_from_to_bytes!(u64, 8);
impl_from_to_bytes!(u128, 16);
impl_from_to_bytes!(i8, 1);
impl_from_to_bytes!(i16, 2);
impl_from_to_bytes!(i32, 4);
impl_from_to_bytes!(i64, 8);
impl_from_to_bytes!(i128, 16);
impl_from_to_bytes!(f32, 4);
impl_from_to_bytes!(f64, 8);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PacketError {
    NotEnoughBytes,
    InvalidInstruction,
    InvalidBytes,
    InvalidBits,
    Unspecified(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct BitsetInstruction {
    pub bits: (usize, usize),
    pub advance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Packet {
    position: usize,
    data: Vec<u8>,
    instructions: Vec<BitsetInstruction>,
}

impl Packet {
    pub fn new() -> Self {
        Packet {
            position: 0,
            data: Vec::new(),
            instructions: Vec::new(),
        }
    }

    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Packet {
            position: 0,
            data: bytes,
            instructions: Vec::new(),
        }
    }

    pub fn from_slice(bytes: &[u8]) -> Self {
        Packet {
            position: 0,
            data: bytes.to_vec(),
            instructions: Vec::new(),
        }
    }

    pub fn get_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn next<T: Sized>(&mut self) -> Result<(), PacketError> {
        let size = std::mem::size_of::<T>();
        if (self.position + size) > self.data.len() {
            return Err(PacketError::NotEnoughBytes);
        }
        self.position += size;
        Ok(())
    }

    // pub fn peek<T: Sized + FromToPacket>(&mut self) -> Result<T, PacketError> {
    //     let size = std::mem::size_of::<T>();
    //     let bytes = &self
    //         .data
    //         .get(self.position..self.position + size)
    //         .ok_or(PacketError::NotEnoughBytes)?;
    //     let res = T::from_packet(self).map_err(|_| PacketError::InvalidBytes);
    //     self.position -= size;
    // }

    pub fn rewind(&mut self) {
        self.position = 0;
    }

    pub fn next_if_eq(&mut self, bytes_eq: &[u8]) -> bool {
        let v = self
            .data
            .get(self.position..self.position + bytes_eq.len())
            .map_or(false, |slice| slice == bytes_eq);
        if v {
            self.position += bytes_eq.len();
        }
        v
    }

    fn push_instruction(&mut self, instruction: BitsetInstruction) {
        self.instructions.push(instruction);
    }

    pub fn unpack<T: FromToPacket>(&mut self) -> Result<T, PacketError> {
        T::from_packet(self)
    }

    pub fn pack<T: FromToPacket>(&mut self, bytes: &T) -> Result<(), PacketError> {
        bytes.to_packet(self)
    }

    pub fn pack_with<T: FromToPacket>(
        &mut self,
        bytes: &T,
        bits: (usize, usize),
        advance: bool,
    ) -> Result<(), PacketError> {
        self.push_instruction(BitsetInstruction { bits, advance });
        bytes.to_packet(self)
    }

    pub fn unpack_with<T: FromToPacket>(
        &mut self,
        bits: (usize, usize),
        advance: bool,
    ) -> Result<T, PacketError> {
        self.push_instruction(BitsetInstruction { bits, advance });
        T::from_packet(self)
    }

    pub fn pack_bytes<T: AsRef<[u8]>>(&mut self, bytes: T) -> Result<(), PacketError> {
        let bytes = bytes.as_ref();
        let size = bytes.len();
        if self.data.len() < self.position + size {
            self.data.resize(self.position + size, 0);
        }
        if let Some(instruction) = self.instructions.last() {
            // Update old bits with new bits
            let mut old_bits = self.data[self.position..self.position + size].to_vec();
            set_bits_le(&mut old_bits, instruction.bits, bytes);

            // Copy the updated bits back to the data
            self.data[self.position..self.position + size].copy_from_slice(&old_bits);
            self.position += if instruction.advance { size } else { 0 };
            self.instructions.pop();
        } else {
            // No instruction, just copy the bytes
            self.data[self.position..self.position + bytes.len()].copy_from_slice(bytes);
            self.position += size;
        }
        Ok(())
    }

    pub fn unpack_bytes(&mut self, size: usize) -> Result<Vec<u8>, PacketError> {
        if self.position + size > self.data.len() {
            return Err(PacketError::NotEnoughBytes);
        }
        if let Some(instruction) = self.instructions.last() {
            let bytes = &self.data[self.position..self.position + size];
            let bytes = get_bits_le(&bytes, instruction.bits);
            self.position += if instruction.advance { size } else { 0 };
            self.instructions.pop();
            return Ok(bytes);
        }

        // No instruction, just copy the bytes
        let bytes = &self.data[self.position..self.position + size];
        self.position += size;
        Ok(bytes.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet() {
        let mut packet = Packet::new();
        packet.pack::<u8>(&0x01).unwrap();
        packet.pack::<u16>(&0x0302).unwrap();
        assert_eq!(packet.get_bytes(), &[0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_unpacket() {
        let mut packet = Packet::from_slice(&[0x01, 0x02, 0x03]);
        assert_eq!(packet.unpack::<u8>().unwrap(), 0x01);
        assert_eq!(packet.unpack::<u16>().unwrap(), 0x0302);

        // Assert throws error if not enough bytes
        assert_eq!(packet.unpack::<u16>(), Err(PacketError::NotEnoughBytes));
    }

    #[test]
    fn test_pack_with_set_bits() {
        let mut packet = Packet::new();

        packet.pack_with::<u8>(&0xff, (2, 3), false).unwrap();
        packet.pack_with::<u8>(&0xff, (6, 7), true).unwrap();
        assert_eq!(packet.get_bytes(), &[0b1100_1100]);

        packet.pack::<u8>(&0xff).unwrap();
        assert_eq!(packet.get_bytes(), &[0b1100_1100, 0b1111_1111]);
    }

    #[test]
    fn test_unpack_with_set_bits() {
        let mut packet = Packet::from_slice(&[0b1100_1100, 0b1111_1111]);
        assert_eq!(packet.unpack_with::<u8>((2, 3), false), Ok(0b0000_0011));
        assert_eq!(packet.unpack_with::<u8>((0, 3), false), Ok(0b0000_1100));
        assert_eq!(packet.unpack_with::<u8>((6, 7), true), Ok(0b0000_0011));
    }

    #[test]
    fn test_pack_bluetooth_acl_header() {
        let connection_handle = 0x040_u16;
        let pb_flag = 0b_10_u16;
        let bc_flag = 0b_10_u16;

        let mut packet = Packet::new();

        packet.pack_with(&pb_flag, (12, 13), false).unwrap();
        packet.pack_with(&bc_flag, (14, 15), false).unwrap();
        packet.pack_with(&connection_handle, (0, 11), true).unwrap();

        assert_eq!(packet.get_bytes(), &[0x40, 0b1010_0000]);
    }

    #[test]
    fn test_unpack_bluetooth_acl_header() {
        let mut packet = Packet::from_slice(&[0x40, 0b1010_0000]);

        let pb_flag = packet.unpack_with::<u16>((12, 13), false).unwrap();
        let bc_flag = packet.unpack_with::<u16>((14, 15), false).unwrap();
        let connection_handle = packet.unpack_with::<u16>((0, 11), true).unwrap();

        assert_eq!(pb_flag, 0b_10_u16);
        assert_eq!(bc_flag, 0b_10_u16);
        assert_eq!(connection_handle, 0x040_u16);
    }
}
