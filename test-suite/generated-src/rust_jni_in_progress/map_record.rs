// AUTOGENERATED FILE - DO NOT MODIFY!
// This file generated by Djinni from map.djinni

#[macro_use(jni_invoke)]
use support_lib;
use support_lib::support::{JType, ForVaridaic};
use support_lib::jni_ffi::{JNIEnv, jobject};

impl JType for ::generated_rust::map_record::MapRecord {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        // TODO(rustgen): have a local scope here
        // TODO(rustgen): use a helper to get the class/methods so they're cached
        let class = support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/MapRecord");
        let field_map = support_lib::support::get_field(jni_env, class, "mMap", "Ljava/util/HashMap;");
        let field_imap = support_lib::support::get_field(jni_env, class, "mImap", "Ljava/util/HashMap;");

        assert!(j != 0 as jobject);
        ::generated_rust::map_record::MapRecord {
            map: HashMap::<String, i64>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_map)),
            imap: HashMap::<i32, i32>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_imap)),
        }
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self) {
        // TODO(rustgen): cache the class/methods
        // TODO(rustgen): class object should have a ref around it
        let class = support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/MapRecord");
        let jconstructor = support_lib::support::get_method(jni_env, class, "<init>", "(Ljava/util/HashMap;Ljava/util/HashMap;)V");

        // TODO(rustgen): handle local refs correctly
        jni_invoke!(jni_env, NewLocalRef, jni_invoke!(jni_env, NewObject, class, jconstructor,
                                                      HashMap::<String, i64>::from_rust(jni_env, r.map).for_variadic(),
                                                      HashMap::<i32, i32>::from_rust(jni_env, r.imap).for_variadic()))
    }

    fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> Self {
        Self::to_rust(jni_env, j)
    }

    fn from_rust_boxed(jni_env: *mut JNIEnv, r: Self) -> jobject {
        Self::from_rust(jni_env, r)
    }
}
