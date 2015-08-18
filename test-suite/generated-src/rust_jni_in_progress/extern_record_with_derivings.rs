// AUTOGENERATED FILE - DO NOT MODIFY!
// This file generated by Djinni from yaml-test.djinni

#[macro_use]
use support_lib;
use support_lib::support::JType;
use support_lib::jni_ffi::{JNIEnv, jobject};
use generated_rust::extern_record_with_derivings::ExternRecordWithDerivings;

pub struct NativeExternRecordWithDerivings;
impl JType for NativeExternRecordWithDerivings
{
    type RustType = ExternRecordWithDerivings;
    type JniType = jobject;

    #[allow(non_snake_case)]
    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self::RustType {
        // TODO(rustgen): have a local scope here
        // TODO(rustgen): use a helper to get the class/methods so they're cached
        let class = support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/ExternRecordWithDerivings");
        let field_mMember = support_lib::support::get_method(jni_env, class, "mMember", "Lcom/dropbox/djinni/test/RecordWithDerivings;");
        let field_mE = support_lib::support::get_method(jni_env, class, "mE", "Lcom/dropbox/djinni/test/Color;");

        assert!(j != 0 as jobject);
        ExternRecordWithDerivings {
            // would grab field_mMember, but MExtern not implemented
            // would grab field_mE, but MExtern not implemented
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
