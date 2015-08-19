use libc::c_void;
use support_lib;
use support_lib::jni_ffi::{JavaVM, jint, JNI_VERSION_1_6};

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn JNI_OnLoad(jvm: *mut JavaVM, _reserved: *mut c_void) -> jint {
    support_lib::support::jni_init(jvm);
    JNI_VERSION_1_6
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn JNI_OnUnload(_jvm: *mut JavaVM, _reserved: *mut c_void) {
    support_lib::support::jni_shutdown();
}
