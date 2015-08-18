// AUTOGENERATED FILE - DO NOT MODIFY!
// This file generated by Djinni from constants.djinni

#[macro_use]
use support_lib;
use support_lib::support::JType;
use support_lib::jni_ffi::{JNIEnv, jobject};
use generated_rust::constants::Constants;

pub struct NativeConstants;
impl JType for NativeConstants
{
    type RustType = Constants;
    type JniType = jobject;

    #[allow(non_snake_case)]
    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self::RustType {
        // TODO(rustgen): have a local scope here
        // TODO(rustgen): use a helper to get the class/methods so they're cached
        let class = support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/Constants");
        let field_mSomeInteger = support_lib::support::get_method(jni_env, class, "mSomeInteger", "I");
        let field_mSomeString = support_lib::support::get_method(jni_env, class, "mSomeString", "Ljava/lang/String;");

        assert!(j != 0 as jobject);
        Constants {
            some_integer: support_lib::support::I32::to_rust(jni_env, jni_invoke!(jni_env, GetIntField, j, field_mSomeInteger)),
            some_string: support_lib::support::String::to_rust(jni_env, (jstring)jni_invoke!(jni_env, GetObjectField, j, field_mSomeString)),
        }
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self::RustType) -> Self::JniType {
        // TODO(rustgen): translate backwards
        0 as jobject
    }

    fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> Self::RustType {
        Self::to_rust(jni_env, j)
    }

    fn from_rust_boxed(jni_env: *mut JNIEnv, r: Self::RustType) -> jobject {
        Self::from_rust(jni_env, r)
    }
}
