use libc::{self};
use log::info;
use std::{io, mem};

use crate::c1::{c1, c1_rev};

// Define the missing Bluetooth constants.
const BTPROTO_HCI: i32 = 1;
const HCI_CHANNEL_RAW: u16 = 0;
const HCI_CHANNEL_USER: u16 = 1;
const HCI_COMMAND_PKT: u8 = 0x01;
const HCI_DEV_DOWN: u64 = 0x400448CA;

const OGF_LE_CTL: u16 = 0x08; // LE controller commands group.
const OCF_LE_SET_ADVERTISING_PARAMETERS: u16 = 0x0006;
const OCF_LE_SET_ADVERTISE_ENABLE: u16 = 0x000A;

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
        0x01, // AuthReq
        0x10, // Max encryption key size
        0x00, // Initiator key distribution
        0x01, // Responder key distribution
    ]
}

fn smp_pairing_confirm(conhandle: &[u8; 2], confirmvalue: &[u8; 16]) -> [u8; 26] {
    // Pairing confirm
    //
    // https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/security-manager-specification.html#UUID-878f7b67-2330-bef1-1d52-8414dab01d87
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

fn smp_pairing_random(conhandle: &[u8; 2], random: &[u8; 16]) -> [u8; 26] {
    // Pairing random
    //
    // https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/security-manager-specification.html#UUID-fd667b14-3d7b-58fa-bd85-6771d85293c8
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

pub fn run_parrot() -> Result<(), String> {
    // let mut state = State::default();
    let server_addr: [u8; 6] = [0xa9, 0x36, 0x3c, 0xde, 0x52, 0xd7];
    let mut preq = [0u8; 7];
    let mut pres = [0u8; 7];
    let mut conhandle: [u8; 2] = [0, 0];

    // Client address
    let mut iat = 0x0u8;
    let mut ia = [0u8; 6];
    let mut irandom = [0u8; 16];
    let mut iconfirm_value = [0u8; 16];

    // Server address
    let mut rat = 0x1u8;
    let mut ra = server_addr;
    let mut rrandom = [
        // TODO: Random 16 bytes
        0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
        0x16,
    ];

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

        println!("Waiting for pairing...");
        loop {
            let mut buf = [0u8; 8096];
            let readsize = read(fd, &mut buf)?;

            if smp_is_pairing_request(&buf, &conhandle) {
                let res = smp_pairing_response(&conhandle);
                // c1 requires preq, and pres
                preq = [0x01, buf[11], buf[12], buf[13], buf[14], buf[15], buf[16]];
                pres = [res[9], res[10], res[11], res[12], res[13], res[14], res[15]];
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
                // TODO: Verify the random
                println!("Validate the confirm value client sent");
                println!("Confirm value: {:?}", iconfirm_value);
                println!("R-Random value: {:?}", irandom);
                println!("I-Random value: {:?}", rrandom);
                println!(
                    "Verify i: {:?}",
                    c1_rev(&[0; 16], &irandom, &pres, &preq, iat, &ia, rat, &ra)
                );
                println!(
                    "Verify r: {:?}",
                    c1_rev(&[0; 16], &rrandom, &pres, &preq, iat, &ia, rat, &ra)
                );

                // Send our pairing random
                write(fd, &smp_pairing_random(&conhandle, &rrandom))?;
            } else {
                println!("Unexpected data");
            }
        }

        libc::close(fd);
    }
    Ok(())
}
