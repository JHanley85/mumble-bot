#![allow(unknown_lints)]
#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

mod sys;
use std;
use libc::c_char;
use std::ffi::CStr;
use std::str;

pub fn get_version() -> String {
    unsafe {
        let mayor: *mut std::os::raw::c_int = &mut 0;
        let minor: *mut std::os::raw::c_int = &mut 0;
        let patch: *mut std::os::raw::c_int = &mut 0;
        let version = sys::ovrAudio_GetVersion(mayor, minor, patch);
        let c_str: &CStr = CStr::from_ptr(version);
        let str_slice: &str = c_str.to_str().unwrap();
        str_slice.to_owned()
    }
}

pub fn create_context() -> () {
    unsafe {
        let context: *mut sys::ovrAudioContext = &mut sys::ovrAudioContextConfig {
            acc_Size: 0,
            acc_MaxNumSources: 1,
            acc_SampleRate: 16000,
            acc_BufferLength: 1024,
        };
        //let config: *const sys::ovrAudioContextConfiguration = std::os::raw::c_void;
        //sys::ovrAudio_CreateConetn
    }
}

// pub fn initialize() -> Result<i32, i32> {
//     unsafe {
//         let res = sys::ovrAudio_Initialize();
//         if res == sys::ovrSuccess as i32 {
//             Ok(sys::ovrSuccess as i32)
//         } else {
//             Err(res)
//         }
//     }
// }
