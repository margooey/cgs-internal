use crate::types::CGSConnectionID;

#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    unsafe fn _CGSDefaultConnection() -> CGSConnectionID;
}

pub fn default_connection_id() -> CGSConnectionID {
    unsafe { _CGSDefaultConnection() }
}