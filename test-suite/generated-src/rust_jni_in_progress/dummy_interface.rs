// AUTOGENERATED FILE - DO NOT MODIFY!
// This file generated by Djinni from interface_nullity.djinni

use support_lib::support::JType;
use support_lib::jni_ffi::{JNIEnv, jobject, jclass};
use generated_rust_jni;

impl JType for Arc<Box<DummyInterface>> {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        ::std::sync::Arc::new(::std::boxed::Box::new(DummyInterfaceJavaProxy { javaRef: j }))
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self) {
        // TODO(rustgen): this
        0 as jobject
    }

    fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> Self {
        Self::to_rust(jni_env, j)
    }

    fn from_rust_boxed(jni_env: *mut JNIEnv, r: Self) -> jobject {
        Self::from_rust(jni_env, r)
    }
}

struct DummyInterfaceJavaProxy {
    javaRef: jobject
}

impl DummyInterface for DummyInterfaceJavaProxy {
}
struct DummyInterfaceCppProxy {
    rustRef: Arc<Box<DummyInterface>>
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_DummyInterface_00024CppProxy_nativeDestroy(jni_env: *mut JNIEnv, _this: jobject, nativeRef: jlong) {
    // TODO(rustgen): remove proxy from cache
}
