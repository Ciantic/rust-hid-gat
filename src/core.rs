use deku::prelude::*;
use facet::{Def, Facet, Field, StructKind};
use facet_reflect::{HeapValue, Wip};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HciEventMsg {
    /// id = &[0x3e, 0x13, 0x01]
    LeConnectionComplete {
        status: HciStatus,
        connection_handle: u16,
        role: Role,
        peer_address_type: AddressType,
        peer_address: [u8; 6],
        connection_interval: u16,
        peripheral_latency: u16,
        supervision_timeout: u16,
        central_clock_accuracy: ClockAccuracy,
    },
    /// id = &[0x0E, 0x04]
    CommandComplete {
        num_hci_command_packets: u8,
        command_opcode: u16,
        status: HciStatus,
    },
    /// id = &[0x0F, 0x04]
    CommandStatus {
        status: HciStatus,
        num_hci_command_packets: u8,
        command_opcode: u16,
    },
    // Other messages...
    // #[deku(id_pat = "_")]
    // Unreachable,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HciStatus {
    /// id = &[0x00]
    Success,
    /// id = &[0x01]
    Failure(u8),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Role {
    Central = 0,
    Peripheral = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AddressType {
    Public = 0,
    Random = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ClockAccuracy {
    Ppm500 = 0,
    Ppm250 = 1,
    Ppm150 = 2,
    Ppm100 = 3,
    Ppm75 = 4,
    Ppm50 = 5,
    Ppm30 = 6,
    Ppm20 = 7,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionHandle(pub u16); // max value 0x0EFF

#[cfg(test)]
mod tests {
    use crate::packer::*;

    use super::*;

    #[test]
    fn deserialize_connection_handle() {
        let mut packet = Packet::from_slice(&[0xEF, 0xBE]);
        let handle = ConnectionHandle::from_packet(&mut packet).unwrap();
        assert_eq!(handle, ConnectionHandle(0xBEEF));
    }

    #[test]
    fn deserialize_hci_stauts() {
        let mut packet = Packet::from_slice(&[0x00]);
        let handle = HciStatus::from_packet(&mut packet).unwrap();
        assert_eq!(handle, HciStatus::Success);
    }

    #[test]
    fn deserialize_role() {
        let mut packet = Packet::from_slice(&[0x00]);
        let handle = Role::from_packet(&mut packet).unwrap();
        assert_eq!(handle, Role::Peripheral);
    }

    #[test]
    fn test_event() {
        const DATA: [u8; 21] = [
            0x3e, 0x13, 0x1, 0x0, 0x40, 0x0, 0x1, 0x0, 0x26, 0xe, 0xd6, 0xe8, 0xc2, 0x50, 0x30,
            0x0, 0x0, 0x0, 0xc0, 0x3, 0x1,
        ];
        let mut packet = Packet::from_slice(&DATA);
        let msg = HciEventMsg::from_packet(&mut packet).unwrap();

        let expected = HciEventMsg::LeConnectionComplete {
            status: HciStatus::Success,
            connection_handle: 0x0040,
            role: Role::Peripheral,
            peer_address_type: AddressType::Public,
            peer_address: [0x26, 0xe, 0xd6, 0xe8, 0xc2, 0x50],
            connection_interval: 48,
            peripheral_latency: 0,
            supervision_timeout: 960,
            central_clock_accuracy: ClockAccuracy::Ppm250,
        };
        assert_eq!(msg, expected);

        // Ensure it can be serialized back to the original bytes
        // assert_eq!(msg.to_bytes().unwrap(), DATA.to_vec());
    }

    /*
    #[test]
    fn test_role() {
        assert_eq!(Role::Central.to_bytes().unwrap(), vec![0x00]);
        assert_eq!(Role::Peripheral.to_bytes().unwrap(), vec![0x01]);

        let ((rest, offset), v) = Role::from_bytes((&[0x00, 0xFF], 0)).unwrap();
        assert_eq!(v, Role::Central);
        assert_eq!(offset, 0);
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0], 0xFF);

        let ((rest, offset), v) = Role::from_bytes((&[0x01, 0xFF], 0)).unwrap();
        assert_eq!(v, Role::Peripheral);
        assert_eq!(offset, 0);
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0], 0xFF);

        // Error
        let err = Role::from_bytes((&[0x02, 0xFF], 0)).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Parse error: Could not match enum variant id = 2 on enum `Role`"
        );
    }

    #[test]
    fn test_address_type() {
        assert_eq!(AddressType::Public.to_bytes().unwrap(), vec![0x00]);
        assert_eq!(AddressType::Random.to_bytes().unwrap(), vec![0x01]);

        let ((rest, offset), v) = AddressType::from_bytes((&[0x00, 0xFF], 0)).unwrap();
        assert_eq!(v, AddressType::Public);
        assert_eq!(offset, 0);
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0], 0xFF);

        let ((rest, offset), v) = AddressType::from_bytes((&[0x01, 0xFF], 0)).unwrap();
        assert_eq!(v, AddressType::Random);
        assert_eq!(offset, 0);
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0], 0xFF);
    }

    #[test]
    fn test_hci_status() {
        assert_eq!(HciStatus::Success.to_bytes().unwrap(), vec![0x00]);
        assert_eq!(HciStatus::Failure(0x05).to_bytes().unwrap(), vec![0x05]);

        let ((rest, offset), v) = HciStatus::from_bytes((&[0x00, 0xFF], 0)).unwrap();
        assert_eq!(v, HciStatus::Success);
        assert_eq!(offset, 0);
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0], 0xFF);

        let ((rest, offset), v) = HciStatus::from_bytes((&[0x05, 0xFF], 0)).unwrap();
        assert_eq!(v, HciStatus::Failure(0x05));
        assert_eq!(offset, 0);
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0], 0xFF);
    }

    #[test]
    fn test_connection_handle() {
        assert_eq!(
            ConnectionHandle(0xBEEF).to_bytes().unwrap(),
            vec![0xEF, 0xBE]
        );

        let ((rest, offset), v) = ConnectionHandle::from_bytes((&[0xEF, 0xBE, 0xCA], 0)).unwrap();
        assert_eq!(v, ConnectionHandle(0xBEEF));
        assert_eq!(offset, 0);
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0], 0xCA);
    }
    */
}
