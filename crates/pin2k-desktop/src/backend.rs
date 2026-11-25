use anyhow::Result;
#[cfg(target_os = "linux")]
use pin2k_hal::{DevPortLpt, LinuxPpdevPort};
use pin2k_hal::{LptPort, NativeSerialPort, PlayfieldSignature, SerialPdbPort, TracedLptPort, VirtualPdbPort};
use pin2k_rom_tool::parse_hex_or_dec;

pub(crate) enum DynamicLptBackend {
    #[cfg(target_os = "linux")]
    Ppdev(LinuxPpdevPort),
    #[cfg(target_os = "linux")]
    DevPort(DevPortLpt),
    Serial(SerialPdbPort<NativeSerialPort>),
    Virtual(VirtualPdbPort),
}

impl DynamicLptBackend {
    pub(crate) fn set_virtual_switch(&mut self, sw_id: u8, closed: bool) {
        if let DynamicLptBackend::Virtual(ref mut v) = self {
            v.set_logical_switch(sw_id, closed);
        }
    }

    pub(crate) fn virtual_stats(&self) -> Option<(u64, u64, u64)> {
        if let DynamicLptBackend::Virtual(ref v) = self {
            Some((v.data_writes, v.ctrl_writes, v.dispatches))
        } else {
            None
        }
    }
}

impl LptPort for DynamicLptBackend {
    fn read_data(&mut self) -> u8 {
        match self {
            #[cfg(target_os = "linux")]
            DynamicLptBackend::Ppdev(p) => p.read_data(),
            #[cfg(target_os = "linux")]
            DynamicLptBackend::DevPort(p) => p.read_data(),
            DynamicLptBackend::Serial(p) => p.read_data(),
            DynamicLptBackend::Virtual(p) => p.read_data(),
        }
    }
    fn write_data(&mut self, val: u8) {
        match self {
            #[cfg(target_os = "linux")]
            DynamicLptBackend::Ppdev(p) => p.write_data(val),
            #[cfg(target_os = "linux")]
            DynamicLptBackend::DevPort(p) => p.write_data(val),
            DynamicLptBackend::Serial(p) => p.write_data(val),
            DynamicLptBackend::Virtual(p) => p.write_data(val),
        }
    }
    fn read_status(&mut self) -> u8 {
        match self {
            #[cfg(target_os = "linux")]
            DynamicLptBackend::Ppdev(p) => p.read_status(),
            #[cfg(target_os = "linux")]
            DynamicLptBackend::DevPort(p) => p.read_status(),
            DynamicLptBackend::Serial(p) => p.read_status(),
            DynamicLptBackend::Virtual(p) => p.read_status(),
        }
    }
    fn read_control(&mut self) -> u8 {
        match self {
            #[cfg(target_os = "linux")]
            DynamicLptBackend::Ppdev(p) => p.read_control(),
            #[cfg(target_os = "linux")]
            DynamicLptBackend::DevPort(p) => p.read_control(),
            DynamicLptBackend::Serial(p) => p.read_control(),
            DynamicLptBackend::Virtual(p) => p.read_control(),
        }
    }
    fn write_control(&mut self, val: u8) {
        match self {
            #[cfg(target_os = "linux")]
            DynamicLptBackend::Ppdev(p) => p.write_control(val),
            #[cfg(target_os = "linux")]
            DynamicLptBackend::DevPort(p) => p.write_control(val),
            DynamicLptBackend::Serial(p) => p.write_control(val),
            DynamicLptBackend::Virtual(p) => p.write_control(val),
        }
    }
}

pub(crate) enum MaybeTracedLpt {
    Plain(DynamicLptBackend),
    Traced(TracedLptPort<DynamicLptBackend>),
}

impl MaybeTracedLpt {
    pub(crate) fn inner_mut(&mut self) -> &mut DynamicLptBackend {
        match self {
            MaybeTracedLpt::Plain(b) => b,
            MaybeTracedLpt::Traced(t) => &mut t.inner,
        }
    }

    pub(crate) fn inner_ref(&self) -> &DynamicLptBackend {
        match self {
            MaybeTracedLpt::Plain(b) => b,
            MaybeTracedLpt::Traced(t) => &t.inner,
        }
    }
}

impl LptPort for MaybeTracedLpt {
    fn read_data(&mut self) -> u8 {
        match self {
            MaybeTracedLpt::Plain(b) => b.read_data(),
            MaybeTracedLpt::Traced(t) => t.read_data(),
        }
    }
    fn write_data(&mut self, val: u8) {
        match self {
            MaybeTracedLpt::Plain(b) => b.write_data(val),
            MaybeTracedLpt::Traced(t) => t.write_data(val),
        }
    }
    fn read_status(&mut self) -> u8 {
        match self {
            MaybeTracedLpt::Plain(b) => b.read_status(),
            MaybeTracedLpt::Traced(t) => t.read_status(),
        }
    }
    fn read_control(&mut self) -> u8 {
        match self {
            MaybeTracedLpt::Plain(b) => b.read_control(),
            MaybeTracedLpt::Traced(t) => t.read_control(),
        }
    }
    fn write_control(&mut self, val: u8) {
        match self {
            MaybeTracedLpt::Plain(b) => b.write_control(val),
            MaybeTracedLpt::Traced(t) => t.write_control(val),
        }
    }
}

pub(crate) fn open_lpt_backend(
    spec: &str,
    serial_device: Option<&str>,
    serial_baud: u32,
    virtual_sig: PlayfieldSignature,
) -> Result<(DynamicLptBackend, String)> {
    if let Some(dev) = serial_device.or_else(|| spec.strip_prefix("serial:")) {
        let port = NativeSerialPort::open(dev, serial_baud)?;
        return Ok((
            DynamicLptBackend::Serial(SerialPdbPort::new(port)),
            format!("serial ({dev} @ {serial_baud} 8N1)"),
        ));
    }
    if spec == "emulated" {
        return Ok((
            DynamicLptBackend::Virtual(VirtualPdbPort::new(virtual_sig)),
            format!("emulated (VirtualPdbPort {virtual_sig:?})"),
        ));
    }
    #[cfg(target_os = "linux")]
    if let Some(port_str) = spec.strip_prefix("ioport:") {
        let base = parse_hex_or_dec(port_str).map_err(|e| anyhow::anyhow!(e))? as u16;
        let p = DevPortLpt::open(base)?;
        return Ok((
            DynamicLptBackend::DevPort(p),
            format!("/dev/port @ 0x{base:03X}"),
        ));
    }
    #[cfg(not(target_os = "linux"))]
    if let Some(_) = spec.strip_prefix("ioport:") {
        anyhow::bail!("direct /dev/port I/O is Linux-only; use --serial-device COMx or --lpt-device emulated on Windows");
    }
    #[cfg(target_os = "linux")]
    {
        if spec.starts_with("/dev/parport") {
            let p = LinuxPpdevPort::open(spec)?;
            return Ok((DynamicLptBackend::Ppdev(p), format!("ppdev ({spec})")));
        }
        if spec == "auto" {
            if let Some(p) = LinuxPpdevPort::auto_discover() {
                let desc = format!("auto-detected ppdev ({})", p.device_path().display());
                return Ok((DynamicLptBackend::Ppdev(p), desc));
            }
        }
    }
    let _ = parse_hex_or_dec;
    Ok((
        DynamicLptBackend::Virtual(VirtualPdbPort::new(virtual_sig)),
        format!("auto -> VirtualPdbPort ({virtual_sig:?})"),
    ))
}
