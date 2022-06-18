use std::os::raw::c_void;

extern "C" {
    fn win_crypt_protect(data: *const u8, len: u32, protect: bool) -> CryptResult;
    fn win_crypt_free(data: *mut c_void);
}

#[repr(C)]
#[derive(Debug, Clone)]
struct CryptResult {
    pub data: *mut u8,
    pub len: u32,
}

impl Drop for CryptResult {
    fn drop(&mut self) {
        unsafe {
            win_crypt_free(self.data as _);
        }
    }
}

pub fn crypt(data: &[u8], encrypt: bool) -> Result<Vec<u8>, ()> {
    let result = unsafe { win_crypt_protect(data.as_ptr(), data.len() as _, encrypt) };
    if !result.data.is_null() && result.len > 0 {
        unsafe {
            return Ok(std::slice::from_raw_parts(result.data as _, result.len as _).to_vec());
        }
    }

    Err(())
}
