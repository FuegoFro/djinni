// AUTOGENERATED FILE - DO NOT MODIFY!
// This file generated by Djinni from nested_collection.djinni

#[macro_use]
use support_lib;
use support_lib::support::JType;
use support_lib::jni_ffi::{JNIEnv, jobject};
use generated_rust::nested_collection::NestedCollection;

pub struct NativeNestedCollection;
impl JType for NativeNestedCollection
{
    type RustType = NestedCollection;
    type JniType = jobject;

    #[allow(non_snake_case)]
    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self::RustType {
        // TODO(rustgen): have a local scope here
        // TODO(rustgen): use a helper to get the class/methods so they're cached
        let class = support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/NestedCollection");
        let field_mSetList = support_lib::support::get_method(jni_env, class, "mSetList", "Ljava/util/ArrayList;");

        assert!(j != 0 as jobject);
        NestedCollection {
            set_list: support_lib::support::List::<support_lib::support::Set::<support_lib::support::String>>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_mSetList)),
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
