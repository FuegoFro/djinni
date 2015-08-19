// AUTOGENERATED FILE - DO NOT MODIFY!
// This file generated by Djinni from set.djinni

#[macro_use(jni_invoke)]
use support_lib;
use support_lib::support::{JType, ForVaridaic};
use support_lib::jni_ffi::{JNIEnv, jobject};

impl JType for ::generated_rust::set_record::SetRecord {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        // TODO(rustgen): have a local scope here
        // TODO(rustgen): use a helper to get the class/methods so they're cached
        let class = ::support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/SetRecord");
        let field_set = ::support_lib::support::get_field(jni_env, class, "mSet", "Ljava/util/HashSet;");
        let field_iset = ::support_lib::support::get_field(jni_env, class, "mIset", "Ljava/util/HashSet;");

        assert!(j != 0 as jobject);
        ::generated_rust::set_record::SetRecord {
            set: HashSet::<String>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_set)),
            iset: HashSet::<i32>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_iset)),
        }
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self) {
        // TODO(rustgen): cache the class/methods
        // TODO(rustgen): class object should have a ref around it
        let class = ::support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/SetRecord");
        let jconstructor = ::support_lib::support::get_method(jni_env, class, "<init>", "(Ljava/util/HashSet;Ljava/util/HashSet;)V");

        // TODO(rustgen): handle local refs correctly
        jni_invoke!(jni_env, NewLocalRef, jni_invoke!(jni_env, NewObject, class, jconstructor,
                                                      HashSet::<String>::from_rust(jni_env, r.set).for_variadic(),
                                                      HashSet::<i32>::from_rust(jni_env, r.iset).for_variadic()))
    }

    fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> Self {
        Self::to_rust(jni_env, j)
    }

    fn from_rust_boxed(jni_env: *mut JNIEnv, r: Self) -> jobject {
        Self::from_rust(jni_env, r)
    }
}
