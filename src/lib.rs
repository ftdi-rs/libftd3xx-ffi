//! Rust FFI bindings to the [FTDI d3xx drivers](https://www.ftdichip.com/Drivers/d3xx.htm).
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::useless_transmute)]

pub mod prelude;

cfg_if::cfg_if! {
    if #[cfg(feature = "bindgen")] {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    } else if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
        include!("bindings_linux_x64.rs");
    } else if #[cfg(all(target_os = "linux", target_arch = "x86"))] {
        include!("bindings_linux_x86.rs");
    } else if #[cfg(all(target_os = "windows", target_arch = "x86_64"))] {
        include!("bindings_windows_x64.rs");
    } else if #[cfg(all(target_os = "windows", target_arch = "x86"))] {
        include!("bindings_windows_x86.rs");
    } else if #[cfg(all(target_os = "linux", target_arch = "arm"))] {
        include!("bindings_linux_armv6.rs");
    } else if #[cfg(all(target_os = "linux", target_arch = "aarch64"))] {
        include!("bindings_linux_armv8.rs");
    } else if #[cfg(all(target_os = "macos", target_arch = "x86_64"))] {
        include!("bindings_macos_x64.rs");
    } else if #[cfg(all(target_os = "macos", target_arch = "aarch64"))] {
        include!("bindings_macos_aarch64.rs");
    } else {
        std::compile_error!("pre-generated bindings are not avaliable for your target");
    }
}

// Bindgen generates different types for linux and windows so we need to implement TryFrom to be
// able to convert into FT_STATUS. In the future enums should be fixed hopefully.
// See https://github.com/rust-lang/rust-bindgen/issues/1907 for more details
type FT_Status = _FT_STATUS;
macro_rules! create_ft_status_tryfrom {
    ($t:ident) => {
        impl TryFrom<$t> for FT_Status {
            type Error = &'static str;

            fn try_from(v: $t) -> Result<Self, Self::Error> {
                match v {
                    x if x == FT_Status::FT_OK as $t => Ok(FT_Status::FT_OK),
                    x if x == FT_Status::FT_INVALID_HANDLE as $t => Ok(FT_Status::FT_INVALID_HANDLE),
                    x if x == FT_Status::FT_DEVICE_NOT_FOUND as $t => Ok(FT_Status::FT_DEVICE_NOT_FOUND),
                    x if x == FT_Status::FT_DEVICE_NOT_OPENED as $t => Ok(FT_Status::FT_DEVICE_NOT_OPENED),
                    x if x == FT_Status::FT_IO_ERROR as $t => Ok(FT_Status::FT_IO_ERROR),
                    x if x == FT_Status::FT_INSUFFICIENT_RESOURCES as $t => Ok(FT_Status::FT_INSUFFICIENT_RESOURCES),
                    x if x == FT_Status::FT_INVALID_PARAMETER as $t => Ok(FT_Status::FT_INVALID_PARAMETER),
                    x if x == FT_Status::FT_INVALID_BAUD_RATE as $t => Ok(FT_Status::FT_INVALID_BAUD_RATE),
                    x if x == FT_Status::FT_DEVICE_NOT_OPENED_FOR_ERASE as $t => Ok(FT_Status::FT_DEVICE_NOT_OPENED_FOR_ERASE),
                    x if x == FT_Status::FT_FAILED_TO_WRITE_DEVICE as $t => Ok(FT_Status::FT_FAILED_TO_WRITE_DEVICE),
                    x if x == FT_Status::FT_EEPROM_READ_FAILED as $t => Ok(FT_Status::FT_EEPROM_READ_FAILED),
                    x if x == FT_Status::FT_EEPROM_WRITE_FAILED as $t => Ok(FT_Status::FT_EEPROM_WRITE_FAILED),
                    x if x == FT_Status::FT_EEPROM_ERASE_FAILED as $t => Ok(FT_Status::FT_EEPROM_ERASE_FAILED),
                    x if x == FT_Status::FT_EEPROM_NOT_PRESENT as $t => Ok(FT_Status::FT_EEPROM_NOT_PRESENT),
                    x if x == FT_Status::FT_EEPROM_NOT_PROGRAMMED as $t => Ok(FT_Status::FT_EEPROM_NOT_PROGRAMMED),
                    x if x == FT_Status::FT_INVALID_ARGS as $t => Ok(FT_Status::FT_INVALID_ARGS),
                    x if x == FT_Status::FT_NOT_SUPPORTED as $t => Ok(FT_Status::FT_NOT_SUPPORTED),
                    x if x == FT_Status::FT_NO_MORE_ITEMS as $t => Ok(FT_Status::FT_NO_MORE_ITEMS),
                    x if x == FT_Status::FT_TIMEOUT as $t => Ok(FT_Status::FT_TIMEOUT),
                    x if x == FT_Status::FT_OPERATION_ABORTED as $t => Ok(FT_Status::FT_OPERATION_ABORTED),
                    x if x == FT_Status::FT_RESERVED_PIPE as $t => Ok(FT_Status::FT_RESERVED_PIPE),
                    x if x == FT_Status::FT_INVALID_CONTROL_REQUEST_DIRECTION as $t => Ok(FT_Status::FT_INVALID_CONTROL_REQUEST_DIRECTION),
                    x if x == FT_Status::FT_INVALID_CONTROL_REQUEST_TYPE as $t => Ok(FT_Status::FT_INVALID_CONTROL_REQUEST_TYPE),
                    x if x == FT_Status::FT_IO_PENDING as $t => Ok(FT_Status::FT_IO_PENDING),
                    x if x == FT_Status::FT_IO_INCOMPLETE as $t => Ok(FT_Status::FT_IO_INCOMPLETE),
                    x if x == FT_Status::FT_HANDLE_EOF as $t => Ok(FT_Status::FT_HANDLE_EOF),
                    x if x == FT_Status::FT_BUSY as $t => Ok(FT_Status::FT_BUSY),
                    x if x == FT_Status::FT_NO_SYSTEM_RESOURCES as $t => Ok(FT_Status::FT_NO_SYSTEM_RESOURCES),
                    x if x == FT_Status::FT_DEVICE_LIST_NOT_READY as $t => Ok(FT_Status::FT_DEVICE_LIST_NOT_READY),
                    x if x == FT_Status::FT_DEVICE_NOT_CONNECTED as $t => Ok(FT_Status::FT_DEVICE_NOT_CONNECTED),
                    x if x == FT_Status::FT_INCORRECT_DEVICE_PATH as $t => Ok(FT_Status::FT_INCORRECT_DEVICE_PATH),
                    x if x == FT_Status::FT_OTHER_ERROR as $t => Ok(FT_Status::FT_OTHER_ERROR),
                    _ => Err("{v} is not a valid FT_Status enum value"),
                }
            }
        }
    };
}
//create_ft_status_tryfrom!(i32);
create_ft_status_tryfrom!(FT_STATUS);

impl From<_FT_STATUS> for FT_STATUS {
    fn from(value: _FT_STATUS) -> Self {
        value as FT_STATUS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ft_status() {
        let status: FT_STATUS = 0;
        assert_eq!(FT_Status::try_from(status), Ok(FT_Status::FT_OK));

        let status: FT_STATUS = FT_Status::FT_OK.try_into().unwrap();
        assert_eq!(status, 0);

        assert_eq!(FT_Status::try_from(99999).is_err(), true);
    }
}
