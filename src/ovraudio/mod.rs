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

pub type Context = sys::ovrAudioContext;

pub fn create_context() -> Context {
    unsafe {
        let config: *const sys::ovrAudioContextConfiguration = &sys::ovrAudioContextConfiguration {
            acc_Size: std::mem::size_of::<sys::ovrAudioContextConfiguration>() as u32,
            acc_MaxNumSources: 16,
            acc_SampleRate: 16000,
            acc_BufferLength: 160,
        };
        let mut context: sys::ovrAudioContext = std::ptr::null_mut();
        sys::ovrAudio_CreateContext(&mut context, config);
        for sound in 0..16 {
            sys::ovrAudio_SetAudioSourceRange(context, sound, 0.0, 1000.0);
            sys::ovrAudio_SetAudioSourceAttenuationMode(
                context,
                sound,
                sys::ovrAudioSourceAttenuationMode::ovrAudioSourceAttenuationMode_InverseSquare,
                0.0,
            );
        }
        context
    }
}

pub fn destroy_context(context: Context) {
    unsafe {
        sys::ovrAudio_DestroyContext(context);
    }
}

pub fn set_range(context: Context, sound: i32, range_min: f32, range_max: f32) {
    unsafe {
        sys::ovrAudio_SetAudioSourceRange(context, sound, range_min, range_max);
    }
}

pub fn set_pos(context: Context, sound: i32, x: f32, y: f32, z: f32) {
    unsafe {
        sys::ovrAudio_SetAudioSourcePos(context, sound, x, y, z);
    }
}

pub fn spatializeMonoSourceInterleaved(context: Context, sound: i32, src: Vec<f32>) -> [f32; 320]{
    unsafe {
        let status = &mut 0;
        let mut dst = [0f32; 320];
        let dst_buf : *mut f32 = dst.as_mut_ptr();
        let src_buf : *const f32 = src.as_ptr();
        sys::ovrAudio_SpatializeMonoSourceInterleaved(
            context,
            sound,
            status,
            dst_buf,
            src_buf,
        );
        dst
    }
}
