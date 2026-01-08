#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use crate::CGError;

pub type CGSConnectionID = u32;

pub const kCGSNullConnectionID: CGSConnectionID = 0;

#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    unsafe fn _CGSDefaultConnection() -> CGSConnectionID;
    unsafe fn CGSMainConnectionID() -> CGSConnectionID;
    unsafe fn CGSDefaultConnectionForThread() -> CGSConnectionID;
    unsafe fn CGSMenuBarExists(cid: CGSConnectionID) -> bool;
    unsafe fn CGSSetLoginwindowConnection(cid: CGSConnectionID) -> CGError;
    unsafe fn CGSDisableUpdate(cid: CGSConnectionID) -> CGError;
    unsafe fn CGSReenableUpdate(cid: CGSConnectionID) -> CGError;
    unsafe fn CGSRegisterForNewConnectionNotification(
        proc: CGSNewConnectionNotificationProc,
    ) -> CGError;
    unsafe fn CGSRemoveNewConnectionNotification(proc: CGSNewConnectionNotificationProc)
    -> CGError;
    unsafe fn CGSRegisterForConnectionDeathNotification(
        proc: CGSConnectionDeathNotificationProc,
    ) -> CGError;
    unsafe fn CGSRemoveConnectionDeathNotification(
        proc: CGSConnectionDeathNotificationProc,
    ) -> CGError;
}

/// DOCUMENTATION PENDING - verify this is Leopard only!
/// AVAILABLE_MAC_OS_X_VERSION_10_5_AND_LATER
pub fn set_login_window_connection(cid: CGSConnectionID) -> CGError {
    unsafe { CGSSetLoginwindowConnection(cid) }
}

/// Enables or disables updates on a connection. The WindowServer will forcibly reenable updates after 1 second.
pub fn disable_update(cid: CGSConnectionID) -> CGError {
    unsafe { CGSDisableUpdate(cid) }
}
pub fn reenable_update(cid: CGSConnectionID) -> CGError {
    unsafe { CGSReenableUpdate(cid) }
}

/// Is there a menubar associated with this connection?
pub fn menu_bar_exists(cid: CGSConnectionID) -> bool {
    unsafe { CGSMenuBarExists(cid) }
}

/// Registers or removes a function to get notified when a connection is created. Only gets notified for connections created in the current application.
type CGSNewConnectionNotificationProc = unsafe extern "C" fn(cid: CGSConnectionID);
pub fn new_connection_notification_proc(proc: CGSNewConnectionNotificationProc) -> CGError {
    unsafe { CGSRegisterForNewConnectionNotification(proc) }
}
pub fn remove_new_connection_notification(proc: CGSNewConnectionNotificationProc) -> CGError {
    unsafe { CGSRemoveNewConnectionNotification(proc) }
}

/// Registers or removes a function to get notified when a connection is released. Only gets notified for connections created in the current application.
type CGSConnectionDeathNotificationProc = unsafe extern "C" fn(cid: CGSConnectionID);
pub fn register_for_connection_death_notification(
    proc: CGSConnectionDeathNotificationProc,
) -> CGError {
    unsafe { CGSRegisterForConnectionDeathNotification(proc) }
}
pub fn remove_connection_death_notification(proc: CGSConnectionDeathNotificationProc) -> CGError {
    unsafe { CGSRemoveConnectionDeathNotification(proc) }
}

/// Gets the default connection for this process. `CGSMainConnectionID` is just a more modern name. */
pub fn default_connection_id() -> CGSConnectionID {
    unsafe { _CGSDefaultConnection() }
}
pub fn main_connection_id() -> CGSConnectionID {
    unsafe { CGSMainConnectionID() }
}

/// Gets the default connection for the current thread. */
pub fn default_connection_for_thread() -> CGSConnectionID {
    unsafe { CGSDefaultConnectionForThread() }
}
