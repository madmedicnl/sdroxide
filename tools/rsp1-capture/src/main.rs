//! Throwaway SoapySDR capture: stream CF32 interleaved (f32 I, f32 Q) to a file.
//!
//! Usage: rspcap <hz> <rate> <seconds> <out.bin> [gain]
use num_complex::Complex32;
use soapysdr::{Device, Direction};

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let hz: f64 = a[1].parse().expect("hz");
    let rate: f64 = a[2].parse().expect("rate");
    let secs: f64 = a[3].parse().expect("seconds");
    let out = &a[4];
    let gain: f64 = a.get(5).and_then(|g| g.parse().ok()).unwrap_or(40.0);

    let dev = Device::new("driver=sdrplay").expect("open the radio");
    dev.set_sample_rate(Direction::Rx, 0, rate).expect("sample rate");
    dev.set_frequency(Direction::Rx, 0, hz, ()).expect("frequency");
    if dev.has_gain_mode(Direction::Rx, 0).unwrap_or(false) {
        let _ = dev.set_gain_mode(Direction::Rx, 0, false);
    }
    dev.set_gain(Direction::Rx, 0, gain).expect("gain");

    let mut stream = dev.rx_stream::<Complex32>(&[0]).expect("stream");
    stream.activate(None).expect("activate");
    let want = (rate * secs) as usize;
    let mut samples: Vec<Complex32> = Vec::with_capacity(want);
    let mut chunk = vec![Complex32::new(0.0f32, 0.0f32); 8192];
    while samples.len() < want {
        let mut bufs: Vec<&mut [Complex32]> = vec![&mut chunk];
        let n = stream.read(&mut bufs, 5_000_000).expect("read");
        if n == 0 { break; }
        samples.extend_from_slice(&chunk[..n]);
    }
    let mut bytes: Vec<u8> = Vec::with_capacity(samples.len() * 8);
    for s in &samples {
        bytes.extend_from_slice(&s.re.to_le_bytes());
        bytes.extend_from_slice(&s.im.to_le_bytes());
    }
    bytes.truncate((want * 8).min(bytes.len()));
    std::fs::write(out, &bytes).expect("write");
    eprintln!("wrote {} bytes = {:.2} s at {rate}", bytes.len(), bytes.len() as f64 / 8.0 / rate);
}
