use std::collections::HashMap;

// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SerializationError {
    InvalidUuid,
    InvalidHandle,
    InvalidProperties,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
    InvalidUuid,
    InvalidHandle,
    InvalidProperties,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Uuid {
    U16(u16),
    U128([u8; 16]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle(pub u16);

/// CharacteristicProperties
///
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-957d2ce5-401b-3cf3-1150-152d226887eb
#[derive(Debug, Clone, Copy, Default)]
pub struct CharacteristicProperties {
    pub read: bool,
    pub write: bool,
    pub write_without_response: bool,
    pub notify: bool,
    pub indicate: bool,
    pub broadcast: bool,
    pub write_authenticated_signed: bool,
    // pub extended_properties: bool
}

impl CharacteristicProperties {
    pub fn to_bytes(self) -> Result<[u8; 1], SerializationError> {
        let mut byte: u8 = 0x00;
        if self.broadcast {
            byte |= 0x01;
        }
        if self.read {
            byte |= 0x02;
        }
        if self.write_without_response {
            byte |= 0x04;
        }
        if self.write {
            byte |= 0x08;
        }
        if self.notify {
            byte |= 0x10;
        }
        if self.indicate {
            byte |= 0x20;
        }
        if self.write_authenticated_signed {
            byte |= 0x40;
        }
        // if self.extended_properties {
        //     byte |= 0x80;
        // }
        Ok([byte])
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
        if bytes.len() != 1 {
            return Err(ParseError::InvalidProperties);
        }
        let byte = bytes[0];
        Ok(CharacteristicProperties {
            broadcast: (byte & 0x01) != 0,
            read: (byte & 0x02) != 0,
            write_without_response: (byte & 0x04) != 0,
            write: (byte & 0x08) != 0,
            notify: (byte & 0x10) != 0,
            indicate: (byte & 0x20) != 0,
            write_authenticated_signed: (byte & 0x40) != 0,
        })
    }
}

// -- Core structs
#[derive(Debug, Clone)]
pub enum Attribute {
    Service(Service),
    Characteristic(Characteristic),
    Descriptor(Descriptor),
}

#[derive(Debug, Clone)]
pub struct Service {
    pub handle: Handle,
    pub uuid: Uuid,
    pub primary: bool,
    pub characteristics: Vec<Characteristic>,
}

#[derive(Debug, Clone)]
pub struct Characteristic {
    pub declaration_handle: Handle,
    pub value_handle: Handle,
    pub uuid: Uuid,
    pub properties: CharacteristicProperties,
    pub descriptors: Vec<Descriptor>,
    pub value: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Descriptor {
    pub handle: Handle,
    pub uuid: Uuid,
    pub value: Vec<u8>,
}

// -- Attribute database
#[derive(Debug)]
pub struct AttributeDatabase {
    pub attributes: HashMap<Handle, Attribute>,
}

impl AttributeDatabase {
    pub fn new() -> Self {
        AttributeDatabase {
            attributes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, attr: Attribute) {
        let handle = match &attr {
            Attribute::Service(s) => s.handle,
            Attribute::Characteristic(c) => c.declaration_handle,
            Attribute::Descriptor(d) => d.handle,
        };
        self.attributes.insert(handle, attr);
    }

    pub fn respond_to_att_find_by_type_value_response(
        &self,
        handle: Handle,
        value: Vec<u8>,
    ) -> Option<Vec<Handle>> {
        if let Some(Attribute::Characteristic(c)) = self.attributes.get(&handle) {
            if c.value == value {
                return Some(vec![c.declaration_handle, c.value_handle]);
            }
        }
        None
    }
}
