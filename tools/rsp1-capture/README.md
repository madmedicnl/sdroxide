# rsp1-capture — throwaway SDRplay RSP1 capture

Streams CF32 interleaved (f32 I, f32 Q, little-endian) to a file, so a band can
be captured and analysed without starting the GUI. Built against the tree's own
`vendor/soapysdr`, so it takes the sdrplay driver the way the program does.

```sh
cargo build --release
./target/release/rspcap <hz> <rate> <seconds> <out.raw> [gain]
# e.g. 14.0735 MHz, 48 kHz wide, 40 s, gain 45 dB
```

Two things that cost a round each:

- **`stream.activate(None)` is required** after `rx_stream`. Without it `read`
  fails with a `Timeout` error — which looks like a radio problem and is not.
- **Tune the rate to the signal.** A 500 Hz mode inside a 48 kHz capture works,
  but the comb is easier to see the narrower the slice.

`rx_stream` takes the sample type as a parameter (`rx_stream::<Complex32>`), and
`set_*` calls take `Direction::Rx` explicitly — the vendored API is not the
upstream SoapySDR signature.
