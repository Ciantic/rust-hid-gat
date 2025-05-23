use libc::{self};
use log::info;
use std::{collections::HashMap, io, mem};

use crate::{
    c1::{c1, c1_rev, s1_rev},
    gatt::AttributeDatabase,
};

// Define the missing Bluetooth constants.
const BTPROTO_HCI: i32 = 1;
const HCI_CHANNEL_RAW: u16 = 0;
const HCI_CHANNEL_USER: u16 = 1;
const HCI_COMMAND_PKT: u8 = 0x01;
const HCI_DEV_DOWN: u64 = 0x400448CA;

const OGF_LE_CTL: u16 = 0x08; // LE controller commands group.
const OCF_LE_SET_ADVERTISING_PARAMETERS: u16 = 0x0006;
const OCF_LE_SET_ADVERTISE_ENABLE: u16 = 0x000A;

// Host Controller Interface documentation:
//
// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html
//
// Security manager documentation (SMP)
//
// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/security-manager-specification.html
//
// Attribute Protocol
//
// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/attribute-protocol--att-.html

fn cmd_le_event_mask() -> [u8; 12] {
    [
        0x01, 0x01, 0x20, 0x08, 0xff, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]
}

fn cmd_write_scan_enabled() -> [u8; 5] {
    [0x01, 0x1A, 0x0C, 0x01, 0x03]
}

fn cmd_write_connection_accept_timeout() -> [u8; 6] {
    [0x01, 0x16, 0x0C, 0x02, 0xA0, 0x3F]
}

fn cmd_write_page_timeout() -> [u8; 6] {
    [0x01, 0x18, 0x0C, 0x02, 0x00, 0x40]
}

fn cmd_read_local_supported_commands() -> [u8; 4] {
    [0x01, 0x02, 0x10, 0x00]
}

fn cmd_read_bd_addr() -> [u8; 4] {
    [0x01, 0x09, 0x10, 0x00]
}
fn cmd_read_buffer_size() -> [u8; 4] {
    [0x01, 0x02, 0x20, 0x00]
}

fn cmd_change_local_name() -> [u8; 252] {
    [
        0x01, 0x13, 0x0C, 0xF8, 0x4D, 0x79, 0x20, 0x50, 0x69, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]
}

fn cmd_le_set_random_address(addr: &[u8; 6]) -> [u8; 10] {
    [
        0x01, 0x05, 0x20, 0x06, addr[0], addr[1], addr[2], addr[3], addr[4], addr[5],
    ]
}

fn cmd_le_set_advertising_parameters() -> [u8; 19] {
    [
        0x01, 0x06, 0x20, 0x0f, 0x00, 0x02, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x07, 0x00,
    ]
}

fn cmd_le_set_advertising_data() -> [u8; 36] {
    [
        0x1, 0x8, 0x20, 0x20, 0x10, 0x2, 0x1, 0x6, 0x3, 0x19, 0xc1, 0x3, 0x4, 0x8, 0x48, 0x49,
        0x44, 0x3, 0x2, 0x12, 0x18, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0,
    ]
}

fn cmd_le_set_advertising_enable() -> [u8; 5] {
    [0x01, 0x0A, 0x20, 0x01, 0x01]
}

fn cmd_le_read_local_p256_public_key() -> [u8; 4] {
    [0x01, 0x25, 0x20, 0x00]
}

fn evt_is_le_connection_complete(buf: &[u8]) -> bool {
    buf[0..5] == [0x04, 0x3e, 0x13, 0x01, 0x00]
}

/// LE Long Term Key Request Event
/// 
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-2bb7b9d8-d02b-0320-3dc8-9699e4b30332
#[rustfmt::skip]
fn evt_is_le_long_term_key_request(buf: &[u8], conhandle: &[u8; 2]) -> bool {
    buf[0..16] == [0x04, 0x3e, 0xd, 0x5, conhandle[0], conhandle[1], 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]
}

/// Encryption Change event
///
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7b7d27f0-1a33-ff57-5b97-7d49a04cea26
fn evt_is_encryption_change(buf: &[u8]) -> bool {
    // Link Level Encryption is ON with AES-CCM for LE.
    buf[0..7] == [0x4, 0x8, 0x4, 0x0, 0x40, 0x0, 0x1]
}

/// Encryption Information
/// 
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/security-manager-specification.html#UUID-868e7828-879b-b232-5135-f04a72ecb7f0
#[rustfmt::skip]
fn smp_encryption_information(conhandle: &[u8; 2], ltk: &[u8; 16]) -> [u8; 26] {
    [
        0x2, conhandle[0], 0x0, 0x15, 0x0, 0x11, 0x0, 0x6, 0x0, 0x6, 
        ltk[0], ltk[1], ltk[2], ltk[3], ltk[4], ltk[5], ltk[6], ltk[7],
        ltk[8], ltk[9], ltk[10], ltk[11], ltk[12], ltk[13], ltk[14], ltk[15],
    ]
}

/// Central Identification
/// 
/// 
#[rustfmt::skip]
fn smp_central_identification(conhandle: &[u8; 2], random: &[u8; 8]) -> [u8; 20] {
    [
        0x2, conhandle[0], 0x0, 0xf, 0x0, 0xb, 0x0, 0x6, 0x0, 0x7, 
        0x0, 0x2, // Encrypted diversifier 0x0200
        random[0], random[1], random[2], random[3], 
        random[4], random[5], random[6], random[7],
    ]

}

/// LE Long Term Key Request Reply
/// 
/// For Just Works key is `s1_rev([0; 16], &rrand, &irand)` where `rrand` is the
/// server (our) random value, and `irand` is the (client) initiator random
/// value
/// 
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e89a5372-a7cd-ae2b-5c8d-e281694793ae
#[rustfmt::skip]
fn cmd_le_long_term_key_request_reply(conhandle: &[u8; 2], key: &[u8; 16]) -> [u8; 22] {
    [
        0x1, 0x1a, 0x20, 0x12, conhandle[0], conhandle[1], 
        key[0], key[1], key[2], key[3], key[4], key[5], key[6], key[7],
        key[8], key[9], key[10], key[11], key[12], key[13], key[14], key[15]
    ]
}

#[rustfmt::skip]
fn att_send_mtu_request(conhandle: &[u8; 2]) -> [u8; 12] {
    [0x2, conhandle[0], conhandle[1], 0x7, 0x0, 0x3, 0x0, 0x4, 0x0, 0x2, 0xf4, 0x0]
}

fn att_send_mtu_response(conhandle: &[u8; 2]) -> [u8; 12] {
    [
        0x2,
        conhandle[0],
        conhandle[1],
        0x7,
        0x0,
        0x3,
        0x0,
        0x4,
        0x0,
        0x3,
        0xf7,
        0x0,
    ]
}

#[rustfmt::skip]
fn att_is_read_by_group_type_request(buf: &[u8], conhandle: &[u8; 2]) -> bool {
    buf[0..10] == [0x2, conhandle[0], 0x20, 0xb, 0x0, 0x7, 0x0, 0x4, 0x0, 0x10]
}

/// ATT_FIND_BY_TYPE_VALUE_REQ
/// 
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/attribute-protocol--att-.html#UUID-48914d5f-be71-cb87-eb16-cc6fe0804da8
#[rustfmt::skip]
fn att_is_find_by_type_value_request(buf: &[u8], conhandle: &[u8; 2]) -> bool {
    buf[0..10] == [0x2, conhandle[0], 0x20, 0xd, 0x0, 0x9, 0x0, 0x4, 0x0, 0x6]
}

fn att_find_by_type_value_response(
    conhandle: &[u8; 2],
    uids_and_handles: &[(u16, u16)],
) -> Vec<u8> {
    todo!();
    /*
       02                         // HCI ACL Packet Indicator
       01 00                      // HCI Handle (0x0001), PB=0, BC=0
       12 00                      // HCI Length = 18
       0E 00                      // L2CAP Length = 14
       04 00                      // L2CAP Channel ID = ATT (0x0004)
       11                         // ATT Opcode = Read By Type Response
       04                         // Entry Length = 4 bytes
       01 00 00 18                // Handle 0x0001, UUID 0x1800
       05 00 0A 18                // Handle 0x0005, UUID 0x180A
       08 00 12 18                // Handle 0x0008, UUID 0x1812
    */
    /*
    let mut buf = vec![
        0x2,
        conhandle[0],
        conhandle[1],
        0xd,
        0x0,
        0x9,
        0x0,
        0x4,
        0x0,
        0x6,
    ];
    for (uid, handle) in uids_and_handles {
        let hbytes = handle.to_le_bytes();
        let uidbytes = uid.to_le_bytes();
        buf.push(hbytes[0]);
        buf.push(hbytes[1]);
        buf.push(uidbytes[0]);
        buf.push(uidbytes[1]);
        // buf.push((handle >> 8) as u8);
        // buf.push(*handle as u8);
        // buf.push((uid >> 8) as u8);
        // buf.push(*uid as u8);
    }
    buf
    */
}

#[rustfmt::skip]
fn att_error_find_by_type_value_attribute_not_found(conhandle: &[u8; 2]) -> [u8; 14] {
    [0x2, conhandle[0], 0x0, 0x9, 0x0, 0x5, 0x0, 0x4, 0x0, 0x1, 0x6, 0x1, 0x0, 0xa]
}

#[rustfmt::skip]
fn att_read_by_group_type_response(
    conhandle: &[u8; 2],
    atthandle: &[u8; 2],
    grpendhandle: &[u8; 2],
    uuid: &[u8; 2],
) -> [u8; 17] {
    [
        0x2, conhandle[0], 0x0, 0xc, 0x0, 0x8, 0x0, 0x4, 0x0, 0x11, 0x6, 
        // Attribute data:
        atthandle[0], atthandle[1], 
        grpendhandle[0], grpendhandle[1], 
        uuid[0], uuid[1]
    ]
}

fn smp_is_pairing_request(buf: &[u8], conhandle: &[u8; 2]) -> bool {
    // Pairing request
    //
    // https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/security-manager-specification.html#UUID-a358268d-39ef-27aa-6b2e-23949a02d79d
    buf[0..10]
        == [
            02,
            // TODO: Strictly this is not correct, connection handle is 12 bits
            conhandle[0],
            0x20,
            0x0b,
            0x00,
            0x07,
            0x00,
            0x06,
            0x00,
            0x01,
        ]
}

fn smp_is_pairing_confirm(buf: &[u8], conhandle: &[u8; 2]) -> bool {
    // Pairing confirm
    //
    // https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/security-manager-specification.html#UUID-878f7b67-2330-bef1-1d52-8414dab01d87
    buf[0..10]
        == [
            0x2,
            // TODO: Strictly this is not correct, connection handle is 12 bits
            conhandle[0],
            0x20,
            0x15,
            0x0,
            0x11,
            0x0,
            0x6,
            0x0,
            0x3,
        ]
}

fn smp_pairing_response(conhandle: &[u8; 2]) -> [u8; 16] {
    // Pairing response
    //
    // https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/security-manager-specification.html#UUID-11de7a80-eb1c-ed73-711b-51bf1720218b

    [
        0x02,
        conhandle[0],
        conhandle[1],
        0x0b,
        0x00,
        0x07,
        0x00,
        0x06,
        0x00,
        0x02, // Code 0x02
        0x03, // No Input No Output
        0x00, // OOB data not present
        0x01, // AuthReq (CT2 0, Keypress 0, Secure Conn 0, MITM 0, Bonding 0x01)
        0x10, // Max encryption key size
        0x00, // Initiator key distribution
        0x01, // Responder key distribution
    ]
}

/// Pairing confirm
///
/// Confirm value is calculated using the C1 algorithm, use `c1_rev` for simple
/// byte arrays.
///
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/security-manager-specification.html#UUID-878f7b67-2330-bef1-1d52-8414dab01d87
fn smp_pairing_confirm(conhandle: &[u8; 2], confirmvalue: &[u8; 16]) -> [u8; 26] {
    [
        0x2,
        conhandle[0],
        0x0,
        0x15,
        0x0,
        0x11,
        0x0,
        0x6,
        0x0,
        0x3,
        // Confirm value:
        confirmvalue[0],
        confirmvalue[1],
        confirmvalue[2],
        confirmvalue[3],
        confirmvalue[4],
        confirmvalue[5],
        confirmvalue[6],
        confirmvalue[7],
        confirmvalue[8],
        confirmvalue[9],
        confirmvalue[10],
        confirmvalue[11],
        confirmvalue[12],
        confirmvalue[13],
        confirmvalue[14],
        confirmvalue[15],
    ]
}

fn smp_is_pairing_random(buf: &[u8], conhandle: &[u8; 2]) -> bool {
    // Pairing random
    //
    // https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/security-manager-specification.html#UUID-fd667b14-3d7b-58fa-bd85-6771d85293c8
    buf[0..10]
        == [
            0x2,
            conhandle[0],
            0x20,
            0x15,
            0x00,
            0x11,
            0x0,
            0x6,
            0x0,
            0x4,
        ]
}

/// Pairing random response
///
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/security-manager-specification.html#UUID-fd667b14-3d7b-58fa-bd85-6771d85293c8
fn smp_pairing_random(conhandle: &[u8; 2], random: &[u8; 16]) -> [u8; 26] {
    [
        0x2,
        conhandle[0],
        0x0,
        0x15,
        0x00,
        0x11,
        0x00,
        0x06,
        0x00,
        0x04,
        // Random value:
        random[0],
        random[1],
        random[2],
        random[3],
        random[4],
        random[5],
        random[6],
        random[7],
        random[8],
        random[9],
        random[10],
        random[11],
        random[12],
        random[13],
        random[14],
        random[15],
    ]
}

/// Pairing Failed response
/// 
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/security-manager-specification.html#UUID-4fde8eb1-530c-dd12-bbd2-78a326f12f31
#[rustfmt::skip]
fn smp_pairing_failed_confirm_value(conhandle: &[u8; 2]) -> [u8; 11] {
    // 0x04 = Confirm Value Failed
    [0x2, conhandle[0], 0x20, 0x6, 0x0, 0x2, 0x0, 0x6, 0x0, 0x5, 0x4]
}

/// Define a structure matching the C `sockaddr_hci` from <bluetooth/hci.h>
#[repr(C)]
struct sockaddr_hci {
    hci_family: libc::sa_family_t,
    hci_dev: u16,
    hci_channel: u16,
}

fn dump_bytes_as_hex(bytes: &[u8], len: usize) {
    for byte in bytes.iter().take(len) {
        print!("{:02x} ", byte);
    }
    println!();
}

fn write(fd: i32, buf: &[u8]) -> Result<(), String> {
    print!("> ");
    dump_bytes_as_hex(buf, buf.len());
    let written =
        unsafe { libc::write(fd, buf.as_ptr() as *const libc::c_void, buf.len() as usize) };
    if written < 0 {
        let last_error = io::Error::last_os_error();
        unsafe {
            libc::close(fd);
        }
        return Err(format!("Write failed: {}", last_error));
    }
    Ok(())
}

fn read(fd: i32, buf: &mut [u8]) -> Result<usize, String> {
    let mut poller = libc::pollfd {
        events: libc::POLLIN,
        fd: fd,
        revents: 0,
    };
    if unsafe { libc::poll(&mut poller, 1, 1000) == 0 } {
        return Ok(0);
    }
    let read = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len()) };
    if read < 0 {
        let last_error = io::Error::last_os_error();
        unsafe {
            libc::close(fd);
        }
        return Err(format!("Read failed: {}", last_error));
    }
    print!("< ");
    dump_bytes_as_hex(buf, read as usize);
    Ok(read as usize)
}

// #[derive(Debug, Default)]
// struct Connection {
//     handle: [u8; 2],
//     role: [u8; 1],
//     peer_address_type: [u8; 1],
//     bd_addr: [u8; 6],
//     interval: [u8; 2],
//     latency: [u8; 2],
//     supervision_timeout: [u8; 2],
//     central_clock_accuracy: [u8; 1],
// }

// #[derive(Debug, Default)]
// struct State {
//     connection: Option<Connection>,
// }

pub fn run_parrot(db: AttributeDatabase) -> Result<(), String> {
    let mut short_term_key: [u8; 16];
    let long_term_key: [u8; 16] = [
        0x08, 0x7A, 0xC7, 0xFB, 0x8C, 0x86, 0xF3, 0xCF, 0x36, 0xF4, 0x0C, 0xD8, 0xDD, 0xA2, 0xF9,
        0xD3,
    ];

    // let mut state = State::default();
    let server_addr: [u8; 6] = [0xa9, 0x36, 0x3c, 0xde, 0x52, 0xd7];
    let mut preq = [0u8; 7];
    let mut pres = [0u8; 7];
    let mut conhandle: [u8; 2] = [0, 0];
    let mut max_key_size: u8 = 16;

    // Client address
    let mut iat = 0x0u8;
    let mut ia = [0u8; 6];
    let mut irandom = [0u8; 16];
    let mut iconfirm_value = [0u8; 16];

    // Server address
    let rat = 0x1u8;
    let ra = server_addr;
    let rrandom = [
        // TODO: Randomize
        0x6d, 0xde, 0x61, 0xf5, 0x68, 0x16, 0x96, 0x67, 0x8a, 0x5e, 0x28, 0x70, 0x1a, 0x34, 0x38,
        0x0,
    ];

    /*
    DEVICE = My Pi   TYPE=Mesh  node=1  ADDRESS = DC:A6:32:04:DB:56
    PRIMARY_SERVICE = 1800
        LECHAR=Device Name   SIZE=4   Permit=02 UUID=2A00
        LECHAR=Appearance    SIZE=2   Permit=02 UUID=2A01
    PRIMARY_SERVICE = 180A
        LECHAR= PnP ID           SIZE=7 Permit=02   UUID=2A50
    PRIMARY_SERVICE = 1812
        LECHAR=Protocol Mode   SIZE=1  Permit=06  UUID=2A4E
        LECHAR=HID Info        SIZE=4  Permit=02  UUID=2A4A
        LECHAR=HID Ctl Point   SIZE=8  Permit=04  UUID=2A4C
        LECHAR=Report Map      SIZE=47 Permit=02  UUID=2A4B
        LECHAR=Report1         SIZE=8  Permit=92  UUID=2A4D
    */

    struct Service {
        uuid: u16,
        start_handle: u16,
        end_handle: u16,
    }

    struct Characteristic {
        uuid: u16,
        handle: u16,
        properties: u8,
    }

    struct ServiceLayout {
        service: Service,
        characteristics: Vec<Characteristic>,
    }

    let mut service_layouts: HashMap<u16, ServiceLayout> = HashMap::new();

    unsafe {
        let fd = libc::socket(
            libc::AF_BLUETOOTH,
            libc::SOCK_RAW | libc::SOCK_CLOEXEC | libc::SOCK_NONBLOCK,
            BTPROTO_HCI,
        );

        if fd < 0 {
            return Err(format!(
                "Failed opening the socket: {}",
                io::Error::last_os_error()
            ));
        }

        // Turn off the HCI device (Bluez might be using it)
        if (libc::ioctl(fd, HCI_DEV_DOWN, 0)) < 0 {
            let last_err = io::Error::last_os_error();
            libc::close(fd);
            return Err(format!("IOCTL failed: {}", last_err));
        }

        // let fd = unsafe {
        //     libc::socket(
        //         libc::AF_BLUETOOTH,
        //         libc::SOCK_RAW | libc::SOCK_CLOEXEC, //| libc::SOCK_NONBLOCK,
        //         BTPROTO_HCI,
        //     )
        // };

        // Bind the socket to the HCI device
        let mut addr: sockaddr_hci = mem::zeroed();
        addr.hci_family = libc::AF_BLUETOOTH as u16;
        addr.hci_dev = 0; // Use hci0; adjust if needed.
        addr.hci_channel = HCI_CHANNEL_USER;
        let ret = libc::bind(
            fd,
            &addr as *const _ as *const libc::sockaddr,
            mem::size_of::<sockaddr_hci>() as libc::socklen_t,
        );
        if ret < 0 {
            let last_err = io::Error::last_os_error();
            libc::close(fd);
            return Err(format!("Binding socket failed: {}", last_err));
        }

        // Send reset
        write(fd, &[0x01, 0x03, 0x0C, 0x00])?;
        read(fd, &mut [0u8; 8096])?;

        // // Sent unknwown command
        // write(
        //     fd,
        //     &[
        //         0x01, 0x01, 0x03, 0x08, 0xff, 0xff, 0xfb, 0xff, 0x07, 0xf8, 0xbf, 0x3d,
        //     ],
        // )?;
        // read(fd, &mut [0u8; 8096])?;

        // LE Set Event Mask
        write(fd, &cmd_le_event_mask())?;
        read(fd, &mut [0u8; 8096])?;

        // Write Scan enabled
        write(fd, &cmd_write_scan_enabled())?;
        read(fd, &mut [0u8; 8096])?;

        // Write Connection Accept Timeout
        write(fd, &cmd_write_connection_accept_timeout())?;
        read(fd, &mut [0u8; 8096])?;

        // Wiret Page Timeout
        write(fd, &cmd_write_page_timeout())?;
        read(fd, &mut [0u8; 8096])?;

        // Read Local Supported Commands
        write(fd, &cmd_read_local_supported_commands())?;
        read(fd, &mut [0u8; 8096])?;

        // Read BD Address
        write(fd, &cmd_read_bd_addr())?;
        read(fd, &mut [0u8; 8096])?;

        // Read Buffer Size
        write(fd, &cmd_read_buffer_size())?;
        read(fd, &mut [0u8; 8096])?;

        // Change local name
        write(fd, &cmd_change_local_name())?;
        read(fd, &mut [0u8; 8096])?;

        // LE Set Random Address 0xFE ...
        write(fd, &cmd_le_set_random_address(&server_addr))?;
        read(fd, &mut [0u8; 8096])?;

        // LE Set Advertising Parameters
        write(fd, &cmd_le_set_advertising_parameters())?;
        read(fd, &mut [0u8; 8096])?;

        // LE Set Advertising Data
        write(fd, &cmd_le_set_advertising_data())?;
        read(fd, &mut [0u8; 8096])?;

        // LE Read Local P-256 Public Key
        write(fd, &cmd_le_read_local_p256_public_key())?;
        read(fd, &mut [0u8; 8096])?;
        read(fd, &mut [0u8; 8096])?;

        // LE Set Advertising Enable
        write(fd, &cmd_le_set_advertising_enable())?;
        read(fd, &mut [0u8; 8096])?;

        loop {
            // Loop until connection is established
            let mut buf = [0u8; 8096];
            let read_len = read(fd, &mut buf)?;
            if read_len > 0 {
                if evt_is_le_connection_complete(&buf) {
                    // Connection handle 5th and 6th byte (12 bits)
                    conhandle = ((buf[5] as u16 | ((buf[6] as u16) << 8)) & 0xfff).to_le_bytes();

                    // Bytes 9-14
                    ia = [buf[9], buf[10], buf[11], buf[12], buf[13], buf[14]];

                    // Peer address type (8th byte)
                    iat = buf[8] & 0x01;

                    println!("Connection established");
                    dump_bytes_as_hex(&ia, 6);
                    println!("Connection handle: {:02x?}", conhandle);
                    break;
                } else {
                    println!("Unexpected data: {:?}", &buf[0..read_len]);
                }
            }
        }
        // Read: Vendor specific command
        // read(fd, &mut [0u8; 8096])?;

        // SEND MTU Request (Client) 244
        write(fd, &att_send_mtu_request(&conhandle))?;

        // Send MTU Response (Server) 247
        write(fd, &att_send_mtu_response(&conhandle))?;

        println!("Waiting for pairing or connection...");
        loop {
            let mut buf = [0u8; 8096];
            let readsize = read(fd, &mut buf)?;

            if smp_is_pairing_request(&buf, &conhandle) {
                let res = smp_pairing_response(&conhandle);
                // c1 requires preq, and pres
                preq = [buf[9], buf[10], buf[11], buf[12], buf[13], buf[14], buf[15]];
                pres = [res[9], res[10], res[11], res[12], res[13], res[14], res[15]];
                max_key_size = buf[13];
                write(fd, &res)?;
            } else if smp_is_pairing_confirm(&buf, &conhandle) {
                iconfirm_value = [
                    buf[10], buf[11], buf[12], buf[13], buf[14], buf[15], buf[16], buf[17],
                    buf[18], buf[19], buf[20], buf[21], buf[22], buf[23], buf[24], buf[25],
                ];
                let confirm_value = c1_rev(&[0; 16], &rrandom, &pres, &preq, iat, &ia, rat, &ra);
                write(fd, &smp_pairing_confirm(&conhandle, &confirm_value))?;
            } else if smp_is_pairing_random(&buf, &conhandle) {
                irandom = [
                    buf[10], buf[11], buf[12], buf[13], buf[14], buf[15], buf[16], buf[17],
                    buf[18], buf[19], buf[20], buf[21], buf[22], buf[23], buf[24], buf[25],
                ];
                let confirm_value = c1_rev(&[0; 16], &irandom, &pres, &preq, iat, &ia, rat, &ra);
                if confirm_value != iconfirm_value {
                    write(fd, &smp_pairing_failed_confirm_value(&conhandle))?;
                } else {
                    write(fd, &smp_pairing_random(&conhandle, &rrandom))?;
                }
            } else if evt_is_le_long_term_key_request(&buf, &conhandle) {
                short_term_key = s1_rev(&[0; 16], &rrandom, &irandom);
                write(
                    fd,
                    &cmd_le_long_term_key_request_reply(&conhandle, &short_term_key),
                )?;
            } else if evt_is_encryption_change(&buf) {
                write(fd, &smp_encryption_information(&conhandle, &long_term_key))?;
                write(
                    fd,
                    &smp_central_identification(
                        &conhandle,
                        &[0x50, 0xc2, 0xe8, 0xd6, 0xe, 0x26, 0x9, 0xa],
                    ),
                )?;
                break;
            } else if att_is_find_by_type_value_request(&buf, &conhandle) {
                write(
                    fd,
                    &att_error_find_by_type_value_attribute_not_found(&conhandle),
                )?;
            } else if att_is_read_by_group_type_request(&buf, &conhandle) {
                let start_handle = [buf[10], buf[11]];
                let end_handle = [buf[12], buf[13]];
                let uuid = [buf[14], buf[15]];

                // 0x2800
                if uuid == [0, 0x28] {
                    write(
                        fd,
                        &att_read_by_group_type_response(
                            &conhandle,
                            &[0x03, 00],   // 0x0003
                            &[0x07, 0x00], // 0x0007
                            &uuid,
                        ),
                    )?;
                } else {
                    println!("Requested UUID {:02x?}", uuid);
                    println!("Start-End: {:02x?} - {:02x?}", start_handle, end_handle);
                    println!("Unexpected UUID: {:?}", &uuid);
                }
            } else {
                println!("Unexpected data");
            }
        }

        libc::close(fd);
    }
    Ok(())
}
