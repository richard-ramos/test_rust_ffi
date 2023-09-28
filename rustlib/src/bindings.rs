pub type WakuCallBack = unsafe extern "C" fn(
    msg: *const c_char,
    len_0: usize,
    user_data: *mut c_void,
);

extern "C" {
    pub fn waku_version(ok_cb: WakuCallBack, user_data: *mut c_void);
}
