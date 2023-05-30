[![crates.io](https://img.shields.io/crates/v/libftd3xx-ffi.svg)](https://crates.io/crates/libftd3xx-ffi)
[![docs.rs](https://docs.rs/libftd3xx-ffi/badge.svg)](https://docs.rs/libftd3xx-ffi/)
[![CI](https://github.com/ftdi-rs/libftd3xx-ffi/workflows/CI/badge.svg)](https://github.com/ftdi-rs/libftd3xx-ffi/actions)

# libftd3xx-ffi

Rust FFI bindings to the [FTDI D3XX drivers]. THIS REPOSITORY IS STILL EXPERIMENTAL AND NON-FUNCTIONAL.

This crate is **just** the C bindings.
There is a separate crate, [libftd3xx], which provides safe wrappers around
the unsafe C bindings.

## Usage
Simply add this crate as a dependency in your `Cargo.toml`.
The static library is distributed in this crate with permission from FTDI.
The default feature set will use dynamic linking.

```toml
[dependencies]
libftd3xx-ffi = "~0.0.2"
```

### Bindgen
The default feature set will use pre-generated bindings.
This is only available for Windows x86_64 and Linux x86_64 platforms.

The bindings can also be generated during compilation using the [bindgen]
feature flag.
```toml
[dependencies]
libftd3xx-ffi = { version = "~0.0.2", features = ["bindgen"] }
```

Bindgen has additional dependencies that must be installed in order to
compile successfully, see the [bindgen requirements] page for more details.

### Static Linking
Static linking the FTD3XX library into this crate can be done by using
the static feature flag.
```toml
[dependencies]
libftd3xx-ffi = { version = "~0.0.2", features = ["static"] }
```
Static linking may be preferred, however there may be license
incompatibilities (static linking with GPL code).
If in doubt, check the FTDI [driver license terms].

## Supported Targets

### Tested Targets

TODO: UPDATE THIS
THIS REPOSITORY IS STILL EXPERIMENTAL AND NON-FUNCTIONAL.

### Untested Targets

These targets are provided, but they are untested.
Use at your own risk.

* `aarch64-unknown-linux-gnu` (dynamic + static)
* `aarch64-unknown-linux-musl` (static)
* `i686-pc-windows-msvc` (dynamic + static)
* `i686-unknown-linux-gnu` (dynamic + static)
* `i686-unknown-linux-musl` (static)
* `x86_64-pc-windows-msvc` (dynamic + static)
* `x86_64-unknown-linux-gnu` (dynamic + static)
* `x86_64-unknown-linux-musl` (static)

* `arm-unknown-linux-gnueabihf` (dynamic + static)
* `arm-unknown-linux-musleabihf` (static)
* `armv7-unknown-linux-gnueabihf` (dynamic + static)
* `armv7-unknown-linux-musleabihf` (static)
* `x86_64-apple-darwin` (dynamic)
* `aarch64-apple-darwin` (dynamic)

## References

* [D3XX Programmers Guide V1.4]
* [D3XX Drivers Download Page]

## Troubleshooting
### Unknown Device on Linux

TODO: UPDATE THIS

Remove the VCP FTDI driver.
```bash
sudo rmmod ftdi_sio
sudo rmmod usbserial
```
See [FTDI Drivers Installation Guide for Linux] for more details.

## License
FTDI provides the D3XX driver as a compiled library and a header file.
These files can be found within the `vendor` directory.

The code within the `vendor` directory is licensed by FTDI.
Please see the [driver license terms] page for their license.

All code outside of the `vendor` directory is MIT licensed.

**Note:** This crate is not affiliated with FTDI.
You will need to contact the vendor for any support requests with the
underlying library because it is closed source.


TODO: UPDATE THIS

[bindgen requirements]: https://rust-lang.github.io/rust-bindgen/requirements.html
[bindgen]: https://github.com/rust-lang/rust-bindgen
[D3XX Drivers Download Page]: https://www.ftdichip.com/Drivers/D3XX.htm
[D3xx Programmers Guide V1.4]: https://ftdichip.com/document/programming-guides/
[driver license terms]: https://ftdichip.com/driver-licence-terms-details/
[FTDI D3XX drivers]: https://www.ftdichip.com/Drivers/D3XX.htm
[FTDI Drivers Installation Guide for Linux]: http://www.ftdichip.cn/Support/Documents/AppNotes/AN_220_FTDI_Drivers_Installation_Guide_for_Linux.pdf
[libftd3xx]: https://github.com/ftdi-rs/libftd3xx
[Rust Edition Guide]: https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/musl-support-for-fully-static-binaries.html
