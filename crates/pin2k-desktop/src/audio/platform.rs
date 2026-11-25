use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

#[cfg(target_os = "linux")]
pub(super) fn play_pcm_platform(
    pcm: &[i16],
    sample_rate: u32,
    channels: u16,
    cancel: &AtomicBool,
    stop_gen: &AtomicU64,
    my_gen: u64,
) {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let bytes: Vec<u8> = pcm.iter().flat_map(|s| s.to_le_bytes()).collect();
    let rate_str = sample_rate.to_string();
    let ch_str = channels.to_string();

    if let Ok(mut child) = Command::new("aplay")
        .args(["-q", "-f", "S16_LE", "-r", &rate_str, "-c", &ch_str])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(&bytes);
        }
        loop {
            if cancel.load(Ordering::Relaxed) || stop_gen.load(Ordering::Relaxed) != my_gen {
                let _ = child.kill();
                let _ = child.wait();
                break;
            }
            match child.try_wait() {
                Ok(Some(_)) | Err(_) => break,
                Ok(None) => thread::sleep(Duration::from_millis(25)),
            }
        }
    }
}

#[cfg(windows)]
pub(super) fn play_pcm_platform(
    pcm: &[i16],
    sample_rate: u32,
    channels: u16,
    cancel: &AtomicBool,
    stop_gen: &AtomicU64,
    my_gen: u64,
) {
    use std::ffi::c_void;
    use std::mem::size_of;

    #[repr(C)]
    struct WaveFormatEx {
        w_format_tag: u16, n_channels: u16, n_samples_per_sec: u32,
        n_avg_bytes_per_sec: u32, n_block_align: u16, w_bits_per_sample: u16, cb_size: u16,
    }
    #[repr(C)]
    struct WaveHdr {
        lp_data: *const u8, dw_buffer_length: u32, dw_bytes_recorded: u32, dw_user: usize,
        dw_flags: u32, dw_loops: u32, lp_next: *mut c_void, reserved: usize,
    }
    #[link(name = "winmm")]
    extern "system" {
        fn waveOutOpen(phwo: *mut *mut c_void, u_device_id: u32, pwfx: *const WaveFormatEx, dw_cb: usize, dw_inst: usize, fdw_open: u32) -> u32;
        fn waveOutPrepareHeader(hwo: *mut c_void, pwh: *mut WaveHdr, cbwh: u32) -> u32;
        fn waveOutWrite(hwo: *mut c_void, pwh: *mut WaveHdr, cbwh: u32) -> u32;
        fn waveOutReset(hwo: *mut c_void) -> u32;
        fn waveOutUnprepareHeader(hwo: *mut c_void, pwh: *mut WaveHdr, cbwh: u32) -> u32;
        fn waveOutClose(hwo: *mut c_void) -> u32;
    }

    if pcm.is_empty() || channels == 0 || sample_rate == 0 {
        return;
    }
    let bytes: Vec<u8> = pcm.iter().flat_map(|s| s.to_le_bytes()).collect();
    let block_align = channels * 2;
    let fmt = WaveFormatEx {
        w_format_tag: 1, n_channels: channels, n_samples_per_sec: sample_rate,
        n_avg_bytes_per_sec: sample_rate * (block_align as u32),
        n_block_align: block_align, w_bits_per_sample: 16, cb_size: 0,
    };
    unsafe {
        let mut hwo: *mut c_void = std::ptr::null_mut();
        if waveOutOpen(&mut hwo, u32::MAX, &fmt, 0, 0, 0) != 0 || hwo.is_null() {
            return;
        }
        let mut hdr = WaveHdr {
            lp_data: bytes.as_ptr(), dw_buffer_length: bytes.len() as u32,
            dw_bytes_recorded: 0, dw_user: 0, dw_flags: 0, dw_loops: 0,
            lp_next: std::ptr::null_mut(), reserved: 0,
        };
        if waveOutPrepareHeader(hwo, &mut hdr, size_of::<WaveHdr>() as u32) == 0 {
            let _ = waveOutWrite(hwo, &mut hdr, size_of::<WaveHdr>() as u32);
            let dur_ms = (pcm.len() as u64 * 1000) / ((sample_rate as u64) * (channels as u64));
            let mut waited = 0u64;
            while waited < dur_ms + 30 {
                if cancel.load(Ordering::Relaxed) || stop_gen.load(Ordering::Relaxed) != my_gen {
                    let _ = waveOutReset(hwo);
                    break;
                }
                thread::sleep(Duration::from_millis(25));
                waited += 25;
            }
            let _ = waveOutUnprepareHeader(hwo, &mut hdr, size_of::<WaveHdr>() as u32);
        }
        let _ = waveOutClose(hwo);
    }
}

#[cfg(not(any(target_os = "linux", windows)))]
pub(super) fn play_pcm_platform(_: &[i16], _: u32, _: u16, _: &AtomicBool, _: &AtomicU64, _: u64) {}
