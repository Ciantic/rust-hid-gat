use bt_hci::cmd::{AsyncCmd, SyncCmd};
use bt_hci::controller::blocking::{Controller as BlockingController, TryError};
// use bt_hci::transport::Transport;
use bt_hci::controller::{
    Controller as StreamingController, ControllerCmdAsync, ControllerCmdSync,
};

use libc::{self, stat};
use log::info;
use std::{io, mem};


// Define the missing Bluetooth constants.
const BTPROTO_HCI: i32 = 1;
const HCI_CHANNEL_RAW: u16 = 0;
const HCI_CHANNEL_USER: u16 = 1;
const HCI_COMMAND_PKT: u8 = 0x01;
const HCI_DEV_DOWN: u64 = 0x400448CA;

const OGF_LE_CTL: u16 = 0x08; // LE controller commands group.
const OCF_LE_SET_ADVERTISING_PARAMETERS: u16 = 0x0006;
const OCF_LE_SET_ADVERTISE_ENABLE: u16 = 0x000A;

/*

let pairing_request_header = [
    02, handle[0],
    0x20, // ACL and handle, and PB,BC flag, notice if Handle is > 255 this fails
    0x0b, 0x00, // ACL Packet length (11)
    0x07, 0x00, // L2CAP length (7)
    0x06, 0x00, // SMP Protocol
    0x01, // Pairing request
            // 1 byte = IO capabilities
            // 1 byte = OOB data present
            // 1 byte = Authentication requirements
            // 1 byte = Max encryption key size
            // 1 byte = Initiator key distribution
            // 1 byte = Responder key distribution
];
*/

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

#[derive(Debug, Default)]
struct Connection {
    handle: [u8; 2],
    role: [u8; 1],
    peer_address_type: [u8; 1],
    bd_addr: [u8; 6],
    interval: [u8; 2],
    latency: [u8; 2],
    supervision_timeout: [u8; 2],
    central_clock_accuracy: [u8; 1],
}

#[derive(Debug, Default)]
struct State {
    connection: Option<Connection>,
}

pub fn run_parrot() -> Result<(), String> {
    let mut state = State::default();

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
            info!("ioctl failed");
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

        // Sent unknwown command
        write(
            fd,
            &[
                0x01, 0x01, 0x03, 0x08, 0xff, 0xff, 0xfb, 0xff, 0x07, 0xf8, 0xbf, 0x3d,
            ],
        )?;
        read(fd, &mut [0u8; 8096])?;

        // LE Set Event Mask
        write(
            fd,
            &[
                0x01, 0x01, 0x20, 0x08, 0xff, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
        )?;
        read(fd, &mut [0u8; 8096])?;

        // Write Scan enabled
        write(fd, &[0x01, 0x1A, 0x0C, 0x01, 0x03])?;
        read(fd, &mut [0u8; 8096])?;

        // Write Connection Accept Timeout
        write(fd, &[0x01, 0x16, 0x0C, 0x02, 0xA0, 0x3F])?;
        read(fd, &mut [0u8; 8096])?;

        // Wiret Page Timeout
        write(fd, &[0x01, 0x18, 0x0C, 0x02, 0x00, 0x40])?;
        read(fd, &mut [0u8; 8096])?;

        // Read Local Supported Commands
        write(fd, &[0x01, 0x02, 0x10, 0x00])?;
        read(fd, &mut [0u8; 8096])?;

        // Read BD Address
        write(fd, &[0x01, 0x09, 0x10, 0x00])?;
        read(fd, &mut [0u8; 8096])?;

        // Read Buffer Size
        write(fd, &[0x01, 0x02, 0x20, 0x00])?;
        read(fd, &mut [0u8; 8096])?;

        // Change local name
        write(
            fd,
            &[
                0x01, 0x13, 0x0C, 0xF8, 0x4D, 0x79, 0x20, 0x50, 0x69, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
        )?;
        read(fd, &mut [0u8; 8096])?;

        // LE Set Random Address 0xFE ...
        write(
            fd,
            &[0x01, 0x05, 0x20, 0x06, 0xFE, 0x89, 0x39, 0xC0, 0x0C, 0xE0],
        )?;
        read(fd, &mut [0u8; 8096])?;

        // LE Set Advertising Parameters
        write(
            fd,
            &[
                0x01, 0x06, 0x20, 0x0f, 0x00, 0x02, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x07, 0x00,
            ],
        )?;
        read(fd, &mut [0u8; 8096])?;

        // LE Set Advertising Data
        write(
            fd,
            &[
                0x1, 0x8, 0x20, 0x20, 0x10, 0x2, 0x1, 0x6, 0x3, 0x19, 0xc1, 0x3, 0x4, 0x8, 0x48,
                0x49, 0x44, 0x3, 0x2, 0x12, 0x18, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0,
            ],
        )?;
        read(fd, &mut [0u8; 8096])?;

        // LE Read Local P-256 Public Key
        write(fd, &[0x1, 0x25, 0x20, 0x0])?;
        read(fd, &mut [0u8; 8096])?;
        read(fd, &mut [0u8; 8096])?;

        // LE Set Advertising Enable
        write(fd, &[0x1, 0xa, 0x20, 0x1, 0x1])?;
        read(fd, &mut [0u8; 8096])?;

        loop {
            // Loop until connection is established
            let mut buf = [0u8; 8096];
            let read_len = read(fd, &mut buf)?;
            if read_len > 0 {
                // 0x04 = HCI Event
                // 0x3e = LE Meta Event
                // 0x13 = Parameter Total Length
                // 0x01 = Connection Complete
                // 0x00 = Status == Success
                let excpected_bytes: [u8; 5] = [0x04, 0x3e, 0x13, 0x01, 0x00];
                if buf[0..5] == excpected_bytes {
                    // Connection complete event
                    // Connection handle is 12 bits
                    let handle_: u16 = (buf[5] as u16 | ((buf[6] as u16) << 8)) & 0xfff;
                    let handle = handle_.to_le_bytes();
                    let role = &buf[7..8];
                    let peer_address_type = &buf[8..9];
                    let bd_addr = &buf[9..15];
                    let interval = &buf[15..17];
                    let latency = &buf[17..19];
                    let supervision_timeout = &buf[19..21];
                    let central_clock_accuracy = &buf[21..22];

                    // bd_addr: [37, 38, 154, 234, 64, 70]

                    state.connection = Some(Connection {
                        handle: handle.try_into().unwrap(),
                        role: role.try_into().unwrap(),
                        peer_address_type: peer_address_type.try_into().unwrap(),
                        bd_addr: bd_addr.try_into().unwrap(),
                        interval: interval.try_into().unwrap(),
                        latency: latency.try_into().unwrap(),
                        supervision_timeout: supervision_timeout.try_into().unwrap(),
                        central_clock_accuracy: central_clock_accuracy.try_into().unwrap(),
                    });
                    println!("Connection established: {:?}", state.connection);
                    break;
                } else {
                    println!("Unexpected data: {:?}", &buf[0..read_len]);
                }
            }
        }
        let handle = state.connection.as_ref().unwrap().handle;

        // Read: Vendor specific command
        // read(fd, &mut [0u8; 8096])?;

        // SEND MTU Request (Client) 244
        write(
            fd,
            &[
                0x2, handle[0], handle[1], 0x7, 0x0, 0x3, 0x0, 0x4, 0x0, 0x2, 0xf4, 0x0,
            ],
        )?;

        // Send MTU Response (Server) 247
        write(
            fd,
            &[
                0x2, handle[0], handle[1], 0x7, 0x0, 0x3, 0x0, 0x4, 0x0, 0x3, 0xf7, 0x0,
            ],
        )?;

        println!("Waiting for pairing...");
        loop {
            let mut buf = [0u8; 8096];
            let readsize = read(fd, &mut buf)?;

            if buf[0..10]
                == [
                    02, handle[0], 0x20, 0x0b, 0x00, 0x07, 0x00, 0x06, 0x00, 0x01,
                ]
            {
                // Pairing request
                write(
                    fd,
                    &[
                        0x02, handle[0], handle[1], 0x0b, 0x00, 0x07, 0x00, 0x06, 0x00, 0x02,
                        0x03, // No Input No Output
                        0x00, // OOB data not present
                        0x01, // AuthReq
                        0x10, // Max encryption key size
                        0x00, // Initiator key distribution
                        0x01, // Responder key distribution
                    ],
                )?;
                println!("Pairing response sent")

            } else if buf[0..10] == [0x2, handle[0], 0x20, 0x15, 0x0, 0x11, 0x0, 0x6, 0x0, 0x3] {
                // Pairing confirm
                write(
                    fd,
                    &[
                        0x2, handle[0], 0x0, 0x15, 0x0, 0x11, 0x0, 0x6, 0x0, 0x3,
                        // Confirm value:
                        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x10, 0x11, 0x12,
                        0x13, 0x14, 0x15, 0x16,
                    ],
                )?;
                println!("Pairing confirm sent")
            } else if buf[0..10] == [0x2, handle[0], 0x20, 0x15, 0x00, 0x11, 0x0, 0x6, 0x0, 0x4] {
                // Pairing random
                write(
                    fd,
                    &[
                        0x2, handle[0], 0x0, 0x15, 0x00, 0x11, 0x00, 0x06, 0x00, 0x04,
                        // Random value:
                        0x6d, 0xde, 0x61, 0xf5, 0x68, 0x16, 0x96, 0x67, 0x8a, 0x5e, 0x28, 0x70, 0x1a, 0x34, 0x38, 0x0,
                    ],
                )?;
                println!("Pairing random sent")
            } else {
                println!("Unexpected data");
            }
                /* else if buf[0..10] == [02, handle[0], 32, 19] {
                println!("Pairing complete");
                break;
            } */
        }

        // read(fd, &mut [0u8; 8096])?;
        // // read(fd, &mut [0u8; 8096])?;
        // // read(fd, &mut [0u8; 8096])?;
        // // read(fd, &mut [0u8; 8096])?;
        // // Send MTU 244

        // read(fd, &mut [0u8; 8096])?;
        // read(fd, &mut [0u8; 8096])?;
        // read(fd, &mut [0u8; 8096])?;
        // read(fd, &mut [0u8; 8096])?;
        /*
                < 04 3e 13 01 00 40 00 01   01 b5 65 b1 1f f7 4f 27 00 00 00 f4 01 01
        < 04 ff 05 55 00 00 40 00
        < 02 40 20 0b 00 07 00 04 00 10 01 00 ff ff 00 28
        < 04 3e 0a 03 00 40 00 06 00 00 00 f4 01
        < 02 40 20 0b 00 07 00 04 00 10 01 00 ff ff 00 28
        < 02 40 20 0b 00 07 00 04 00 10 01 00 ff ff 00 28
        < 04 05 04 00 40 00 13

                 */

        // read(fd, &mut [0u8; 8096])?;

        // read(fd, &mut [0u8; 8096])?;

        // read(fd, &mut [0u8; 8096])?;
        // read(fd, &mut [0u8; 8096])?;
        // read(fd, &mut [0u8; 8096])?;
        // read(fd, &mut [0u8; 8096])?;
        // read(fd, &mut [0u8; 8096])?;

        libc::close(fd);
    }
    Ok(())
}
