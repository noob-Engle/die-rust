extern crate clap;
extern crate widestring;
extern crate winapi;

use std::ffi::CString;
use widestring::U16CString;
use winapi::um::libloaderapi::LoadLibraryA;
use winapi::um::libloaderapi::GetProcAddress;
use winapi::um::winnt::LPCSTR;
use std::os::raw::c_char;

pub const DIE_SHOWERRORS: u32 = 0x00000001;
pub const DIE_SHOWOPTIONS: u32 = 0x00000002;
pub const DIE_SHOWVERSION: u32 = 0x00000004;
pub const DIE_SHOWENTROPY: u32 = 0x00000008;
pub const DIE_SINGLELINEOUTPUT: u32 = 0x00000010;
pub const DIE_SHOWFILEFORMATONCE: u32 = 0x00000020;

type DieScanW = unsafe extern "stdcall" fn(*const u16, *mut c_char, i32, u32) -> i32;

pub fn die_scan(file_name: &str, flags: u32) -> Result<String, String> {
    unsafe {
        let dll_name = CString::new("diedll.dll").unwrap();
        let library = LoadLibraryA(dll_name.as_ptr() as LPCSTR);
        if library.is_null() {
            return Err(format!("Failed to load diedll.dll. Ensure the file exists and is in the correct path."));
        }

        let proc_name = CString::new("_DIE_scanW@16").unwrap();
        let proc_addr = GetProcAddress(library, proc_name.as_ptr() as LPCSTR);
        if proc_addr.is_null() {
            return Err(format!("Failed to find _DIE_scanW@16. Ensure the function exists in the DLL."));
        }
        let proc_die_scan_w: DieScanW = std::mem::transmute(proc_addr);

        let u16_file_name = U16CString::from_str(file_name).unwrap();
        let mut buf_size_incr = 1024;
        let mut buf_size = buf_size_incr;
        let mut res = String::new();

        loop {
            let mut buf = vec![0u8; buf_size as usize];
            let ret = proc_die_scan_w(u16_file_name.as_ptr(), buf.as_mut_ptr() as *mut c_char, buf.len() as i32, flags);

            if ret == buf_size as i32 {
                buf_size += buf_size_incr;
                continue;
            }

            res = uint8_to_string(&buf);
            break;
        }

        Ok(res)
    }
}

fn uint8_to_string(arr: &[u8]) -> String {
    arr.iter().take_while(|&&c| c != 0).map(|&c| c as char).collect()
}
