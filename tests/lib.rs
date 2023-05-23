#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let mut version: LPDWORD = 0;
        let status: FT_STATUS = unsafe {
            FT_GetLibraryVersion(&mut version)
        };
        assert_eq(status, FT_OK);

        // version "1.0.5" is represented as 0x010005
        assert_eq!(version, 0x01_00_05);

        todo!()
        /*
        extern "C" {
            pub fn FT_GetDriverVersion(ftHandle: FT_HANDLE, lpdwVersion: LPDWORD) -> FT_STATUS;
        }
        */
    }
}
