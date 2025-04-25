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
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum PacketError {
    NotEnoughBytes,
    InvalidInstruction,
    InvalidBytes,
    InvalidBits,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct BitsetInstruction {
    pub bits: (usize, usize),
    pub bytes: usize,
    pub advance: usize,
}

pub trait FromToBytes {
    type ByteArray: AsRef<[u8]>; // = [u8; 8], etc.
    fn to_bytes(&self) -> Self::ByteArray;
    fn from_bytes(bytes: &[u8]) -> Result<Self, PacketError>
    where
        Self: Sized;
}

impl FromToBytes for bool {
    type ByteArray = [u8; 1];
    fn to_bytes(&self) -> Self::ByteArray {
        [if *self { 1 } else { 0 }]
    }
    fn from_bytes(bytes: &[u8]) -> Result<Self, PacketError> {
        Ok(*bytes.get(0).ok_or(PacketError::InvalidBytes)? == 1_u8)
    }
}

macro_rules! impl_from_to_bytes {
    ($type:ty, $size:expr) => {
        impl FromToBytes for $type {
            type ByteArray = [u8; $size];
            fn to_bytes(&self) -> Self::ByteArray {
                self.to_le_bytes()
            }
            fn from_bytes(bytes: &[u8]) -> Result<Self, PacketError> {
                <$type>::from_le_bytes(bytes.try_into().map_err(|_| PacketError::NotEnoughBytes)?)
                    .try_into()
                    .map_err(|_| PacketError::InvalidBytes)
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

    pub fn rewind(&mut self) {
        self.position = 0;
    }

    pub fn if_next_bytes(&mut self, bytes_eq: &[u8]) -> bool {
        let v = self
            .data
            .get(self.position..bytes_eq.len())
            .map_or(false, |slice| slice == bytes_eq);
        if v {
            self.position += bytes_eq.len();
        }
        v
    }

    fn push_instruction(
        &mut self,
        bits: (usize, usize),
        bytes: usize,
        advance: usize,
    ) -> Result<(), PacketError> {
        if bits.0 > (8 * bytes - 1) || bits.1 > (8 * bytes - 1) {
            return Err(PacketError::InvalidBits);
        }
        self.instructions.push(BitsetInstruction {
            bits,
            bytes,
            advance,
        });
        Ok(())
    }

    /// Set bit mask for the next byte to be packed. The bits are set in the range (start, end).
    pub fn set_bits<T: Sized>(&mut self, start: usize, end: usize) -> Result<(), PacketError> {
        let size = std::mem::size_of::<T>();
        self.push_instruction((start, end), size, 0)
    }

    fn pop_instruction(&mut self) -> Result<(), PacketError> {
        if self.instructions.is_empty() {
            return Err(PacketError::InvalidInstruction);
        }
        self.instructions.pop();
        Ok(())
    }

    pub fn pack<T: FromToBytes>(&mut self, bytes: T) -> Result<(), PacketError> {
        let bytes_ = bytes.to_bytes();
        let bytes = bytes_.as_ref();
        self.pack_bytes(bytes)
    }

    pub fn unpack<T: FromToBytes>(&mut self) -> Result<T, PacketError>
    where
        T::ByteArray: Default,
    {
        let size = std::mem::size_of::<T>();
        let bytes = self.unpack_bytes(size)?;
        T::from_bytes(&bytes.as_slice())
    }

    pub fn pack_bytes<T: AsRef<[u8]>>(&mut self, bytes: T) -> Result<(), PacketError> {
        let bytes = bytes.as_ref();
        if self.data.len() < self.position + bytes.len() {
            self.data.resize(self.position + bytes.len(), 0);
        }
        if let Some(instruction) = self.instructions.last() {
            if instruction.bytes != bytes.len() {
                return Err(PacketError::InvalidBytes);
            }
            // Update old bits with new bits
            let mut old_bits = self.data[self.position..self.position + instruction.bytes].to_vec();
            set_bits_le(&mut old_bits, instruction.bits, bytes);

            // Copy the updated bits back to the data
            self.data[self.position..self.position + instruction.bytes].copy_from_slice(&old_bits);
            self.position += instruction.advance;
            self.instructions.pop();
        } else {
            // No instruction, just copy the bytes
            self.data[self.position..self.position + bytes.len()].copy_from_slice(bytes);
            self.position += bytes.len();
        }
        Ok(())
    }

    pub fn unpack_bytes(&mut self, size: usize) -> Result<Vec<u8>, PacketError> {
        if self.position + size > self.data.len() {
            return Err(PacketError::NotEnoughBytes);
        }
        if let Some(instruction) = self.instructions.last() {
            if instruction.bytes != size {
                return Err(PacketError::InvalidBytes);
            }
            let bytes = &self.data[self.position..self.position + size];
            let bytes = get_bits_le(&bytes, instruction.bits);
            self.position += instruction.advance;
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
        packet.pack::<u8>(0x01).unwrap();
        packet.pack::<u16>(0x0302).unwrap();
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

        packet.set_bits::<u8>(2, 3).unwrap();
        packet.pack::<u8>(0xff).unwrap();

        packet.set_bits::<u8>(6, 7).unwrap();
        packet.pack::<u8>(0xff).unwrap();

        assert_eq!(packet.get_bytes(), &[0b1100_1100]);

        packet.next::<u8>().unwrap();
        packet.pack::<u8>(0xFF).unwrap();
        assert_eq!(packet.get_bytes(), &[0b1100_1100, 0b1111_1111]);
    }

    #[test]
    fn test_unpack_with_set_bits() {
        let mut packet = Packet::from_slice(&[0b1100_1100, 0b1111_1111]);
        packet.set_bits::<u8>(2, 3).unwrap();
        assert_eq!(packet.unpack::<u8>().unwrap(), 0b0000_0011);
        packet.set_bits::<u8>(0, 3).unwrap();
        assert_eq!(packet.unpack::<u8>().unwrap(), 0b0000_1100);
        packet.set_bits::<u8>(6, 7).unwrap();
        assert_eq!(packet.unpack::<u8>().unwrap(), 0b0000_0011);
    }

    #[test]
    fn test_pack_bluetooth_acl_header() {
        let connection_handle = 0x040_u16;
        let pb_flag = 0b_10_u16;
        let bc_flag = 0b_10_u16;

        let mut packet = Packet::new();

        // PB Flag (2 bits)
        packet.set_bits::<u16>(12, 13).unwrap();
        packet.pack::<u16>(pb_flag).unwrap();

        // BC Flag (2 bits)
        packet.set_bits::<u16>(14, 15).unwrap();
        packet.pack::<u16>(bc_flag).unwrap();

        // Connection Handle (12 bits)
        packet.set_bits::<u16>(0, 11).unwrap();
        packet.pack::<u16>(connection_handle).unwrap();

        assert_eq!(packet.get_bytes(), &[0x40, 0b1010_0000]);
    }

    #[test]
    fn test_unpack_bluetooth_acl_header() {
        let mut packet = Packet::from_slice(&[0x40, 0b1010_0000]);

        // PB Flag (2 bits)
        packet.set_bits::<u16>(12, 13).unwrap();
        let pb_flag = packet.unpack::<u16>().unwrap();

        // BC Flag (2 bits)
        packet.set_bits::<u16>(14, 15).unwrap();
        let bc_flag = packet.unpack::<u16>().unwrap();

        // Connection Handle (12 bits)
        packet.set_bits::<u16>(0, 11).unwrap();
        let connection_handle = packet.unpack::<u16>().unwrap();

        assert_eq!(pb_flag, 0b_10_u16);
        assert_eq!(bc_flag, 0b_10_u16);
        assert_eq!(connection_handle, 0x040_u16);
    }
}
