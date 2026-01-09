/*
 * Copyright (C) 2007-2008 Alacatia Labs
 *
 * This software is provided 'as-is', without any express or implied
 * warranty.  In no event will the authors be held liable for any damages
 * arising from the use of this software.
 *
 * Permission is granted to anyone to use this software for any purpose,
 * including commercial applications, and to alter it and redistribute it
 * freely, subject to the following restrictions:
 *
 * 1. The origin of this software must not be misrepresented; you must not
 *    claim that you wrote the original software. If you use this software
 *    in a product, an acknowledgment in the product documentation would be
 *    appreciated but is not required.
 * 2. Altered source versions must be plainly marked as such, and must not be
 *    misrepresented as being the original software.
 * 3. This notice may not be removed or altered from any source distribution.
 *
 * Joe Ranieri joe@alacatia.com
 *
 */

//
//  Updated by Robert Widmann.
//  Copyright © 2015-2016 CodaFi. All rights reserved.
//  Released under the MIT license.
//

// Rust bindings by margooey.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use crate::CGError;
use core_foundation::{base::CFTypeRef, string::CFStringRef};

/// The type of connections to the Window Server.
///
/// Every application is given a singular connection ID through which it can receive and manipulate
/// values, state, notifications, events, etc. in the Window Server.
pub type CGSConnectionID = i32;

#[repr(C)]
pub struct ProcessSerialNumber {
    pub high_long_of_psn: u32,
    pub low_long_of_psn: u32,
}

pub type pid_t = libc::pid_t;

pub const kCGSNullConnectionID: CGSConnectionID = 0;

#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    unsafe fn CGSMainConnectionID() -> CGSConnectionID;
    unsafe fn CGSNewConnection(unused: i32, outConnection: *mut CGSConnectionID) -> CGError;
    unsafe fn CGSReleaseConnection(cid: CGSConnectionID) -> CGError;
    unsafe fn CGSDefaultConnectionForThread() -> CGSConnectionID;
    unsafe fn CGSConnectionGetPID(cid: CGSConnectionID, outPID: *mut pid_t) -> CGError;
    unsafe fn CGSGetConnectionIDForPSN(
        cid: CGSConnectionID,
        psn: *const ProcessSerialNumber,
        outOwnerCID: *mut CGSConnectionID,
    ) -> CGError;
    unsafe fn CGSMenuBarExists(cid: CGSConnectionID) -> bool;
    unsafe fn CGSShutdownServerConnections() -> CGError;
    unsafe fn CGSCopyConnectionProperty(
        cid: CGSConnectionID,
        targetCID: CGSConnectionID,
        key: CFStringRef,
        outValue: *mut CFTypeRef,
    ) -> CGError;
    unsafe fn CGSSetConnectionProperty(
        cid: CGSConnectionID,
        targetCID: CGSConnectionID,
        key: CFStringRef,
        value: CFTypeRef,
    ) -> CGError;
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
    unsafe fn CGSSetOtherUniversalConnection(
        cid: CGSConnectionID,
        otherConnection: CGSConnectionID,
    ) -> CGError;
    unsafe fn CGSSetUniversalOwner(cid: CGSConnectionID) -> CGError;
    unsafe fn CGSSetLoginwindowConnection(cid: CGSConnectionID) -> CGError;
}

/// Gets the default connection for this process.
pub fn main_connection_id() -> CGSConnectionID {
    unsafe { CGSMainConnectionID() }
}

/// Creates a new connection to the Window Server.
pub unsafe fn new_connection(unused: i32, out_connection: *mut CGSConnectionID) -> CGError {
    unsafe { CGSNewConnection(unused, out_connection) }
}

/// Releases a CGSConnection and all CGSWindows owned by it.
pub fn release_connection(cid: CGSConnectionID) -> CGError {
    unsafe { CGSReleaseConnection(cid) }
}

/// Gets the default connection for the current thread.
pub fn default_connection_for_thread() -> CGSConnectionID {
    unsafe { CGSDefaultConnectionForThread() }
}

/// Gets the pid of the process that owns this connection to the Window Server.
pub unsafe fn connection_get_pid(cid: CGSConnectionID, out_pid: *mut pid_t) -> CGError {
    unsafe { CGSConnectionGetPID(cid, out_pid) }
}

/// Gets the connection for the given process serial number.
pub unsafe fn get_connection_id_for_psn(
    cid: CGSConnectionID,
    psn: *const ProcessSerialNumber,
    out_owner_cid: *mut CGSConnectionID,
) -> CGError {
    unsafe { CGSGetConnectionIDForPSN(cid, psn, out_owner_cid) }
}

/// Returns whether the menu bar exists for the given connection ID.
///
/// For the majority of applications, this function should return true.  But at system updates,
/// initialization, and shutdown, the menu bar will be either initially gone then created or
/// hidden and then destroyed.
pub fn menu_bar_exists(cid: CGSConnectionID) -> bool {
    unsafe { CGSMenuBarExists(cid) }
}

/// Closes ALL connections to the Window Server by the current application.
///
/// The application is effectively turned into a Console-based application after the invocation of
/// this method.
pub fn shutdown_server_connections() -> CGError {
    unsafe { CGSShutdownServerConnections() }
}

/// Retrieves the value associated with the given key for the given connection.
///
/// This method is structured so processes can send values through the Window Server to other
/// processes - assuming they know each others connection IDs.  The recommended use case for this
/// function appears to be keeping state around for application-level sub-connections.
pub unsafe fn copy_connection_property(
    cid: CGSConnectionID,
    target_cid: CGSConnectionID,
    key: CFStringRef,
    out_value: *mut CFTypeRef,
) -> CGError {
    unsafe { CGSCopyConnectionProperty(cid, target_cid, key, out_value) }
}

/// Associates a value for the given key on the given connection.
pub unsafe fn set_connection_property(
    cid: CGSConnectionID,
    target_cid: CGSConnectionID,
    key: CFStringRef,
    value: CFTypeRef,
) -> CGError {
    unsafe { CGSSetConnectionProperty(cid, target_cid, key, value) }
}

/// Disables updates on a connection
///
/// Calls to disable updates nest much like `-beginUpdates`/`-endUpdates`.  the Window Server will
/// forcibly reenable updates after 1 second if you fail to invoke `CGSReenableUpdate`.
pub fn disable_update(cid: CGSConnectionID) -> CGError {
    unsafe { CGSDisableUpdate(cid) }
}

/// Re-enables updates on a connection.
///
/// Calls to enable updates nest much like `-beginUpdates`/`-endUpdates`.
pub fn reenable_update(cid: CGSConnectionID) -> CGError {
    unsafe { CGSReenableUpdate(cid) }
}

type CGSNewConnectionNotificationProc = unsafe extern "C" fn(cid: CGSConnectionID);

/// Registers a function that gets invoked when the application's connection ID is created by the
/// Window Server.
pub unsafe fn new_connection_notification_proc(proc: CGSNewConnectionNotificationProc) -> CGError {
    unsafe { CGSRegisterForNewConnectionNotification(proc) }
}

/// Removes a function that was registered to receive notifications for the creation of the
/// application's connection to the Window Server.
pub unsafe fn remove_new_connection_notification(proc: CGSNewConnectionNotificationProc) -> CGError {
    unsafe { CGSRemoveNewConnectionNotification(proc) }
}

type CGSConnectionDeathNotificationProc = unsafe extern "C" fn(cid: CGSConnectionID);

/// Registers a function that gets invoked when the application's connection ID is destroyed -
/// ideally by the Window Server.
///
/// Connection death is supposed to be a fatal event that is only triggered when the application
/// terminates or when you have explicitly destroyed a sub-connection to the Window Server.
pub unsafe fn register_for_connection_death_notification(
    proc: CGSConnectionDeathNotificationProc,
) -> CGError {
    unsafe { CGSRegisterForConnectionDeathNotification(proc) }
}

/// Removes a function that was registered to receive notifications for the destruction of the
/// application's connection to the Window Server.
pub unsafe fn remove_connection_death_notification(proc: CGSConnectionDeathNotificationProc) -> CGError {
    unsafe { CGSRemoveConnectionDeathNotification(proc) }
}

/// Sets a "Universal Owner" for the connection ID.  Currently, that owner is Dock.app, which needs
/// control over the window to provide system features like hiding and showing windows, moving them
/// around, etc.
///
/// Because the Universal Owner owns every window under this connection, it can manipulate them
/// all as it sees fit.  If you can beat the dock, you have total control over the process'
/// connection.
pub fn set_universal_owner(cid: CGSConnectionID) -> CGError {
    unsafe { CGSSetUniversalOwner(cid) }
}

/// Assuming you have the connection ID of the current universal owner, or are said universal owner,
/// allows you to specify another connection that has total control over the application's windows.
pub fn set_other_universal_connection(
    cid: CGSConnectionID,
    other_connection: CGSConnectionID,
) -> CGError {
    unsafe { CGSSetOtherUniversalConnection(cid, other_connection) }
}

/// Sets the given connection ID as the login window connection ID.  Windows for the application are
/// then brought to the fore when the computer logs off or goes to sleep.
///
/// Why this is still here, I have no idea.  Window Server only accepts one process calling this
/// ever.  If you attempt to invoke this after loginwindow does you will be yelled at and nothing
/// will happen.  If you can manage to beat loginwindow, however, you know what they say:
///
///    When you teach a man to phish...
pub fn set_login_window_connection(cid: CGSConnectionID) -> CGError {
    unsafe { CGSSetLoginwindowConnection(cid) }
}
