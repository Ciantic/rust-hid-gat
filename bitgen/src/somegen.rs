use std::io::Bytes;

/// bitgen
pub struct Foo {
    /// type = u16
    /// bits = (3, 5)
    pub a: u16,

    /// bits = 4
    pub b: u8,
    /// bits = 4
    pub c: u8,
}

pub enum Zoo {
    /// size = 2
    /// id = &[0x01, 0x02, 0x03]
    A(u16),
    /// id = &[0x04, 0x05]
    B(u8),
    /// id = &[0x06]
    C(u8),
    /// id = &[0x07, 0x08, 0x09]
    D { a: u16, b: u8, c: u8 },
    /// id = &[0x0A, 0x0B]
    E(u8, u8),
    /// id = &[0x0C]
    Foo,
}

#[cfg(test)]
mod tests {
    use crate::deps::*;

    use super::*;

    #[test]
    fn test_something() {
        // let what = Zoo::A(0x1234).to_bytes().unwrap();
        // assert_eq!(what, vec![0x01, 0x02, 0x03, 0x34, 0x12]);
    }
}
