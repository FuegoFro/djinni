// AUTOGENERATED FILE - DO NOT MODIFY!
// This file generated by Djinni from derivings.djinni

#[macro_use(jni_invoke)]
use support_lib;
use support_lib::support::{JType, ForVaridaic};
use support_lib::jni_ffi::{JNIEnv, jobject};

impl JType for ::generated_rust::record_with_nested_derivings::RecordWithNestedDerivings {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        // TODO(rustgen): have a local scope here
        // TODO(rustgen): use a helper to get the class/methods so they're cached
        let class = support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/RecordWithNestedDerivings");
        let field_key = support_lib::support::get_field(jni_env, class, "mKey", "I");
        let field_rec = support_lib::support::get_field(jni_env, class, "mRec", "Lcom/dropbox/djinni/test/RecordWithDerivings;");

        assert!(j != 0 as jobject);
        ::generated_rust::record_with_nested_derivings::RecordWithNestedDerivings {
            key: i32::to_rust(jni_env, jni_invoke!(jni_env, GetIntField, j, field_key)),
            rec: RecordWithDerivings::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_rec)),
        }
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self) {
        // TODO(rustgen): cache the class/methods
        // TODO(rustgen): class object should have a ref around it
        let class = support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/RecordWithNestedDerivings");
        let jconstructor = support_lib::support::get_method(jni_env, class, "<init>", "(ILcom/dropbox/djinni/test/RecordWithDerivings;)V");

        // TODO(rustgen): handle local refs correctly
        jni_invoke!(jni_env, NewLocalRef, jni_invoke!(jni_env, NewObject, class, jconstructor,
                                                      i32::from_rust(jni_env, r.key).for_variadic(),
                                                      RecordWithDerivings::from_rust(jni_env, r.rec).for_variadic()))
    }

    fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> Self {
        Self::to_rust(jni_env, j)
    }

    fn from_rust_boxed(jni_env: *mut JNIEnv, r: Self) -> jobject {
        Self::from_rust(jni_env, r)
    }
}
