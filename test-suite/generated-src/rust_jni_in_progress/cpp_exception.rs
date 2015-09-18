// AUTOGENERATED FILE - DO NOT MODIFY!
// This file generated by Djinni from exception.djinni

use support_lib::support::jni_get_thread_env;
use support_lib::support::get_class;
use support_lib::support::get_field;
use support_lib::jni_ffi::jobject;
use generated_rust_jni;
use support_lib::support::ForVariadic;
use std::sync::Arc;
use std::boxed::Box;
use support_lib::support::RustProxyable;
use support_lib::support::JType;
use generated_rust::cpp_exception::CppException;
use support_lib::support::get_method;
use support_lib::jni_ffi::jint;
use support_lib::jni_ffi::JNIEnv;
use support_lib::support::GlobalRef;
use support_lib::jni_ffi::jclass;
use support_lib::jni_ffi::jlong;
use std::mem;

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_CppException_get(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = ::cpp_exception::get();
    Arc::<Box<CppException>>::from_rust(jni_env, r)
}

// TODO(rustgen): correct strong/weak Java references
// TODO(rustgen): cache the proxies
// TODO(rustgen): look into using catch_panic
impl JType for Arc<Box<CppException>> {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        let proxy_class = get_class(jni_env, "com/dropbox/djinni/test/CppException$CppProxy");
        let object_class = jni_invoke!(jni_env, GetObjectClass, j);
        let is_proxy = bool::to_rust(jni_env, jni_invoke!(jni_env, IsSameObject, proxy_class, object_class));
        let native_ref_field = get_field(jni_env, proxy_class, "nativeRef", "J");
        let handle = jni_invoke!(jni_env, GetLongField, j, native_ref_field);
        *Self::from_handle(handle)
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self) -> Self::JniType {
        // Is not a Java proxy, need to create a new CppProxy
        // TODO(rustgen) - cache the CppProxys
        let class = ::support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/CppException$CppProxy");
        let constructor = ::support_lib::support::get_method(jni_env, class, "<init>", "(J)V");
        let handle = Self::to_handle(r.clone());
        jni_invoke!(jni_env, NewObject, class, constructor, handle)
    }

    fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> Self {
        Self::to_rust(jni_env, j)
    }

    fn from_rust_boxed(jni_env: *mut JNIEnv, r: Self) -> jobject {
        Self::from_rust(jni_env, r)
    }
}

impl RustProxyable for Arc<Box<CppException>> {
    fn to_handle(self) -> jlong {
        // Convert our box into a pointer, leaving the memory there and not running
        // the destructor on the contents. We can't use Box::into_raw since it's unstable.
        unsafe { mem::transmute(Box::new(self)) }
    }
    fn from_handle(rust_proxy_handle: jlong) -> Box<Self> {
        // Convert our pointer back into a box. We can't use Box::from_raw since it's unstable.
        unsafe { mem::transmute(rust_proxy_handle as *mut Arc<Box<Self>>) }
    }
}

struct CppExceptionCppProxy {
    rust_ref: Arc<Box<CppException>>
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_CppException_00024CppProxy_nativeDestroy(jni_env: *mut JNIEnv, _this: jobject, native_ref: jlong) {
    let _to_delete: Box<Arc<Box<CppException>>> = Arc::<Box<CppException>>::from_handle(native_ref);
    // Let the destructor run on the Box and its Arc when _to_delete goes out of scope.
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_CppException_00024CppProxy_native_1throwAnException(jni_env: *mut JNIEnv, _this: jobject, native_ref: jlong) -> jint {
    let rust_ref: Box<Arc<Box<CppException>>> = Arc::<Box<CppException>>::from_handle(native_ref);
    let r = rust_ref.throw_an_exception();
    i32::from_rust(jni_env, r)
}
