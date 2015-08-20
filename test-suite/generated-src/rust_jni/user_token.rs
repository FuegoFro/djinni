use std::boxed::Box;
use std::sync::Arc;
use std::mem;
use support_lib::support::JType;
use support_lib::jni_ffi::{JNIEnv, jobject, jlong, jstring};
use generated_rust::user_token::UserToken;

impl JType for Arc<Box<UserToken>> {
    type JniType = jobject;

    fn to_rust(_jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        // Todo - handle unwrapping of Java-side CppProxy
        Arc::new(Box::new(UserTokenJavaProxy {
            java_ref: j,
        }))
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self) -> Self::JniType {
        match r.downcast_ref::<UserTokenJavaProxy>() {
            Some(user_token_java_proxy) => user_token_java_proxy.java_ref,
            None => {
                // Is not a Java proxy, need to create a new CppProxy
                // Todo - cache the CppProxys
                let class = ::support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/UserToken$CppProxy");
                let constructor = ::support_lib::support::get_method(jni_env, class, "<init>", "(J)V");
                // Convert our box into a pointer, leaving the memory there and not running
                // the destructor on the contents. We can't use Box::into_raw since it's unstable.
                let handle: *const Self = unsafe { mem::transmute(Box::new(r.clone())) };
                jni_invoke!(jni_env, NewObject, class, constructor, handle)
            },
        }
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

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_UserToken_00024CppProxy_native_1whoami(jni_env: *mut JNIEnv, _this: jobject, native_ref: jlong) -> jstring {
    // Convert our pointer back into a box. We can't use Box::from_raw since it's unstable.
    let obj: Box<Arc<Box<UserToken>>> = unsafe { mem::transmute(native_ref as *mut Arc<Box<UserToken>>) };
    String::from_rust(jni_env, obj.whoami())
}
