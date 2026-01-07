use std::os::raw::c_int;

pub type CGSConnectionID = c_int;

#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    unsafe fn _CGSDefaultConnection() -> CGSConnectionID;
}

pub fn default_connection_id() -> CGSConnectionID {
    unsafe { _CGSDefaultConnection() }
}