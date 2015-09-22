// AUTOGENERATED FILE - DO NOT MODIFY!
// This file generated by Djinni from client_interface.djinni

use support_lib::support::jni_get_thread_env;
use support_lib::support::get_class;
use support_lib::support::get_field;
use support_lib::jni_ffi::jobject;
use generated_rust_jni;
use support_lib::support::ForVariadic;
use std::sync::Arc;
use generated_rust::client_returned_record::ClientReturnedRecord;
use std::boxed::Box;
use support_lib::support::RustProxyable;
use support_lib::support::JType;
use support_lib::support::get_method;
use support_lib::jni_ffi::JNIEnv;
use support_lib::jni_ffi::jstring;
use support_lib::support::GlobalRef;
use support_lib::jni_ffi::jclass;
use support_lib::jni_ffi::jlong;
use std::mem;
use generated_rust::client_interface::ClientInterface;

// TODO(rustgen): correct strong/weak Java references
// TODO(rustgen): cache the proxies
// TODO(rustgen): look into using catch_panic
impl JType for Arc<Box<ClientInterface>> {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        Arc::new(Box::new(ClientInterfaceJavaProxy { java_ref: GlobalRef::new(jni_env, j) }))
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self) -> Self::JniType {
        if let Some(proxy) = r.downcast_ref::<ClientInterfaceJavaProxy>() {
            return proxy.java_ref.get();
        }
        panic!("Expected to get a ClientInterfaceJavaProxy passed in from Java")
    }

    fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> Self {
        Self::to_rust(jni_env, j)
    }

    fn from_rust_boxed(jni_env: *mut JNIEnv, r: Self) -> jobject {
        Self::from_rust(jni_env, r)
    }
}

struct ClientInterfaceJavaProxy {
    java_ref: GlobalRef
}

impl ClientInterface for ClientInterfaceJavaProxy {
    fn get_record(&self, r_record_id: i64, r_utf8string: String, r_misc: Option<String>) -> ClientReturnedRecord {
        let jni_env = jni_get_thread_env();
        // TODO(rustgen): local scope
        // TODO(rustgen): use helper to cache class object and method IDs
        let class = get_class(jni_env, "com/dropbox/djinni/test/ClientInterface");
        let jmethod = get_method(jni_env, class, "getRecord", "(JLjava/lang/String;Ljava/lang/String;)Lcom/dropbox/djinni/test/ClientReturnedRecord;");
        // TODO(rustgen): handle local refs correctly
        let jret = jni_invoke!(jni_env, CallObjectMethod, self.java_ref.get(), jmethod,
                    i64::from_rust(jni_env, r_record_id).for_variadic(),
                    String::from_rust(jni_env, r_utf8string).for_variadic(),
                    Option::<String>::from_rust(jni_env, r_misc).for_variadic());
        ClientReturnedRecord::to_rust(jni_env, jret)
    }
    fn return_str(&self) -> String {
        let jni_env = jni_get_thread_env();
        // TODO(rustgen): local scope
        // TODO(rustgen): use helper to cache class object and method IDs
        let class = get_class(jni_env, "com/dropbox/djinni/test/ClientInterface");
        let jmethod = get_method(jni_env, class, "returnStr", "()Ljava/lang/String;");
        // TODO(rustgen): handle local refs correctly
        let jret = jni_invoke!(jni_env, CallObjectMethod, self.java_ref.get(), jmethod);
        String::to_rust(jni_env, jret)
    }
}
