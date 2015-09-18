// AUTOGENERATED FILE - DO NOT MODIFY!
// This file generated by Djinni from client_interface.djinni

#[macro_use(jni_invoke)]
use support_lib;
use support_lib::support::get_class;
use support_lib::support::get_field;
use support_lib::jni_ffi::jobject;
use support_lib::support::ForVariadic;
use generated_rust::client_returned_record::ClientReturnedRecord;
use support_lib::support::JType;
use support_lib::support::get_method;
use support_lib::jni_ffi::JNIEnv;
use support_lib::jni_ffi::jstring;
use support_lib::jni_ffi::jlong;

impl JType for ::generated_rust::client_returned_record::ClientReturnedRecord {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        // TODO(rustgen): have a local scope here
        // TODO(rustgen): use a helper to get the class/methods so they're cached
        let class = get_class(jni_env, "com/dropbox/djinni/test/ClientReturnedRecord");
        let field_record_id = get_field(jni_env, class, "mRecordId", "J");
        let field_content = get_field(jni_env, class, "mContent", "Ljava/lang/String;");
        let field_misc = get_field(jni_env, class, "mMisc", "Ljava/lang/String;");

        assert!(j != 0 as jobject);
        ::generated_rust::client_returned_record::ClientReturnedRecord {
            record_id: i64::to_rust(jni_env, jni_invoke!(jni_env, GetLongField, j, field_record_id)),
            content: String::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_content)),
            misc: Option::<String>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_misc)),
        }
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self) -> Self::JniType {
        // TODO(rustgen): cache the class/methods
        // TODO(rustgen): class object should have a ref around it
        let class = get_class(jni_env, "com/dropbox/djinni/test/ClientReturnedRecord");
        let jconstructor = get_method(jni_env, class, "<init>", "(JLjava/lang/String;Ljava/lang/String;)V");

        // TODO(rustgen): handle local refs correctly
        jni_invoke!(jni_env, NewLocalRef, jni_invoke!(jni_env, NewObject, class, jconstructor,
                                                      i64::from_rust(jni_env, r.record_id).for_variadic(),
                                                      String::from_rust(jni_env, r.content).for_variadic(),
                                                      Option::<String>::from_rust(jni_env, r.misc).for_variadic()))
    }

    fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> Self {
        Self::to_rust(jni_env, j)
    }

    fn from_rust_boxed(jni_env: *mut JNIEnv, r: Self) -> jobject {
        Self::from_rust(jni_env, r)
    }
}
