#[cfg(test)]
mod tests {
    use libftd3xx_ffi::{FT_GetLibraryVersion, FT_STATUS, DWORD, _FT_STATUS::*};

    #[test]
    fn test_version() {
        let mut version: DWORD = 0;
        let status: FT_STATUS = unsafe { FT_GetLibraryVersion(&mut version) };
        assert_eq!(status, FT_OK as FT_STATUS);

        // version "1.0.5" is represented as 0x010005
        assert_eq!(version, 0x01_00_05);

        /*
        extern "C" {
            pub fn FT_GetDriverVersion(ftHandle: FT_HANDLE, lpdwVersion: LPDWORD) -> FT_STATUS;
        }
        */
    }
}
