use std::boxed::Box;
use std::sync::Arc;
use std::mem;
use support_lib::support::{JType, RustProxyable};
use support_lib::jni_ffi::{JNIEnv, jobject, jlong, jstring};
use generated_rust::user_token::UserToken;

// Todo - native destroy
// Todo - correct strong/weak Java references
// Todo(everywhere) - local/global JNI references
// Todo - Cache the proxies
impl JType for Arc<Box<UserToken>> {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        let proxy_class = ::support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/UserToken$CppProxy");
        let object_class = jni_invoke!(jni_env, GetObjectClass, j);
        let is_proxy = bool::to_rust(jni_env, jni_invoke!(jni_env, IsSameObject, proxy_class, object_class));
        if is_proxy {
            let native_ref_field = ::support_lib::support::get_field(jni_env, proxy_class, "nativeRef", "J");
            let handle = jni_invoke!(jni_env, GetLongField, j, native_ref_field);
            // Move the Arc out of its box, destroying the box and returning the Arc on the stack.
            *Self::from_handle(handle)
        } else {
            Arc::new(Box::new(UserTokenJavaProxy {
                java_ref: j,
            }))
        }
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self) -> Self::JniType {
        match r.downcast_ref::<UserTokenJavaProxy>() {
            Some(user_token_java_proxy) => user_token_java_proxy.java_ref,
            None => {
                // Is not a Java proxy, need to create a new CppProxy
                // Todo - cache the CppProxys
                let class = ::support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/UserToken$CppProxy");
                let constructor = ::support_lib::support::get_method(jni_env, class, "<init>", "(J)V");
                let handle = Self::to_handle(r.clone());
                jni_invoke!(jni_env, NewObject, class, constructor, handle)
            },
        }
    }

    boxed_call_through!();
}

impl RustProxyable for Arc<Box<UserToken>> {
    fn to_handle(self) -> jlong {
        // Convert our box into a pointer, leaving the memory there and not running
        // the destructor on the contents. We can't use Box::into_raw since it's unstable.
        unsafe { mem::transmute(Box::new(self)) }
    }
    fn from_handle(rust_proxy_handle: jlong) -> Box<Self> {
        // Convert our pointer back into a box. We can't use Box::from_raw since it's unstable.
        unsafe { mem::transmute(rust_proxy_handle as *mut Arc<Box<Self>>) }
    }
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
    let obj: Box<Arc<Box<UserToken>>> = Arc::<Box<UserToken>>::from_handle(native_ref);
    String::from_rust(jni_env, obj.whoami())
}
