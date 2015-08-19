use std::ffi::CString;
use libc::{c_int, c_uint, c_long, c_double};
use support_lib::jni_ffi::{
    JavaVM,
    JNIEnv,
    jclass,
    jmethodID,
    jfieldID,
    jboolean,
    jbyte,
    jshort,
    jint,
    jlong,
    jfloat,
    jdouble,
    jobject,
    jstring,
    JNI_VERSION_1_6,
};

// Helper for calling a method on the JNIEnv or JavaVM.
#[macro_export]
macro_rules! jni_invoke {
    ($env:expr, $field:ident$( ,$arg:expr)*) => {
        ((unsafe { &*(*$env).functions }).$field)($env, $($arg,)*)
    }
}

static mut global_cached_jvm: Option<JavaVmWrapper> = None;

struct JavaVmWrapper(*mut JavaVM);

unsafe impl Sync for JavaVmWrapper {}

pub fn jni_init(jvm: *mut JavaVM) {
    // Todo - make sure this is safe
    unsafe { global_cached_jvm = Some(JavaVmWrapper(jvm)); }
    // Todo - initialize cached class stuff here.
}

pub fn jni_shutdown() {
    // Todo - make sure this is safe
    unsafe { global_cached_jvm = None; }
}

pub fn jni_get_thread_env() -> *mut JNIEnv {
    // Todo - make sure this is safe
    match unsafe { &global_cached_jvm } {
        &Some(ref jvm_wrapper) => {
            let &JavaVmWrapper(ref jvm) = jvm_wrapper;
            let mut env: *mut JNIEnv = 0 as *mut JNIEnv;
            let env_handle: *mut *mut JNIEnv = &mut env;
            let get_res = jni_invoke!(*jvm, GetEnv, env_handle, JNI_VERSION_1_6);
            assert!(get_res == 0 && env != 0 as *mut JNIEnv);
            env
        },
        &None => panic!(),
    }
}

fn c_str(rust_str: &str) -> CString {
    CString::new(rust_str).unwrap()
}


pub fn get_class(jni_env: *mut JNIEnv, name: &str) -> jclass {
    let c_name = c_str(name);
    let unreferenced_class = jni_invoke!(jni_env, FindClass, c_name.as_ptr());
    jni_invoke!(jni_env, NewLocalRef, unreferenced_class)
}

pub fn get_method(jni_env: *mut JNIEnv, class: jclass, name: &str, signature: &str) -> jmethodID {
    let c_name = c_str(name);
    let c_sig = c_str(signature);
    jni_invoke!(jni_env, GetMethodID, class, c_name.as_ptr(), c_sig.as_ptr())
}

pub fn get_static_method(jni_env: *mut JNIEnv, class: jclass, name: &str, signature: &str) -> jmethodID {
    let c_name = c_str(name);
    let c_sig = c_str(signature);
    jni_invoke!(jni_env, GetStaticMethodID, class, c_name.as_ptr(), c_sig.as_ptr())
}

pub fn get_field(jni_env: *mut JNIEnv, class: jclass, name: &str, signature: &str) -> jfieldID {
    let c_name = c_str(name);
    let c_sig = c_str(signature);
    jni_invoke!(jni_env, GetFieldID, class, c_name.as_ptr(), c_sig.as_ptr())
}

/*
 * Implementations of the conversions to/from rust.
 */

pub trait JType {
    type JniType: ForVariadic;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self;
    fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> Self;
    fn from_rust(jni_env: *mut JNIEnv, r: Self) -> Self::JniType;
    fn from_rust_boxed(jni_env: *mut JNIEnv, r: Self) -> jobject;
}

macro_rules! primitive_marshal {
    (
        $rust_type:ty,
        $jni_type:ty,
        $java_class_spec:expr,
        $static_box_method:expr,
        $static_box_method_signature:expr,
        $static_unbox_method:expr,
        $static_unbox_method_signature:expr,
        $unbox_jni_method:ident
    ) => {
        impl JType for $rust_type {
            type JniType = $jni_type;

            fn to_rust(_jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
                j as Self
            }

            fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> Self {
                let class = get_class(jni_env, $java_class_spec);
                let method = get_method(jni_env, class, $static_unbox_method, $static_unbox_method_signature);
                Self::to_rust(jni_env, jni_invoke!(jni_env, $unbox_jni_method, j, method))
            }

            fn from_rust(_jni_env: *mut JNIEnv, r: Self) -> Self::JniType {
                r as Self::JniType
            }

            fn from_rust_boxed(jni_env: *mut JNIEnv, r: Self) -> jobject {
                let class = get_class(jni_env, $java_class_spec);
                let method = get_static_method(jni_env, class, $static_box_method, $static_box_method_signature);
                jni_invoke!(jni_env, CallStaticObjectMethod, class, method, Self::from_rust(jni_env, r).for_variadic())
            }
        }
    }
}

// Cannot cast from jboolean to bool, need to do equality comparison, so we can't use the macro to implement Bool
impl JType for bool {
    type JniType = jboolean;

    fn to_rust(_jni_env: *mut JNIEnv, j: jboolean) -> Self {
        j == (1 as jboolean)
    }

    fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> Self {
        let class = get_class(jni_env, "java/lang/Boolean");
        let method = get_method(jni_env, class, "booleanValue", "()Z");
        Self::to_rust(jni_env, jni_invoke!(jni_env, CallBooleanMethod, j, method))
    }

    fn from_rust(_jni_env: *mut JNIEnv, r: bool) -> Self::JniType {
        if r {
            1 as jboolean
        } else {
            0 as jboolean
        }
    }

    fn from_rust_boxed(jni_env: *mut JNIEnv, r: bool) -> jobject {
        let class = get_class(jni_env, "java/lang/Boolean");
        let method = get_static_method(jni_env, class, "valueOf", "(Z)Ljava/lang/Boolean;");
        jni_invoke!(jni_env, CallStaticObjectMethod, class, method, Self::from_rust(jni_env, r).for_variadic())
    }
}

primitive_marshal!(i8, jbyte, "java/lang/Byte", "valueOf", "(B)Ljava/lang/Byte;", "byteValue", "()B", CallByteMethod);
primitive_marshal!(i16, jshort, "java/lang/Short", "valueOf", "(S)Ljava/lang/Short;", "shortValue", "()S", CallShortMethod);
primitive_marshal!(i32, jint, "java/lang/Integer", "valueOf", "(I)Ljava/lang/Integer;", "intValue", "()I", CallIntMethod);
primitive_marshal!(i64, jlong, "java/lang/Long", "valueOf", "(J)Ljava/lang/Long;", "longValue", "()J", CallLongMethod);
primitive_marshal!(f32, jfloat, "java/lang/Float", "valueOf", "(F)Ljava/lang/Float;", "floatValue", "()F", CallFloatMethod);
primitive_marshal!(f64, jdouble, "java/lang/Double", "valueOf", "(D)Ljava/lang/Double;", "doubleValue", "()D", CallDoubleMethod);

#[macro_export]
macro_rules! boxed_call_through {
    () => {
        fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> Self {
            Self::to_rust(jni_env, j)
        }

        fn from_rust_boxed(jni_env: *mut JNIEnv, r: Self) -> jobject {
            Self::from_rust(jni_env, r)
        }
    }
}

impl<T: JType> JType for Option<T> {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: jobject) -> Self {
        if j == 0 as jobject {
            None
        } else {
            Some(T::to_rust_boxed(jni_env, j))
        }
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self) -> jobject {
        match r {
            Some(value) => T::from_rust_boxed(jni_env, value),
            None => 0 as jobject,
        }
    }

    boxed_call_through!();
}

impl JType for String {
    type JniType = jstring;

    fn to_rust(_jni_env: *mut JNIEnv, _j: jobject) -> Self {
        "".to_string()
    }

    fn from_rust(_jni_env: *mut JNIEnv, _r: Self) -> jobject {
        0 as jobject
    }

    boxed_call_through!();
}

/*
 * Helpers for converting strings between UTF8 and UTF16
 */

/*
 * Help translate the jni types to sizes that are
 * able to be passed to variadic functions.
 */

pub trait ForVariadic {
    type VariadicType;

    fn for_variadic(self) -> Self::VariadicType;
}

macro_rules! impl_for_variadic {
    ($jni_type:ty, $variadic_type:ty) => {
        impl ForVariadic for $jni_type {
            type VariadicType = $variadic_type;
            fn for_variadic(self) -> Self::VariadicType {
                self as Self::VariadicType
            }
        }
    }
}

impl_for_variadic!(jboolean, c_uint);
impl_for_variadic!(jbyte, c_uint);
impl_for_variadic!(jshort, c_int);
impl_for_variadic!(jint, c_int);
impl_for_variadic!(jlong, c_long);
impl_for_variadic!(jfloat, c_double);
impl_for_variadic!(jdouble, c_double);
impl_for_variadic!(jobject, jobject);
