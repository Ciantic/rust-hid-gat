use bt_hci::cmd::{AsyncCmd, SyncCmd};
use bt_hci::controller::blocking::{Controller as BlockingController, TryError};
// use bt_hci::transport::Transport;
use bt_hci::controller::{
    Controller as StreamingController, ControllerCmdAsync, ControllerCmdSync,
};
use bt_hci::transport::Transport as StreamingTransport;
use bt_hci::transport::blocking::Transport as BlockingTransport;
use bt_hci::{ControllerToHostPacket, HostToControllerPacket, ReadHci, WriteHci, cmd, data};
use libc;
use log::info;
use std::io;
use std::mem;
use std::ops::DerefMut;
use std::sync::Mutex;

// Define the missing Bluetooth constants.
const BTPROTO_HCI: i32 = 1;
const HCI_CHANNEL_RAW: u16 = 0;
const HCI_CHANNEL_USER: u16 = 1;
const HCI_COMMAND_PKT: u8 = 0x01;
const HCI_DEV_DOWN: u64 = 0x400448CA;

const OGF_LE_CTL: u16 = 0x08; // LE controller commands group.
const OCF_LE_SET_ADVERTISING_PARAMETERS: u16 = 0x0006;
const OCF_LE_SET_ADVERTISE_ENABLE: u16 = 0x000A;

/// Define a structure matching the C `sockaddr_hci` from <bluetooth/hci.h>
#[repr(C)]
struct sockaddr_hci {
    hci_family: libc::sa_family_t,
    hci_dev: u16,
    hci_channel: u16,
}

// Implement controller for libc sockets.

struct MutableSocket {
    fd: i32,
}
impl MutableSocket {
    fn new(fd: i32) -> Self {
        Self { fd }
    }
}
impl Drop for MutableSocket {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd) };
    }
}

pub struct LibBtSocket {
    fd: Mutex<MutableSocket>,
}

impl LibBtSocket {
    pub fn new() -> Result<Self, LibcSocketError> {
        // Bluez down
        unsafe {
            let foo = libc::socket(
                libc::AF_BLUETOOTH,
                libc::SOCK_RAW | libc::SOCK_CLOEXEC, //  | libc::SOCK_NONBLOCK,
                BTPROTO_HCI,
            );
            if foo >= 0 {
                if (libc::ioctl(foo, HCI_DEV_DOWN, 0)) < 0 {
                    info!("ioctl failed");
                    let last_err = io::Error::last_os_error();
                    unsafe { libc::close(foo) };
                    return Err(LibcSocketError::Io(last_err));
                }
                libc::close(foo);
            }
        }

        let fd = unsafe {
            libc::socket(
                libc::AF_BLUETOOTH,
                libc::SOCK_RAW | libc::SOCK_CLOEXEC, //| libc::SOCK_NONBLOCK,
                BTPROTO_HCI,
            )
        };
        if fd < 0 {
            info!("Opening socket failed");
            return Err(LibcSocketError::Other);
        }

        // Bind to hci0 (device index 0) on the raw HCI channel.
        let mut addr: sockaddr_hci = unsafe { mem::zeroed() };
        addr.hci_family = libc::AF_BLUETOOTH as u16;
        addr.hci_dev = 0; // Use hci0; adjust if needed.
        addr.hci_channel = HCI_CHANNEL_USER;
        let ret = unsafe {
            libc::bind(
                fd,
                &addr as *const _ as *const libc::sockaddr,
                mem::size_of::<sockaddr_hci>() as libc::socklen_t,
            )
        };
        if ret < 0 {
            info!("Binding socket failed");
            let last_err = io::Error::last_os_error();
            unsafe { libc::close(fd) };
            return Err(LibcSocketError::Io(last_err));
        }
        info!("Send resetbt");
        let resetbt: [u8; 4] = [0x01, 0x03, 0x0C, 0x00];
        let count = resetbt.len();
        unsafe {
            let written = libc::write(fd, resetbt.as_ptr() as *const libc::c_void, count as usize);
            if written < 0 {
                return Err(LibcSocketError::Io(io::Error::last_os_error()));
            }
            info!("Wrote {}", written);
        }
        // unsafe {
        //     let mut pollfd = libc::pollfd {
        //         fd,
        //         events: libc::POLLIN,
        //         revents: 0,
        //     };
        //     let res = libc::poll(&mut pollfd, 1, 10000);
        //     info!("Poll result {}", res);
        //     info!("Poll revents {}", pollfd.revents);
        // }
        unsafe {
            info!("Read bytes of sentok");
            let readbuffer = [0 as u8; 8190];
            let read = libc::read(
                fd,
                readbuffer.as_ptr() as *mut libc::c_void,
                readbuffer.len(),
            );
            if read < 0 {
                return Err(LibcSocketError::Io(io::Error::last_os_error()));
            }
            info!("Read hex bytes: {:?}", &readbuffer[..read as usize]);
        }
        Ok(Self {
            fd: Mutex::new(MutableSocket::new(fd)),
        })
    }
}

impl embedded_io::Write for MutableSocket {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        info!("Write bytes");
        let ret = unsafe { libc::write(self.fd, buf.as_ptr() as *const libc::c_void, buf.len()) };
        if ret < 0 {
            return Err(LibcSocketError::Io(io::Error::last_os_error()));
        }
        Ok(ret as usize)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        // unsafe {
        //     libc::fflush(self.fd);
        // }
        Ok(())
    }
}

impl embedded_io::Read for MutableSocket {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        info!("Read bytes");
        let ret = unsafe { libc::read(self.fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len()) };
        if ret < 0 {
            return Err(LibcSocketError::Io(io::Error::last_os_error()));
        }
        Ok(ret as usize)
    }
}

impl embedded_io_async::Read for MutableSocket {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        info!("Read bytes async {}", buf.len());
        // This is a blocking call, but we can use async/await to make it non-blocking.
        let ret = unsafe { libc::read(self.fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len()) };
        info!("Read bytes async ret: {}", ret);
        if ret < 0 {
            info!("Read bytes async error: {}", io::Error::last_os_error());
            return Ok(0 as usize);
            // return Err(LibcSocketError::Io(io::Error::last_os_error()));
        }
        Ok(ret as usize)
    }
}

impl embedded_io_async::Write for MutableSocket {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        info!("Write bytes async");
        let ret = unsafe { libc::write(self.fd, buf.as_ptr() as *const libc::c_void, buf.len()) };
        if ret < 0 {
            return Err(LibcSocketError::Io(io::Error::last_os_error()));
        }
        Ok(ret as usize)
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        // unsafe {
        //     libc::fflush(self.fd);
        // }
        Ok(())
    }
}

impl BlockingTransport for LibBtSocket {
    fn read<'a>(
        &self,
        rx: &'a mut [u8],
    ) -> Result<ControllerToHostPacket<'a>, TryError<Self::Error>> {
        let mut lock = self.fd.lock().map_err(|_| TryError::Busy)?;
        let packet = ControllerToHostPacket::read_hci(lock.deref_mut(), rx)
            .map_err(|rhc| TryError::Error(LibcSocketError::Other))?;
        Ok(packet)
    }

    fn write<T: HostToControllerPacket>(&self, val: &T) -> Result<(), TryError<Self::Error>> {
        let mut lock = self.fd.lock().map_err(|_| TryError::Busy)?;
        val.write_hci(lock.deref_mut())
            .map_err(|err| TryError::Error(err))?;
        Ok(())
    }
}

impl StreamingTransport for LibBtSocket {
    async fn read<'a>(&self, rx: &'a mut [u8]) -> Result<ControllerToHostPacket<'a>, Self::Error> {
        let mut lock = self.fd.lock().unwrap();
        let packet = ControllerToHostPacket::read_hci_async(lock.deref_mut(), rx)
            .await
            .unwrap();
        Ok(packet)
    }

    async fn write<T: HostToControllerPacket>(&self, tx: &T) -> Result<(), Self::Error> {
        let mut lock = self.fd.lock().unwrap();
        tx.write_hci_async(lock.deref_mut()).await.unwrap();
        Ok(())
    }
}

#[derive(Debug)]
pub enum LibcSocketError {
    Io(io::Error),
    Other,
}

impl embedded_io::Error for LibcSocketError {
    fn kind(&self) -> embedded_io::ErrorKind {
        match self {
            LibcSocketError::Io(_) => embedded_io::ErrorKind::Other,
            LibcSocketError::Other => embedded_io::ErrorKind::Other,
        }
    }
}

impl embedded_io::ErrorType for LibBtSocket {
    type Error = LibcSocketError;
}
impl embedded_io::ErrorType for MutableSocket {
    type Error = LibcSocketError;
}

impl BlockingController for LibBtSocket {
    fn write_acl_data(&self, packet: &data::AclPacket) -> Result<(), Self::Error> {
        let mut lock = self.fd.lock().unwrap();
        packet.write_hci(lock.deref_mut())?;
        Ok(())
    }

    fn write_sync_data(&self, packet: &data::SyncPacket) -> Result<(), Self::Error> {
        let mut lock = self.fd.lock().unwrap();
        packet.write_hci(lock.deref_mut())?;
        Ok(())
    }

    fn write_iso_data(&self, packet: &data::IsoPacket) -> Result<(), Self::Error> {
        let mut lock = self.fd.lock().unwrap();
        packet.write_hci(lock.deref_mut())?;

        Ok(())
    }

    fn try_write_acl_data(&self, packet: &data::AclPacket) -> Result<(), TryError<Self::Error>> {
        let mut lock = self.fd.lock().unwrap();
        packet
            .write_hci(lock.deref_mut())
            .map_err(|err| TryError::Error(LibcSocketError::Other))?;
        Ok(())
    }

    fn try_write_sync_data(&self, packet: &data::SyncPacket) -> Result<(), TryError<Self::Error>> {
        let mut lock = self.fd.lock().unwrap();
        packet
            .write_hci(lock.deref_mut())
            .map_err(|err| TryError::Error((LibcSocketError::Other)))?;
        Ok(())
    }

    fn try_write_iso_data(&self, packet: &data::IsoPacket) -> Result<(), TryError<Self::Error>> {
        let mut lock = self.fd.lock().unwrap();
        packet
            .write_hci(lock.deref_mut())
            .map_err(|err| TryError::Error((LibcSocketError::Other)))?;
        Ok(())
    }

    fn read<'a>(&self, buf: &'a mut [u8]) -> Result<ControllerToHostPacket<'a>, Self::Error> {
        info!("BlockingController read");
        let mut lock = self.fd.lock().unwrap();
        let packet = ControllerToHostPacket::read_hci(lock.deref_mut(), buf)
            .map_err(|rhc| LibcSocketError::Other)?;
        Ok(packet)
    }

    fn try_read<'a>(
        &self,
        buf: &'a mut [u8],
    ) -> Result<ControllerToHostPacket<'a>, TryError<Self::Error>> {
        info!("BlockingController try_read");
        let mut lock = self.fd.lock().unwrap();
        let packet = ControllerToHostPacket::read_hci(lock.deref_mut(), buf)
            .map_err(|rhc| TryError::Error(LibcSocketError::Other))?;
        Ok(packet)
    }
}

impl StreamingController for LibBtSocket {
    async fn write_acl_data(&self, packet: &data::AclPacket<'_>) -> Result<(), Self::Error> {
        let mut lock = self.fd.lock().unwrap();
        packet.write_hci_async(lock.deref_mut()).await.unwrap();
        Ok(())
    }

    async fn write_sync_data(&self, packet: &data::SyncPacket<'_>) -> Result<(), Self::Error> {
        let mut lock = self.fd.lock().unwrap();
        packet.write_hci_async(lock.deref_mut()).await.unwrap();
        Ok(())
    }

    async fn write_iso_data(&self, packet: &data::IsoPacket<'_>) -> Result<(), Self::Error> {
        let mut lock = self.fd.lock().unwrap();
        packet.write_hci_async(lock.deref_mut()).await.unwrap();
        Ok(())
    }

    async fn read<'a>(&self, buf: &'a mut [u8]) -> Result<ControllerToHostPacket<'a>, Self::Error> {
        // panic!("StreamingController read");
        info!("StreamingController read {}", buf.len());
        let mut lock = self.fd.lock().unwrap();
        let packet = ControllerToHostPacket::read_hci_async(lock.deref_mut(), buf).await;
        info!("Packet {:?}", packet);
        Ok(packet.unwrap())
    }
}

impl<C: SyncCmd> ControllerCmdSync<C> for LibBtSocket {
    fn exec(&self, cmd: &C) -> impl Future<Output = Result<C::Return, cmd::Error<Self::Error>>> {
        info!("ControllerCmdSync");
        async { todo!() }
    }
}
impl<C: AsyncCmd> ControllerCmdAsync<C> for LibBtSocket {
    fn exec(&self, cmd: &C) -> impl Future<Output = Result<(), cmd::Error<Self::Error>>> {
        info!("ControllerCmdAsync");
        async { todo!() }
    }
}
