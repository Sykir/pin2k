//! Cross-platform Serial Port (`COMx` on Windows, `/dev/ttyUSB*`/`/dev/ttyACM*` on Linux/macOS)
//! transport for bridging Pinball 2000 Power Driver Board (`PdbDriver`) over USB-Serial / RS-232.
//!
//! Zero external C library dependencies:
//! - Unix/Linux: `termios` 8N1 raw mode via `libc`.
//! - Windows: Win32 `CreateFileW` / `SetCommState` / `SetCommTimeouts` / `ReadFile` / `WriteFile`.
//!
//! ## Framed LPT-over-Serial Bridge Protocol
//! - Write SPP register (`0=DATA`, `2=CONTROL`): `[0x57 ('W'), reg, val]`
//! - Read SPP register (`0=DATA`, `1=STATUS`, `2=CONTROL`): `[0x52 ('R'), reg]` -> 1-byte reply

use std::io::{self, Read, Write};

use crate::lpt::{LptPort, PlayfieldSignature, VirtualPdbPort};
use crate::HalError;

/// Serial LPT-over-UART transport wrapping any `Read + Write` stream (`NativeSerialPort` or loopback).
pub struct SerialPdbPort<S: Read + Write> {
    pub stream: S,
    last_data: u8,
    last_ctrl: u8,
    pub serial_tx_bytes: u64,
    pub serial_rx_bytes: u64,
}

impl<S: Read + Write> SerialPdbPort<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            last_data: 0,
            last_ctrl: 0x04,
            serial_tx_bytes: 0,
            serial_rx_bytes: 0,
        }
    }

    fn send_write(&mut self, reg: u8, val: u8) {
        let pkt = [b'W', reg, val];
        if self.stream.write_all(&pkt).is_ok() {
            self.serial_tx_bytes += pkt.len() as u64;
        }
    }

    fn send_read(&mut self, reg: u8, fallback: u8) -> u8 {
        let pkt = [b'R', reg];
        if self.stream.write_all(&pkt).is_err() {
            return fallback;
        }
        self.serial_tx_bytes += pkt.len() as u64;
        let _ = self.stream.flush();
        let mut resp = [fallback; 1];
        if self.stream.read_exact(&mut resp).is_ok() {
            self.serial_rx_bytes += 1;
            resp[0]
        } else {
            fallback
        }
    }
}

impl<S: Read + Write> LptPort for SerialPdbPort<S> {
    fn read_data(&mut self) -> u8 {
        self.send_read(0, self.last_data)
    }

    fn write_data(&mut self, val: u8) {
        self.last_data = val;
        self.send_write(0, val);
    }

    fn read_status(&mut self) -> u8 {
        self.send_read(1, 0x7F)
    }

    fn read_control(&mut self) -> u8 {
        self.send_read(2, self.last_ctrl)
    }

    fn write_control(&mut self, val: u8) {
        self.last_ctrl = val;
        self.send_write(2, val);
    }
}

// ============================================================================
// Native Cross-Platform Serial Port (`NativeSerialPort`)
// ============================================================================

#[cfg(unix)]
mod unix_serial {
    use super::*;
    use std::ffi::CString;
    use std::os::unix::io::RawFd;

    pub struct NativeSerialPort {
        fd: RawFd,
        device: String,
        baud: u32,
    }

    impl NativeSerialPort {
        pub fn open(device: &str, baud: u32) -> Result<Self, HalError> {
            let c_path = CString::new(device)
                .map_err(|_| HalError::BackendIo("invalid serial device path".into()))?;
            let fd = unsafe {
                libc::open(
                    c_path.as_ptr(),
                    libc::O_RDWR | libc::O_NOCTTY | libc::O_SYNC,
                )
            };
            if fd < 0 {
                return Err(HalError::BackendIo(format!(
                    "cannot open serial port '{device}': {}",
                    io::Error::last_os_error()
                )));
            }

            let speed = match baud {
                9600 => libc::B9600,
                19200 => libc::B19200,
                38400 => libc::B38400,
                57600 => libc::B57600,
                115200 => libc::B115200,
                230400 => libc::B230400,
                #[cfg(target_os = "linux")]
                460800 => libc::B460800,
                #[cfg(target_os = "linux")]
                921600 => libc::B921600,
                _ => {
                    unsafe { libc::close(fd) };
                    return Err(HalError::BackendIo(format!(
                        "unsupported serial baud rate: {baud}"
                    )));
                }
            };

            unsafe {
                let mut tty: libc::termios = std::mem::zeroed();
                if libc::tcgetattr(fd, &mut tty) != 0 {
                    let err = io::Error::last_os_error();
                    libc::close(fd);
                    return Err(HalError::BackendIo(format!("tcgetattr failed: {err}")));
                }
                libc::cfmakeraw(&mut tty);
                tty.c_cflag = (tty.c_cflag & !libc::CSIZE) | libc::CS8 | libc::CREAD | libc::CLOCAL;
                tty.c_cflag &= !(libc::PARENB | libc::CSTOPB);
                tty.c_cc[libc::VMIN] = 0;
                tty.c_cc[libc::VTIME] = 2; // 200 ms read timeout
                libc::cfsetspeed(&mut tty, speed);
                if libc::tcsetattr(fd, libc::TCSANOW, &tty) != 0 {
                    let err = io::Error::last_os_error();
                    libc::close(fd);
                    return Err(HalError::BackendIo(format!("tcsetattr failed: {err}")));
                }
                libc::tcflush(fd, libc::TCIOFLUSH);
            }

            Ok(Self {
                fd,
                device: device.to_string(),
                baud,
            })
        }

        pub fn device_name(&self) -> &str {
            &self.device
        }

        pub fn baud_rate(&self) -> u32 {
            self.baud
        }
    }

    impl Read for NativeSerialPort {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let n = unsafe { libc::read(self.fd, buf.as_mut_ptr().cast(), buf.len()) };
            if n < 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(n as usize)
            }
        }
    }

    impl Write for NativeSerialPort {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let n = unsafe { libc::write(self.fd, buf.as_ptr().cast(), buf.len()) };
            if n < 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(n as usize)
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Drop for NativeSerialPort {
        fn drop(&mut self) {
            unsafe {
                libc::close(self.fd);
            }
        }
    }
}

#[cfg(windows)]
mod win_serial {
    use super::*;
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    type Handle = *mut std::ffi::c_void;
    const INVALID_HANDLE_VALUE: Handle = -1isize as Handle;
    const GENERIC_READ: u32 = 0x8000_0000;
    const GENERIC_WRITE: u32 = 0x4000_0000;
    const OPEN_EXISTING: u32 = 3;

    #[repr(C)]
    #[allow(non_snake_case)]
    struct Dcb {
        DCBlength: u32,
        BaudRate: u32,
        fBitFields: u32,
        wReserved: u16,
        XonLim: u16,
        XoffLim: u16,
        ByteSize: u8,
        Parity: u8,
        StopBits: u8,
        XonChar: i8,
        XoffChar: i8,
        ErrorChar: i8,
        EofChar: i8,
        EvtChar: i8,
        wReserved1: u16,
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    struct CommTimeouts {
        ReadIntervalTimeout: u32,
        ReadTotalTimeoutMultiplier: u32,
        ReadTotalTimeoutConstant: u32,
        WriteTotalTimeoutMultiplier: u32,
        WriteTotalTimeoutConstant: u32,
    }

    extern "system" {
        fn CreateFileW(
            lpFileName: *const u16,
            dwDesiredAccess: u32,
            dwShareMode: u32,
            lpSecurityAttributes: *mut std::ffi::c_void,
            dwCreationDisposition: u32,
            dwFlagsAndAttributes: u32,
            hTemplateFile: Handle,
        ) -> Handle;
        fn GetCommState(hFile: Handle, lpDCB: *mut Dcb) -> i32;
        fn SetCommState(hFile: Handle, lpDCB: *mut Dcb) -> i32;
        fn SetCommTimeouts(hFile: Handle, lpCommTimeouts: *const CommTimeouts) -> i32;
        fn ReadFile(
            hFile: Handle,
            lpBuffer: *mut u8,
            nNumberOfBytesToRead: u32,
            lpNumberOfBytesRead: *mut u32,
            lpOverlapped: *mut std::ffi::c_void,
        ) -> i32;
        fn WriteFile(
            hFile: Handle,
            lpBuffer: *const u8,
            nNumberOfBytesToWrite: u32,
            lpNumberOfBytesWritten: *mut u32,
            lpOverlapped: *mut std::ffi::c_void,
        ) -> i32;
        fn CloseHandle(hObject: Handle) -> i32;
    }

    pub struct NativeSerialPort {
        handle: Handle,
        device: String,
        baud: u32,
    }

    unsafe impl Send for NativeSerialPort {}

    impl NativeSerialPort {
        pub fn open(device: &str, baud: u32) -> Result<Self, HalError> {
            let win_name = if device.starts_with(r"\\.\") {
                device.to_string()
            } else {
                format!(r"\\.\{device}")
            };
            let wide: Vec<u16> = OsStr::new(&win_name)
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();

            let handle = unsafe {
                CreateFileW(
                    wide.as_ptr(),
                    GENERIC_READ | GENERIC_WRITE,
                    0,
                    std::ptr::null_mut(),
                    OPEN_EXISTING,
                    0,
                    std::ptr::null_mut(),
                )
            };
            if handle == INVALID_HANDLE_VALUE || handle.is_null() {
                return Err(HalError::BackendIo(format!(
                    "cannot open serial port '{device}': {}",
                    io::Error::last_os_error()
                )));
            }

            unsafe {
                let mut dcb: Dcb = std::mem::zeroed();
                dcb.DCBlength = std::mem::size_of::<Dcb>() as u32;
                if GetCommState(handle, &mut dcb) == 0 {
                    let err = io::Error::last_os_error();
                    CloseHandle(handle);
                    return Err(HalError::BackendIo(format!("GetCommState failed: {err}")));
                }
                dcb.BaudRate = baud;
                dcb.ByteSize = 8;
                dcb.Parity = 0; // NOPARITY
                dcb.StopBits = 0; // ONESTOPBIT
                dcb.fBitFields = 0x0001; // fBinary = 1
                if SetCommState(handle, &mut dcb) == 0 {
                    let err = io::Error::last_os_error();
                    CloseHandle(handle);
                    return Err(HalError::BackendIo(format!("SetCommState failed: {err}")));
                }

                let timeouts = CommTimeouts {
                    ReadIntervalTimeout: 20,
                    ReadTotalTimeoutMultiplier: 0,
                    ReadTotalTimeoutConstant: 200,
                    WriteTotalTimeoutMultiplier: 0,
                    WriteTotalTimeoutConstant: 200,
                };
                SetCommTimeouts(handle, &timeouts);
            }

            Ok(Self {
                handle,
                device: device.to_string(),
                baud,
            })
        }

        pub fn device_name(&self) -> &str {
            &self.device
        }

        pub fn baud_rate(&self) -> u32 {
            self.baud
        }
    }

    impl Read for NativeSerialPort {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let mut read_bytes = 0u32;
            let ok = unsafe {
                ReadFile(
                    self.handle,
                    buf.as_mut_ptr(),
                    buf.len().min(u32::MAX as usize) as u32,
                    &mut read_bytes,
                    std::ptr::null_mut(),
                )
            };
            if ok == 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(read_bytes as usize)
            }
        }
    }

    impl Write for NativeSerialPort {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let mut written = 0u32;
            let ok = unsafe {
                WriteFile(
                    self.handle,
                    buf.as_ptr(),
                    buf.len().min(u32::MAX as usize) as u32,
                    &mut written,
                    std::ptr::null_mut(),
                )
            };
            if ok == 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(written as usize)
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Drop for NativeSerialPort {
        fn drop(&mut self) {
            unsafe {
                CloseHandle(self.handle);
            }
        }
    }
}

#[cfg(unix)]
pub use unix_serial::NativeSerialPort;
#[cfg(windows)]
pub use win_serial::NativeSerialPort;

// ============================================================================
// In-Memory Serial Bridge Loopback (Testing & Simulation)
// ============================================================================

/// Simulates a microcontroller UART-to-PDB bridge (`'W' reg val` / `'R' reg`)
/// connected to a `VirtualPdbPort`.
pub struct VirtualSerialBridge {
    pub board: VirtualPdbPort,
    rx_buf: Vec<u8>,
    tx_queue: Vec<u8>,
    tx_cursor: usize,
}

impl Default for VirtualSerialBridge {
    fn default() -> Self {
        Self::new(PlayfieldSignature::Swe1)
    }
}

impl VirtualSerialBridge {
    pub fn new(sig: PlayfieldSignature) -> Self {
        Self {
            board: VirtualPdbPort::new(sig),
            rx_buf: Vec::new(),
            tx_queue: Vec::new(),
            tx_cursor: 0,
        }
    }

    fn process_packets(&mut self) {
        let mut idx = 0;
        while idx < self.rx_buf.len() {
            match self.rx_buf[idx] {
                b'W' => {
                    if idx + 3 > self.rx_buf.len() {
                        break;
                    }
                    let reg = self.rx_buf[idx + 1];
                    let val = self.rx_buf[idx + 2];
                    match reg {
                        0 => self.board.write_data(val),
                        2 => self.board.write_control(val),
                        _ => {}
                    }
                    idx += 3;
                }
                b'R' => {
                    if idx + 2 > self.rx_buf.len() {
                        break;
                    }
                    let reg = self.rx_buf[idx + 1];
                    let byte = match reg {
                        0 => self.board.read_data(),
                        1 => self.board.read_status(),
                        2 => self.board.read_control(),
                        _ => 0xFF,
                    };
                    self.tx_queue.push(byte);
                    idx += 2;
                }
                _ => {
                    idx += 1;
                }
            }
        }
        if idx > 0 {
            self.rx_buf.drain(..idx);
        }
    }
}

impl Write for VirtualSerialBridge {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.rx_buf.extend_from_slice(buf);
        self.process_packets();
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Read for VirtualSerialBridge {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let avail = self.tx_queue.len().saturating_sub(self.tx_cursor);
        if avail == 0 {
            return Ok(0);
        }
        let n = avail.min(buf.len());
        buf[..n].copy_from_slice(&self.tx_queue[self.tx_cursor..self.tx_cursor + n]);
        self.tx_cursor += n;
        if self.tx_cursor >= self.tx_queue.len() {
            self.tx_queue.clear();
            self.tx_cursor = 0;
        }
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LampState, PdbDriver, PinballHardware};

    #[test]
    fn serial_pdb_port_runs_full_pdb_protocol_over_uart_framing() {
        let bridge = VirtualSerialBridge::new(PlayfieldSignature::Swe1);
        let serial = SerialPdbPort::new(bridge);
        let mut drv = PdbDriver::new(serial);

        assert!(drv.pdb_present);
        assert_eq!(drv.probe_playfield_signature(), PlayfieldSignature::Swe1);
        assert_eq!(drv.read_fuse_status(), (0x00, 0x00));

        drv.port.stream.board.set_logical_switch(2, true); // Start button
        drv.fire_coil(8, 64); // Trough Eject
        drv.set_lamp(29, LampState::On); // JEDI 'J'

        let sw = drv.sync_io(16);
        assert_ne!(sw[0] & (1 << 2), 0);
        assert!(drv.port.stream.board.is_drive_active(8));
        assert!(drv.port.stream.board.is_lamp_lit(29));
        assert!(drv.port.serial_tx_bytes > 100);
        assert!(drv.port.serial_rx_bytes > 20);
    }
}
