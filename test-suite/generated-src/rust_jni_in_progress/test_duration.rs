// AUTOGENERATED FILE - DO NOT MODIFY!
// This file generated by Djinni from duration.djinni

use support_lib::support::JType;
use support_lib::jni_ffi::{JNIEnv, jobject, jclass};
use generated_rust_jni;
impl JType for Arc<Box<TestDuration>> {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        Arc::new(Box::new(TestDurationJavaProxy { javaRef: j }))
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self {
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

