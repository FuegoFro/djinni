// AUTOGENERATED FILE - DO NOT MODIFY!
// This file generated by Djinni from client_interface.djinni

use support_lib::support::JType;
use support_lib::jni_ffi::{JNIEnv, jobject, jclass};
use generated_rust_jni;

impl JType for Arc<Box<ClientInterface>> {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        Arc::new(Box::new(ClientInterfaceJavaProxy { javaRef: j }))
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

struct ClientInterfaceJavaProxy {
    javaRef: jobject
}

impl ClientInterface for ClientInterfaceJavaProxy {
    fn get_record(&self, r_record_id: i64, r_utf8string: String, r_misc: Option<String>) -> ClientReturnedRecord {
        let class = support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/ClientInterface");
        let jmethod = support_lib::support::get_method(jni_env, class, "getRecord", "(JLjava/lang/String;Ljava/lang/String;)Lcom/dropbox/djinni/test/ClientReturnedRecord;");
        // TODO(rustgen): handle local refs correctly
        let jret = jni_invoke!(jni_env, CallObjectMethod, self.javaRef, jmethod,
                    i64::from_rust(jni_env, r_record_id),
                    String::from_rust(jni_env, r_utf8string),
                    Option::<String>::from_rust(jni_env, r_misc));
        ClientReturnedRecord::to_rust(jni_env, jret)
    }
    fn identifier_check(&self, r_data: Box<[u8]>, r_r: i32, r_jret: i64) -> f64 {
        let class = support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/ClientInterface");
        let jmethod = support_lib::support::get_method(jni_env, class, "identifierCheck", "([BIJ)D");
        // TODO(rustgen): handle local refs correctly
        let jret = jni_invoke!(jni_env, CallDoubleMethod, self.javaRef, jmethod,
                    Box<[u8]>::from_rust(jni_env, r_data),
                    i32::from_rust(jni_env, r_r),
                    i64::from_rust(jni_env, r_jret));
        f64::to_rust(jni_env, jret)
    }
    fn return_str(&self) -> String {
        let class = support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/ClientInterface");
        let jmethod = support_lib::support::get_method(jni_env, class, "returnStr", "()Ljava/lang/String;");
        // TODO(rustgen): handle local refs correctly
        let jret = jni_invoke!(jni_env, CallObjectMethod, self.javaRef, jmethod);
        String::to_rust(jni_env, jret)
    }
}
