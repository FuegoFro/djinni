// AUTOGENERATED FILE - DO NOT MODIFY!
// This file generated by Djinni from primitive_list.djinni

#[macro_use(jni_invoke)]
use support_lib;
use support_lib::support::{JType, ForVaridaic};
use support_lib::jni_ffi::{JNIEnv, jobject};

impl JType for ::generated_rust::primitive_list::PrimitiveList {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        // TODO(rustgen): have a local scope here
        // TODO(rustgen): use a helper to get the class/methods so they're cached
        let class = support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/PrimitiveList");
        let field_list = support_lib::support::get_field(jni_env, class, "mList", "Ljava/util/ArrayList;");

        assert!(j != 0 as jobject);
        ::generated_rust::primitive_list::PrimitiveList {
            list: Vec::<i64>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_list)),
        }
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self {
        // TODO(rustgen): cache the class/methods
        // TODO(rustgen): class object should have a ref around it
        let class = support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/PrimitiveList");
        let jconstructor = support_lib::support::get_method(jni_env, class, "<init>", "(Ljava/util/ArrayList;)V");

        // TODO(rustgen): handle local refs correctly
        jni_invoke!(jni_env, NewLocalRef, jni_invoke!(jni_env, NewObject, class, jconstructor,
                                                      Vec::<i64>::from_rust(jni_env, r.list).for_variadic()))
    }

    fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> Self {
        Self::to_rust(jni_env, j)
    }

    fn from_rust_boxed(jni_env: *mut JNIEnv, r: Self) -> jobject {
        Self::from_rust(jni_env, r)
    }
}
