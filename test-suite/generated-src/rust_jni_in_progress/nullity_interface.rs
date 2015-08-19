// AUTOGENERATED FILE - DO NOT MODIFY!
// This file generated by Djinni from interface_nullity.djinni

use support_lib::support::JType;
use support_lib::jni_ffi::{JNIEnv, jobject, jclass};
use generated_rust_jni;

impl JType for Arc<Box<NullityInterface>> {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        Arc::new(Box::new(NullityInterfaceJavaProxy { javaRef: j }))
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self) {
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

struct NullityInterfaceJavaProxy {
    javaRef: jobject
}

impl NullityInterface for NullityInterfaceJavaProxy {
    fn non_null_parameters(&self, r_p1: Arc<Box<DummyInterface>>, r_p2: Arc<Box<DummyInterface>>) {
        let jni_env = ::support_lib::support::jni_get_thread_env();
        // TODO(rustgen): local scope
        // TODO(rustgen): use helper to cache class object and method IDs
        let class = ::support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/NullityInterface");
        let jmethod = ::support_lib::support::get_method(jni_env, class, "nonNullParameters", "(Lcom/dropbox/djinni/test/DummyInterface;Lcom/dropbox/djinni/test/DummyInterface;)V");
        // TODO(rustgen): handle local refs correctly
        jni_invoke!(jni_env, CallVoidMethod, self.javaRef, jmethod,
                    Arc::<Box<DummyInterface>>::from_rust(jni_env, r_p1),
                    Arc::<Box<DummyInterface>>::from_rust(jni_env, r_p2));
    }
    fn non_null_return(&self, r_should_return_null: bool) -> Arc<Box<DummyInterface>> {
        let jni_env = ::support_lib::support::jni_get_thread_env();
        // TODO(rustgen): local scope
        // TODO(rustgen): use helper to cache class object and method IDs
        let class = ::support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/NullityInterface");
        let jmethod = ::support_lib::support::get_method(jni_env, class, "nonNullReturn", "(Z)Lcom/dropbox/djinni/test/DummyInterface;");
        // TODO(rustgen): handle local refs correctly
        let jret = jni_invoke!(jni_env, CallObjectMethod, self.javaRef, jmethod,
                    bool::from_rust(jni_env, r_should_return_null));
        Arc::<Box<DummyInterface>>::to_rust(jni_env, jret)
    }
    fn nullable_parameters(&self, r_p1: Option<Arc<Box<DummyInterface>>>, r_p2: Option<Arc<Box<DummyInterface>>>) {
        let jni_env = ::support_lib::support::jni_get_thread_env();
        // TODO(rustgen): local scope
        // TODO(rustgen): use helper to cache class object and method IDs
        let class = ::support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/NullityInterface");
        let jmethod = ::support_lib::support::get_method(jni_env, class, "nullableParameters", "(Lcom/dropbox/djinni/test/DummyInterface;Lcom/dropbox/djinni/test/DummyInterface;)V");
        // TODO(rustgen): handle local refs correctly
        jni_invoke!(jni_env, CallVoidMethod, self.javaRef, jmethod,
                    Option::<Arc<Box<DummyInterface>>>::from_rust(jni_env, r_p1),
                    Option::<Arc<Box<DummyInterface>>>::from_rust(jni_env, r_p2));
    }
    fn nullable_return(&self, r_should_return_null: bool) -> Option<Arc<Box<DummyInterface>>> {
        let jni_env = ::support_lib::support::jni_get_thread_env();
        // TODO(rustgen): local scope
        // TODO(rustgen): use helper to cache class object and method IDs
        let class = ::support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/NullityInterface");
        let jmethod = ::support_lib::support::get_method(jni_env, class, "nullableReturn", "(Z)Lcom/dropbox/djinni/test/DummyInterface;");
        // TODO(rustgen): handle local refs correctly
        let jret = jni_invoke!(jni_env, CallObjectMethod, self.javaRef, jmethod,
                    bool::from_rust(jni_env, r_should_return_null));
        Option::<Arc<Box<DummyInterface>>>::to_rust(jni_env, jret)
    }
}
struct NullityInterfaceCppProxy {
    rustRef: Arc<Box<NullityInterface>>
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_NullityInterface_00024CppProxy_nonNullParameters(jni_env: *mut JNIEnv, _: jobject, nativeRef: jlong, j_p1: jobject, j_p2: jobject) {
    let jni_env = ::support_lib::support::jni_get_thread_env();
    let rustRef = support_lib::support::CppProxyHandle::<NullityInterface>::get(nativeRef);
    rustRef.non_null_parameters(Arc::<Box<DummyInterface>>::to_rust(jni_env, j_p1),
                                Arc::<Box<DummyInterface>>::to_rust(jni_env, j_p2));
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_NullityInterface_00024CppProxy_nonNullReturn(jni_env: *mut JNIEnv, _: jobject, nativeRef: jlong, j_shouldReturnNull: jboolean) {
    let jni_env = ::support_lib::support::jni_get_thread_env();
    let rustRef = support_lib::support::CppProxyHandle::<NullityInterface>::get(nativeRef);
    let r = rustRef.non_null_return(bool::to_rust(jni_env, j_shouldReturnNull));
    Arc::<Box<DummyInterface>>::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_NullityInterface_00024CppProxy_nullableParameters(jni_env: *mut JNIEnv, _: jobject, nativeRef: jlong, j_p1: jobject, j_p2: jobject) {
    let jni_env = ::support_lib::support::jni_get_thread_env();
    let rustRef = support_lib::support::CppProxyHandle::<NullityInterface>::get(nativeRef);
    rustRef.nullable_parameters(Option::<Arc<Box<DummyInterface>>>::to_rust(jni_env, j_p1),
                                Option::<Arc<Box<DummyInterface>>>::to_rust(jni_env, j_p2));
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_NullityInterface_00024CppProxy_nullableReturn(jni_env: *mut JNIEnv, _: jobject, nativeRef: jlong, j_shouldReturnNull: jboolean) {
    let jni_env = ::support_lib::support::jni_get_thread_env();
    let rustRef = support_lib::support::CppProxyHandle::<NullityInterface>::get(nativeRef);
    let r = rustRef.nullable_return(bool::to_rust(jni_env, j_shouldReturnNull));
    Option::<Arc<Box<DummyInterface>>>::from_rust(jni_env, r)
}
