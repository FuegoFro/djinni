use std::any::Any;
use std::boxed::Box;
use std::sync::Arc;
use support_lib::support::JType;
use support_lib::jni_ffi::{JNIEnv, jobject};
use generated_rust::user_token::UserToken;

impl JType for Arc<Box<UserToken>> {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        Arc::new(Box::new(UserTokenJavaProxy {
            javaRef: j,
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

// impl NativeUserToken {
//     fn get_proxy_object<T: Any>(obj: &T) -> &Any {
//         obj as &Any
//     }
// }

struct UserTokenJavaProxy {
    javaRef: jobject,
}

impl UserToken for UserTokenJavaProxy {
    fn whoami(&self) -> String {
        "".to_string()
    }
}
