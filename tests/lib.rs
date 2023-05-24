#[cfg(test)]
mod tests {
    use libftd3xx_ffi::{FT_GetLibraryVersion, FT_STATUS, DWORD, _FT_STATUS::*};

    #[test]
    fn test_version() {
        let mut version: DWORD = 0;
        let status: FT_STATUS = unsafe { FT_GetLibraryVersion(&mut version) };
        assert_eq!(status, FT_OK as FT_STATUS);

        //let expected_version: u32 = 0x0;
        cfg_if::cfg_if! {
            if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
                // version "1.0.5" is represented as 0x010005
                let expected_version = 0x01_00_05;
            } else if #[cfg(all(target_os = "linux", target_arch = "x86"))] {
                // version "1.0.5" is represented as 0x010005
                let expected_version = 0x01_00_05;
            } else if #[cfg(all(target_os = "windows", target_arch = "x86_64"))] {
                // version "1.3.0.4" is represented as 0x1030004
                let expected_version = 0x1_03_00_04; 
            } else if #[cfg(all(target_os = "windows", target_arch = "x86"))] {
                let expected_version = 0x1_03_00_04; 
            } else if #[cfg(all(target_os = "linux", target_arch = "arm"))] {
                todo!()
                let expected_version = 0x0;
            } else if #[cfg(all(target_os = "linux", target_arch = "aarch64"))] {
                todo!()
                let expected_version = 0x0;
            } else if #[cfg(all(target_os = "macos", target_arch = "x86_64"))] {
                todo!()
                let expected_version = 0x0;
            } else if #[cfg(all(target_os = "macos", target_arch = "aarch64"))] {
                todo!()
                let expected_version = 0x0;
            } else {
                std::compile_error!("pre-generated bindings are not avaliable for your target");
            }
        };
        assert_eq!(version, expected_version);

        /*
        extern "C" {
            pub fn FT_GetDriverVersion(ftHandle: FT_HANDLE, lpdwVersion: LPDWORD) -> FT_STATUS;
        }
        */
    }
}
