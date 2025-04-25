#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerializationError(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError(String);

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct U16Le(u16);

impl U16Le {
    pub fn new(handle: u16) -> Self {
        U16Le(handle)
    }

    #[inline]
    pub fn value(&self) -> u16 {
        self.0
    }
}

impl FromToBytes for U16Le {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
        if bytes.len() != 2 {
            return Err(ParseError::new("Not enough bytes for U16Le"));
        }
        let handle = u16::from_le_bytes([bytes[0], bytes[1]]);
        Ok(U16Le::new(handle))
    }

    fn to_bytes(&self) -> Result<Vec<u8>, SerializationError> {
        Ok(self.0.to_le_bytes().to_vec())
    }
}

impl SerializationError {
    pub fn new(msg: &str) -> Self {
        SerializationError(msg.to_string())
    }
}

impl std::fmt::Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Serialization Error: {}", self.0)
    }
}

impl std::error::Error for SerializationError {}

impl ParseError {
    pub fn new(msg: &str) -> Self {
        ParseError(msg.to_string())
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse Error: {}", self.0)
    }
}

impl std::error::Error for ParseError {}

pub trait FromToBytes {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError>
    where
        Self: Sized;
    fn to_bytes(&self) -> Result<Vec<u8>, SerializationError>;
}

macro_rules! make_enum {
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident {
            $(
                $variant:ident $( ( $($val:ty),* ) )? $(= $disc:expr)?
            ),* $(,)?
        }
    ) => {
        $(#[$attr])*
        $vis enum $name {
            $(
                $variant $( ( $($val),* ) )?, // $(= $disc)?,
            )*
        }
    };
}

#[macro_export]
macro_rules! derive_from_to_bytes_tuples {
    ($(#[$attr:meta])* $vis:vis struct $name:ident(
        $($ty:ty),*
    )) => {
        $(#[$attr])*
        $vis struct $name(
            $($ty),*
        );

        impl FromToBytes for $name {
            fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
                if bytes.len() != std::mem::size_of::<Self>() {
                    return Err(ParseError::new("Invalid byte length for struct"));
                }
                let value = <$($ty),*>::from_bytes(bytes)
                    .map_err(|_| ParseError::new("Failed to parse tuple struct"))?;
                Ok(Self(value))
            }

            fn to_bytes(&self) -> Result<Vec<u8>, SerializationError> {
                self.0.to_bytes()
                    .map_err(|_| SerializationError::new("Failed to serialize tuple struct"))
            }
        }
    };
}

#[macro_export]
macro_rules! derive_from_to_bytes_struct {
    ($(#[$attr:meta])* $vis:vis struct $name:ident {
        $($field:ident : $ty:ty),* $(,)?
    }) => {
        $(#[$attr])*
        $vis struct $name {
            $($field : $ty),*
        }

        impl FromToBytes for $name {
            fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
                let mut offset = 0;
                Ok(Self {
                    $(
                        $field: {
                            let size = std::mem::size_of::<$ty>();
                            if bytes.len() < offset + size {
                                return Err(ParseError::new(concat!("Not enough bytes for field: ", stringify!($field))));
                            }
                            let value = <$ty as FromToBytes>::from_bytes(&bytes[offset..offset + size])
                                .map_err(|_| ParseError::new(concat!("Failed to parse field: ", stringify!($field))))?;
                            offset += size;
                            value
                        }
                    ),*
                })
            }

            fn to_bytes(&self) -> Result<Vec<u8>, SerializationError> {
                let mut bytes = Vec::new();
                $(
                    bytes.extend(self.$field.to_bytes()
                        .map_err(|_| SerializationError::new(concat!("Failed to serialize field: ", stringify!($field))))?);
                )*
                Ok(bytes)
            }
        }

        // impl $name {
        //     $(
        //         $vis fn $field(&self) -> &$ty {
        //             &self.$field
        //         }
        //     )*
        // }
    };
}

make_enum! {

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum HciStatus {
        Success = (0x00, "foo"),
        Error(u8) = 0x01
    }
}
