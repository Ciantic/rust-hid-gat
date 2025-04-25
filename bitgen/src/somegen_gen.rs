// use crate::deps::*;
// use crate::somegen::*;
// impl FromToBytes for Foo {
//     fn from_bytes(bytes: &mut Packet) -> Result<Self, PacketError> {
//         let result = Self {
//             a: u16::from_bytes(bytes)?,
//             b: u8::from_bytes(bytes)?,
//             c: u8::from_bytes(bytes)?,
//         };
//         Ok(result)
//     }
//     fn to_bytes(&self, bytes: &mut Packet) -> Result<(), PacketError> {
//         self.a.to_bytes(bytes)?;
//         self.b.to_bytes(bytes)?;
//         self.c.to_bytes(bytes)?;
//         Ok(())
//     }
// }
// impl FromToBytes for Zoo {
//     fn from_bytes(bytes: &mut Packet) -> Result<Self, PacketError> {
//         {
//             if bytes.if_next_bytes(&[0x01, 0x02, 0x03]) {
//                 return Ok(Zoo::A(u16::from_bytes(bytes)?));
//             }
//         }
//         {
//             if bytes.if_next_bytes(&[0x04, 0x05]) {
//                 return Ok(Zoo::B(u8::from_bytes(bytes)?));
//             }
//         }
//         {
//             if bytes.if_next_bytes(&[0x06]) {
//                 return Ok(Zoo::C(u8::from_bytes(bytes)?));
//             }
//         }
//         {
//             if bytes.if_next_bytes(&[0x07, 0x08, 0x09]) {
//                 Zoo::D {
//                     a: u16::from_bytes(bytes)?,
//                     b: u8::from_bytes(bytes)?,
//                     c: u8::from_bytes(bytes)?,
//                 }
//             }
//         }
//         {
//             if bytes.if_next_bytes(&[0x0A, 0x0B]) {
//                 return Ok(Zoo::E(u8::from_bytes(bytes)?, u8::from_bytes(bytes)?));
//             }
//         }
//         {
//             if bytes.if_next_bytes(&[0x0C]) {
//                 Zoo::Foo
//             }
//         }
//         Err("No matching variant found".to_string())
//     }
//     fn to_bytes(&self, bytes: &mut Packet) -> Result<(), PacketError> {
//         match self {
//             Zoo::A(m0) => {
//                 let mut output = vec![];
//                 output.extend(&[0x01, 0x02, 0x03]);
//                 output.extend(m0.to_bytes()?);
//                 Ok(output)
//             }
//             Zoo::B(m0) => {
//                 let mut output = vec![];
//                 output.extend(&[0x04, 0x05]);
//                 output.extend(m0.to_bytes()?);
//                 Ok(output)
//             }
//             Zoo::C(m0) => {
//                 let mut output = vec![];
//                 output.extend(&[0x06]);
//                 output.extend(m0.to_bytes()?);
//                 Ok(output)
//             }
//             Zoo::D { a, b, c } => {
//                 let mut output = Packet::new();
//                 a.to_bytes(&mut output)?;
//                 b.to_bytes(&mut output)?;
//                 c.to_bytes(&mut output)?;
//                 Ok(output)
//             }
//             Zoo::E(m0, m1) => {
//                 let mut output = vec![];
//                 output.extend(&[0x0A, 0x0B]);
//                 output.extend(m0.to_bytes()?);
//                 output.extend(m1.to_bytes()?);
//                 Ok(output)
//             }
//             Zoo::Foo => Ok(Vec::from(&[0x0C])),
//         }
//     }
// }
