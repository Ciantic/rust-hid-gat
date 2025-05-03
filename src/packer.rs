use std::{
    collections::HashSet,
    error::Error,
    fmt::{Display, Formatter},
};

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
    fn test_set_bits_u128() {
        let val = 0x0_u128;
        let mut bytes = val.to_le_bytes();
        set_bits_le(&mut bytes, (100, 101), &[0b11]);
        let res = u128::from_le_bytes(bytes);
        assert_eq!(res, 0x30000000000000000000000000);
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedSizeUtf8<const N: usize>(String);

impl<const N: usize> FixedSizeUtf8<N> {
    pub fn new(string: &str) -> Self {
        FixedSizeUtf8(string.to_string())
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

impl<const N: usize> FromToPacket for FixedSizeUtf8<N> {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        let bytes = bytes.unpack_bytes(N)?;
        let bytes = bytes.split(|&b| b == 0).next().unwrap_or(&[]);
        let string = String::from_utf8(bytes.to_vec()).map_err(|_| PacketError::InvalidBytes)?;
        Ok(FixedSizeUtf8(string))
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        let str = self.0.as_bytes();
        if str.len() > N {
            return Err(PacketError::InvalidBytes);
        }
        bytes.pack_bytes(str)?;
        if str.len() < N {
            bytes.pack_bytes(vec![0; N - str.len()])?;
        }
        Ok(())
    }
}

impl FromToPacket for bool {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        let res = bytes.unpack_bytes(1)?;
        Ok(res[0] == 1_u8)
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        bytes.pack_bytes(&[*self as u8])?;
        Ok(())
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
        bytes.pack_bytes(*self)?;
        Ok(())
    }
}

impl FromToPacket for Vec<u8> {
    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
        Ok(bytes.data[bytes.position..].to_vec())
    }
    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
        bytes.pack_bytes(self.as_slice())?;
        Ok(())
    }
}

macro_rules! impl_from_to_bytes {
    ($type:ty, $size:expr) => {
        impl FromToPacket for $type {
            fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
                // Pad bytes to the size of the type
                let mut opbytes = bytes.unpack_bytes($size)?.clone();
                opbytes.resize($size, 0);

                Ok(Self::from_le_bytes(
                    opbytes
                        .as_slice()
                        .try_into()
                        .map_err(|_| PacketError::NotEnoughBytes)?,
                ))
            }
            fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                bytes.pack_bytes(self.to_le_bytes())?;
                Ok(())
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
    Unspecified(String),
}

impl Display for PacketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketError::NotEnoughBytes => write!(f, "Not enough bytes"),
            PacketError::InvalidInstruction => write!(f, "Invalid instruction"),
            PacketError::InvalidBytes => write!(f, "Invalid bytes"),
            PacketError::Unspecified(msg) => write!(f, "Unspecified: {}", msg),
        }
    }
}

impl Error for PacketError {
    fn description(&self) -> &str {
        match self {
            PacketError::NotEnoughBytes => "Not enough bytes",
            PacketError::InvalidInstruction => "Invalid instruction",
            PacketError::InvalidBytes => "Invalid bytes",
            PacketError::Unspecified(msg) => msg,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct LengthPosition {
    position: usize,
    offset: i32,
    bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Packet {
    position: usize,
    bit_position: usize,
    bits: usize,
    data: Vec<u8>,
    length_positions: HashSet<LengthPosition>,
}

impl AsMut<Packet> for Packet {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Packet {
    pub fn new() -> Self {
        Packet {
            position: 0,
            bit_position: 0,
            bits: 0,
            data: Vec::new(),
            length_positions: HashSet::new(),
        }
    }

    pub fn from_slice(bytes: &[u8]) -> Self {
        Packet {
            position: 0,
            bit_position: 0,
            bits: 0,
            data: bytes.to_vec(),
            length_positions: HashSet::new(),
        }
    }

    pub fn get_bytes(&mut self) -> &[u8] {
        // TODO: This doesn't need to mutate `data`, it could return a copy of
        // the data with length positions updated with the values
        for length_position in &self.length_positions {
            // Length of bytes from position to end of data, minus the length and offset
            let length = (self.data.len() as i32 - 1)
                - (length_position.position as i32 + length_position.bytes as i32 - 1)
                + length_position.offset;

            let len_bytes = length.to_le_bytes();

            self.data[length_position.position
                ..length_position.position + length_position.bytes.min(len_bytes.len())]
                .copy_from_slice(&len_bytes[0..length_position.bytes.min(len_bytes.len())]);
        }
        &self.data
    }

    pub fn next_if_eq<T>(&mut self, bytes_eq: &T) -> bool
    where
        T: FromToPacket + Eq,
    {
        let pos = self.position;
        let bits = self.bits;
        let bit_position = self.bit_position;

        if let Ok(val) = T::from_packet(self) {
            if val == *bytes_eq {
                return true;
            }
        }

        // Reset position and bit position if unpack fails
        self.position = pos;
        self.bit_position = bit_position;
        self.bits = bits;
        false
    }

    pub fn set_bits(&mut self, size: usize) -> &mut Self {
        self.bits = size;
        self
    }

    pub fn pack_length<T: FromToPacket + Default>(&mut self) -> Result<&mut Self, PacketError> {
        // Only allow packing length if no instructions are present
        if self.bits != 0 {
            return Err(PacketError::InvalidInstruction);
        }

        let bytes = std::mem::size_of::<T>();
        self.length_positions.insert(LengthPosition {
            position: self.position,
            offset: 0,
            bytes,
        });

        self.pack_bytes(vec![0; bytes])?;
        Ok(self)
    }

    pub fn pack_length_with_offset<T: FromToPacket + Default>(
        &mut self,
        offset: i32,
    ) -> Result<&mut Self, PacketError> {
        // Only allow packing length if bit width is 0
        if self.bits != 0 {
            return Err(PacketError::InvalidInstruction);
        }

        let bytes = std::mem::size_of::<T>();
        self.length_positions.insert(LengthPosition {
            position: self.position,
            offset,
            bytes,
        });

        self.pack_bytes(vec![0; bytes])?;
        Ok(self)
    }

    pub fn unpack_length<T: FromToPacket>(&mut self) -> Result<&mut Self, PacketError> {
        // Only allow packing length if bit width is 0
        if self.bits != 0 {
            return Err(PacketError::InvalidInstruction);
        }

        // Ignore the length, for now at least
        T::from_packet(self)?;
        Ok(self)
    }

    pub fn unpack<T: FromToPacket>(&mut self) -> Result<T, PacketError> {
        T::from_packet(self)
    }

    pub fn pack<T: FromToPacket>(&mut self, bytes: &T) -> Result<&mut Self, PacketError> {
        bytes.to_packet(self)?;
        Ok(self)
    }

    fn pack_bytes<T: Into<Vec<u8>>>(&mut self, bytes: T) -> Result<&mut Self, PacketError> {
        let bytes: Vec<u8> = bytes.into();
        if self.bits != 0 {
            // Use instruction.bits as *length* of the instruction
            let byte_count = (self.bit_position + self.bits + 7) / 8;

            // Resize to accommodate the instruction size
            if self.data.len() < self.position + byte_count {
                self.data.resize(self.position + byte_count, 0);
            }

            // Note: bit_position is guaranteed to be 0-7
            let end_bit = self.bit_position + self.bits - 1;

            // Get n-amount of bits from the `bytes` to be stored in the `data` array
            let input_bytes = get_bits_le(&bytes, (0, self.bits - 1));
            let mut mut_bytes = &mut self.data[self.position..self.position + byte_count];
            set_bits_le(&mut mut_bytes, (self.bit_position, end_bit), &input_bytes);
            self.progress_bits();

            // for i in 0..byte_count {
            //     let byte_index = self.position + i;

            //     // Calculate the start and end bits for the current byte (zero-indexed)
            //     let end_bit_in_byte = if i == byte_count - 1 { end_bit % 8 } else { 7 };
            //     let start_bit_in_byte = if i == 0 { self.bit_position } else { 0 };

            //     // Zero out the bits in the byte (between start_bit and end_bit)
            //     let mask =
            //         ((1 << (end_bit_in_byte - start_bit_in_byte + 1)) - 1) << start_bit_in_byte;
            //     self.data[byte_index] &= !mask;

            //     // Store the section from input_bytes into the data array
            //     let input_byte = input_bytes[i];
            //     let input_mask = (1 << (end_bit_in_byte - start_bit_in_byte + 1)) - 1;
            //     let extracted_bits = input_byte & input_mask;
            //     self.data[byte_index] |= extracted_bits << start_bit_in_byte;
            // }
        } else {
            let size = bytes.len();
            if self.data.len() < self.position + size {
                self.data.resize(self.position + size, 0);
            }

            // No instruction, just copy the bytes
            self.data[self.position..self.position + size].copy_from_slice(&bytes);
            self.progress_bytes(size);
        }
        Ok(self)
    }

    fn unpack_bytes(&mut self, size: usize) -> Result<Vec<u8>, PacketError> {
        let val = self.try_unpack_bytes(size)?;
        if self.bits != 0 {
            self.progress_bits();
        } else {
            self.progress_bytes(size);
        }
        Ok(val)
    }

    fn try_unpack_bytes(&self, size: usize) -> Result<Vec<u8>, PacketError> {
        if self.bits != 0 {
            let byte_count = (self.bit_position + self.bits + 7) / 8;
            if self.position + byte_count > self.data.len() {
                return Err(PacketError::NotEnoughBytes);
            }
            let out = get_bits_le(
                &self.data[self.position..self.position + byte_count],
                (self.bit_position, self.bit_position + self.bits - 1),
            );
            return Ok(out);
        }
        if self.position + size > self.data.len() {
            return Err(PacketError::NotEnoughBytes);
        }
        // No instruction, just copy the bytes
        let bytes = &self.data[self.position..self.position + size];
        Ok(bytes.to_vec())
    }

    fn progress_bits(&mut self) {
        let end_bit = self.bit_position + self.bits - 1;
        self.position += (self.bit_position + self.bits) / 8;
        self.bit_position = (end_bit + 1) % 8;
        self.bits = 0;
    }

    fn progress_bytes(&mut self, bytes: usize) {
        self.position += bytes;
        self.bit_position = 0; // Reset bit position
        self.bits = 0; // Reset bits after use
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
    fn test_packet_pack_set_bits_small() {
        let mut packet = Packet::new();

        // Set 0-3 bits to 0xB, and pack 4-7 bits to 0xA

        packet.set_bits(4).pack::<u128>(&0xBBBBB).unwrap();
        assert_eq!(packet.get_bytes(), &[0xB]);

        packet.set_bits(4).pack::<u128>(&0xAAAAA).unwrap();
        assert_eq!(packet.get_bytes(), &[0xAB]);
    }

    #[test]
    fn test_packet_unpack_set_bits_small() {
        let mut packet = Packet::from_slice(&[0xAB, 0xFF]);
        assert_eq!(packet.set_bits(4).unpack::<u128>(), Ok(0xB));
        assert_eq!(packet.set_bits(4).unpack::<u128>(), Ok(0xA));
        assert_eq!(packet.unpack::<u8>(), Ok(0xFF));
    }

    #[test]
    fn test_packet_pack_set_bits_large() {
        let mut packet = Packet::new();
        packet.set_bits(128).pack::<u8>(&0xbb).unwrap();
        assert_eq!(
            packet.get_bytes(),
            &[0xBB, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );

        packet.pack::<u8>(&0xAA).unwrap();
        assert_eq!(
            packet.get_bytes(),
            &[0xBB, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xAA]
        );
    }

    #[test]
    fn test_packet_unpack_set_bits_large() {
        let mut packet =
            Packet::from_slice(&[0xBB, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xAA]);
        assert_eq!(packet.set_bits(128).unpack::<u8>(), Ok(0xBB));
        assert_eq!(packet.set_bits(8).unpack::<u8>().unwrap(), 0xAA);
    }

    #[test]
    fn test_packet_set_bits_then_pack() {
        let mut packet = Packet::new();
        packet.set_bits(4).pack::<u128>(&0xBBBBB).unwrap();
        assert_eq!(packet.get_bytes(), &[0xB]);

        // Notice: set_bits below 8 bits does not advance the position, thus next pack will overwrite the previous one

        packet.pack::<u8>(&0xAA).unwrap();
        assert_eq!(packet.get_bytes(), &[0xAA]);
    }

    #[test]
    fn test_unpack_with_set_bits() {
        let mut packet = Packet::from_slice(&[0b1100_1100, 0b1111_1111]);
        assert_eq!(packet.set_bits(4).unpack::<u8>(), Ok(0b1100));
        assert_eq!(packet.set_bits(2).unpack::<u8>(), Ok(0b00));
        assert_eq!(packet.set_bits(2).unpack::<u8>(), Ok(0b11));
        assert_eq!(packet.unpack::<u8>(), Ok(0b1111_1111));
    }

    #[test]
    fn test_pack_length() {
        let mut packet = Packet::new();
        // Length from *current* position to the end of the packet,
        packet.pack_length::<u8>().unwrap();
        packet.pack_bytes(&[0xA, 0xB, 0xC]).unwrap();
        assert_eq!(packet.get_bytes(), &[0x03, 0xA, 0xB, 0xC]);
    }

    #[test]
    fn test_pack_length2() {
        let mut packet = Packet::new();
        packet.pack_bytes(&[0x1, 0x2, 0x3]).unwrap();

        // Length from *current* position to the end of the packet
        packet.pack_length::<u16>().unwrap();
        packet.pack_bytes(&[0xA, 0xB, 0xC]).unwrap();

        // It should be 3 bytes from the current position to the end of the packet
        assert_eq!(
            packet.get_bytes(),
            &[
                0x1, 0x2, 0x3, // Prefix
                0x03, 0x00, // Length
                0xA, 0xB, 0xC // Data
            ]
        );
    }

    #[test]
    fn test_pack_length_with_offset() {
        let mut packet = Packet::new();
        packet.pack_bytes(&[0x1, 0x2, 0x3]).unwrap();

        // Length from *current* position to the end of the packet
        packet.pack_length_with_offset::<u16>(-2).unwrap();
        packet.pack_bytes(&[0xA, 0xB, 0xC]).unwrap();

        // It should be 3 bytes from the current position to the end of the packet
        assert_eq!(
            packet.get_bytes(),
            &[
                0x1, 0x2, 0x3, // Prefix
                0x01, 0x00, // Length
                0xA, 0xB, 0xC // Data
            ]
        );
    }

    #[test]
    fn test_unpack_length() {
        let mut packet = Packet::from_slice(&[0x03, 0xA, 0xB, 0xC]);
        packet.unpack_length::<u8>().unwrap();
        let value = packet.unpack::<u8>().unwrap();
        assert_eq!(value, 0xA);

        assert_eq!(packet.get_bytes(), &[0x3, 0xA, 0xB, 0xC]);
    }

    #[test]
    fn test_pack_24bit_field() {
        let mut packet = Packet::new();
        let value: u32 = 0xABCDEF01;
        packet.set_bits(24).pack::<u32>(&value).unwrap();
        assert_eq!(packet.get_bytes(), &[0x01, 0xEF, 0xCD]);

        let mut packet = Packet::from_slice(&[0x01, 0xEF, 0xCD]);
        let value = packet.set_bits(24).unpack::<u32>().unwrap();
        assert_eq!(value, 0xCDEF01);
    }

    #[test]
    fn test_next_if_eq_with_set_bits() {
        let mut packet = Packet::from_slice(&[0xFF]);
        assert_eq!(packet.set_bits(1).next_if_eq(&[0x01]), true);
        assert_eq!(packet.set_bits(1).next_if_eq(&[0x01]), true);
        assert_eq!(packet.set_bits(6).unpack::<u8>(), Ok(0b0011_1111));
    }
}
