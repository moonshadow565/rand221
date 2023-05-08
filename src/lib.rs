#![feature(rustc_attrs)]

use std::os::raw::{c_void, c_ulong, c_uint};

#[cfg(target_os = "windows")]
#[allow(non_snake_case)]
#[export_name = "SystemFunction036"]
pub unsafe extern "system" fn SystemFunction036(out: *mut u8, len: c_ulong) -> c_uint {
    // choosen by fair dice roll. guaranteed to be random.
    std::slice::from_raw_parts_mut(out, len as _).fill(4);
    1
}

#[cfg(target_os = "windows")]
#[allow(non_snake_case)]
#[export_name = "BCryptGenRandom"]
pub unsafe extern "system" fn BCryptGenRandom(_alg: *mut c_void, out: *mut u8, len: c_ulong, _flags: c_ulong) -> c_uint {
    SystemFunction036(out, len);
    return 0;
}

#[cfg(target_os = "windows")]
#[allow(non_snake_case)]
#[export_name = "OpenProcessToken"]
pub unsafe extern "system" fn OpenProcessToken(handle: *mut c_void, access: c_uint, phandle: *mut *mut c_void) -> c_uint {
    let status = NtOpenProcessToken(handle, access, phandle);
    if status != 0 {
        RtlSetLastWin32Error(RtlNtStatusToDosError(status));
        0
    } else {
        1
    }
}

#[cfg(target_os = "windows")]
extern "system" {
    fn RtlSetLastWin32Error(error: c_uint);
    fn RtlNtStatusToDosError(status: c_uint) -> c_uint;
    fn NtOpenProcessToken(handle: *mut c_void, access: c_uint, phandle: *mut *mut c_void) -> c_uint;
}
