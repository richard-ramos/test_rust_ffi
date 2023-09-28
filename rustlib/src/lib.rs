#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{slice, str};
use libc::*;
include!("./bindings.rs");

unsafe extern "C" fn trampoline<F>(
    data: *const ::std::os::raw::c_char,
    data_len: usize,
    user_data: *mut ::std::os::raw::c_void,
) where
    F: FnMut(&str),
{
    let user_data = &mut *(user_data as *mut F);
    let result =
        str::from_utf8(slice::from_raw_parts(data as *mut u8, data_len)).expect("valid utf8");
    user_data(&result);
}

fn get_trampoline<F>(_closure: &F) -> WakuCallBack
where
    F: FnMut(&str),
{
    trampoline::<F>
}

pub fn version() -> String {
    let mut result: String = Default::default();
    let ok_cb = |v: &str| result = v.to_string();
    unsafe {
        let mut closure = ok_cb;
        let cb = get_trampoline(&closure);
        waku_version(cb, &mut closure as *mut _ as *mut c_void);
    }
    result
}