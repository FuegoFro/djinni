use std::any::Any;
use std::boxed::Box;
use std::sync::Arc;
use support_lib::support::JType;
use support_lib::jni_ffi::{JNIEnv, jobject};
use generated_rust::user_token::UserToken;

impl JType for Arc<Box<UserToken>> {
    type JniType = jobject;

    fn to_rust(_jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        Arc::new(Box::new(UserTokenJavaProxy {
            java_ref: j,
        }))
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self) -> Self::JniType {
        // let any_ref: &Any = r.as_any();
        // let user_token_ref: &UserToken = &**r;
        // let any_ref_2 = NativeUserToken::get_proxy_object(user_token_ref);
        // let foo = match any_ref.downcast_ref::<UserTokenJavaProxy>() {
        //     Some(..) => 0,
        //     None => 0,
        // };
        // box_ref.downcast();
        0 as jobject
    }

    boxed_call_through!();
}

struct UserTokenJavaProxy {
    // Todo - have a global ref to the object
    java_ref: jobject,
}

impl UserToken for UserTokenJavaProxy {
    fn whoami(&self) -> String {
        let jni_env = ::support_lib::support::jni_get_thread_env();
        // Todo - local scope
        // Todo - use helper to cache class object and method IDs
        let class = ::support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/UserToken");
        let method = ::support_lib::support::get_method(jni_env, class, "whoami", "()Ljava/lang/String;");
        // Todo - unpack the global ref
        let ret = jni_invoke!(jni_env, CallObjectMethod, self.java_ref, method);
        String::to_rust(jni_env, ret)
    }
}
