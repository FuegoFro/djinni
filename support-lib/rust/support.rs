use std::ffi::CString;
use std::marker::PhantomData;
use libc::{c_int, c_uint, c_long, c_double};
use support_lib::jni_ffi::{
    JNIEnv,
    jclass,
    jmethodID,
    jboolean,
    jbyte,
    jshort,
    jint,
    jlong,
    jfloat,
    jdouble,
    jobject,
};

// Helper for calling a method on the JNIEnv.
#[macro_export]
macro_rules! f {
    ($env:expr, $field:ident$( ,$arg:expr)*) => {
        ((unsafe { &*(*$env).functions }).$field)($env, $($arg,)*)
    }
}

fn c_str(rust_str: &str) -> CString {
    CString::new(rust_str).unwrap()
}


pub fn get_class(jni_env: *mut JNIEnv, name: &str) -> jclass {
    let c_name = c_str(name);
    f!(jni_env, FindClass, c_name.as_ptr())
}

pub fn get_method(jni_env: *mut JNIEnv, class: jclass, name: &str, signature: &str) -> jmethodID {
    let c_name = c_str(name);
    let c_sig = c_str(signature);
    f!(jni_env, GetMethodID, class, c_name.as_ptr(), c_sig.as_ptr())
}

pub trait JType {
    type RustType;
    type JniType;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self::RustType;
    fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> Self::RustType;
    fn from_rust(jni_env: *mut JNIEnv, r: Self::RustType) -> Self::JniType;
    fn from_rust_boxed(jni_env: *mut JNIEnv, r: Self::RustType) -> jobject;
}

macro_rules! primitive_marshal {
    (
        $namespace:ident,
        $rust_type:ty,
        $jni_type:ty,
        $java_class_spec:expr,
        $static_box_method:expr,
        $static_box_method_signature:expr,
        $box_jni_variadic_type:ty,
        $static_unbox_method:expr,
        $static_unbox_method_signature:expr,
        $unbox_jni_method:ident
    ) => {
        pub struct $namespace;
        impl JType for $namespace {
            type RustType = $rust_type;
            type JniType = $jni_type;

            fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self::RustType {
                j as Self::RustType
            }

            fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> Self::RustType {
                let class = get_class(jni_env, $java_class_spec);
                let method = get_method(jni_env, class, $static_unbox_method, $static_unbox_method_signature);
                $namespace::to_rust(jni_env, f!(jni_env, $unbox_jni_method, j, method))
            }

            fn from_rust(jni_env: *mut JNIEnv, r: Self::RustType) -> Self::JniType {
                r as Self::JniType
            }

            fn from_rust_boxed(jni_env: *mut JNIEnv, r: Self::RustType) -> jobject {
                let class = get_class(jni_env, $java_class_spec);
                let method = get_method(jni_env, class, $static_box_method, $static_box_method_signature);
                f!(jni_env, CallStaticObjectMethod, class, method, Self::from_rust(jni_env, r) as $box_jni_variadic_type)
            }
        }
    }
}

// Cannot cast from jboolean to bool, need to do equality comparison, so we can't use the macro to implement Bool
pub struct Bool;
impl JType for Bool {
    type RustType = bool;
    type JniType = jboolean;

    fn to_rust(_jni_env: *mut JNIEnv, j: jboolean) -> bool {
        j == (1 as jboolean)
    }

    fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> bool {
        let class = get_class(jni_env, "java/lang/Boolean");
        let method = get_method(jni_env, class, "booleanValue", "()Z");
        Self::to_rust(jni_env, f!(jni_env, CallBooleanMethod, j, method))
    }

    fn from_rust(jni_env: *mut JNIEnv, r: bool) -> jboolean {
        if r {
            1 as jboolean
        } else {
            0 as jboolean
        }
    }

    fn from_rust_boxed(jni_env: *mut JNIEnv, r: bool) -> jobject {
        let class = get_class(jni_env, "java/lang/Boolean");
        let method = get_method(jni_env, class, "valueOf", "(Z)Ljava/lang/Boolean;");
        f!(jni_env, CallStaticObjectMethod, class, method, Self::from_rust(jni_env, r) as c_uint)
    }
}

primitive_marshal!(I8, i8, jbyte, "java/lang/Byte", "valueOf", "(B)Ljava/lang/Byte;", c_uint, "byteValue", "()B", CallByteMethod);
primitive_marshal!(I16, i16, jshort, "java/lang/Short", "valueOf", "(S)Ljava/lang/Short;", c_int, "shortValue", "()S", CallShortMethod);
primitive_marshal!(I32, i32, jint, "java/lang/Integer", "valueOf", "(I)Ljava/lang/Integer;", c_int, "intValue", "()I", CallIntMethod);
primitive_marshal!(I64, i64, jlong, "java/lang/Long", "valueOf", "(J)Ljava/lang/Long;", c_long, "longValue", "()J", CallLongMethod);
primitive_marshal!(F32, f32, jfloat, "java/lang/Float", "valueOf", "(F)Ljava/lang/Float;", c_double, "floatValue", "()F", CallFloatMethod);
primitive_marshal!(F64, f64, jdouble, "java/lang/Double", "valueOf", "(D)Ljava/lang/Double;", c_double, "doubleValue", "()D", CallDoubleMethod);

#[macro_export]
macro_rules! boxed_call_through {
    () => {
        fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> Self::RustType {
            Self::to_rust(jni_env, j)
        }

        fn from_rust_boxed(jni_env: *mut JNIEnv, r: Self::RustType) -> jobject {
            Self::from_rust(jni_env, r)
        }
    }
}

pub struct Optional<T: JType> {
    optional_type: PhantomData<T>,
}
impl<T: JType> JType for Optional<T> {
    type RustType = Option<T::RustType>;
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: jobject) -> Option<T::RustType> {
        if j == 0 as jobject {
            None
        } else {
            Some(T::to_rust_boxed(jni_env, j))
        }
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Option<T::RustType>) -> jobject {
        match r {
            Some(value) => T::from_rust_boxed(jni_env, value),
            None => 0 as jobject,
        }
    }

    boxed_call_through!();
}