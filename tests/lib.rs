#[cfg(test)]
mod tests {
    use std::ffi::c_uint;
    use std::ffi::c_void;

    use libftd3xx_ffi::prelude::*;
    use libftd3xx_ffi::DWORD;

    #[test]
    fn test_ft_getlibraryversion() {
        let mut version: DWORD = 0;
        let status: FT_STATUS = unsafe { FT_GetLibraryVersion(&mut version) };
        assert_eq!(status, FT_OK as FT_STATUS);

        //let expected_version: u32 = 0x0;
        cfg_if::cfg_if! {
            if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
                // version "1.0.5" is represented as 0x010005
                // let expected_version = 0x01_00_05;
                // for some reason 1.0.5 reports as 0x1000016
                let expected_version = 0x1_00_00_16;
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
    }

    #[cfg(feature = "hardware_tests")]
    #[test]
    fn test_ft_getdriverversion() {
        let mut version: DWORD = 0;
        let status: FT_STATUS = unsafe { FT_GetDriverVersion(std::ptr::null_mut(), &mut version) };
        assert_eq!(status, FT_OK as FT_STATUS, "FT_GetDriverVersion() failed with errror code {status}");
    }

    
    #[test]
    fn test_ft_listdevices() {
        let mut num_devs: DWORD = 0;
        let status: FT_STATUS = unsafe {
            FT_ListDevices(
                &mut num_devs as *mut c_uint as *mut c_void,
                std::ptr::null_mut(),
                FT_LIST_NUMBER_ONLY,
            )
        };
        assert_eq!(status, FT_OK as FT_STATUS);

        cfg_if::cfg_if! {
            if #[cfg(feature = "hardware_tests")] {
                assert!(num_devs >= 1, "Expected at least one device, found {num_devs}!");
            } else {
                assert_eq!(num_devs, 0, "Expected zero devices, found {num_devs}!");
            }
        }
    }

    #[cfg(feature = "hardware_tests")]
    #[test]
    fn test_ft_create() {
        // lets create the first device
        let mut handle: FT_HANDLE = FT_INVALID_HANDLE as u32 as *mut std::os::raw::c_void;
        let status: FT_STATUS = unsafe {
            let mut open_type = FT_OPEN_BY_INDEX;
            FT_Create(0 as *mut std::os::raw::c_void,
                FT_OPEN_BY_INDEX, //&mut open_type as *mut _ as *mut c_void,
                &mut handle)
        };
        assert_eq!(status, FT_OK as FT_STATUS);
        // close the device
        let status: FT_STATUS = unsafe {
            FT_Close(handle)
        };
        assert_eq!(status, FT_OK as FT_STATUS);
    }
}
