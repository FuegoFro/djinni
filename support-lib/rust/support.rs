use std::ffi::CString;
use std::slice;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use encoding::all::UTF_16LE;
use encoding::{Encoding, EncoderTrap};
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
    jchar,
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

    fn to_rust(jni_env: *mut JNIEnv, j: jstring) -> Self {
        let len = jni_invoke!(jni_env, GetStringLength, j) as usize;
        let jni_chars = jni_invoke!(jni_env, GetStringChars, j, 0 as *mut jboolean);
        let rust_chars = unsafe { slice::from_raw_parts(jni_chars, len) };
        String::from_utf16(rust_chars).unwrap()
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self) -> jstring {
        let bytes = UTF_16LE.encode(&r, EncoderTrap::Strict).unwrap();
        assert!(bytes.len() % 2 == 0);
        let string_length = bytes.len() / 2;
        let chars_for_java: &[u16] = unsafe { slice::from_raw_parts(bytes.as_ptr() as *const _, string_length) };
        jni_invoke!(jni_env, NewString, chars_for_java.as_ptr() as *const jchar, string_length as i32)
    }

    boxed_call_through!();
}

impl<T: JType> JType for Vec<T> {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: jobject) -> Self {
        let class = get_class(jni_env, "java/util/ArrayList");
        let method_size = get_method(jni_env, class, "size", "()I");
        let method_get = get_method(jni_env, class, "get", "(I)Ljava/lang/Object;");
        let list_size = jni_invoke!(jni_env, CallIntMethod, j, method_size) as usize;
        let mut r = Vec::with_capacity(list_size);
        for i in 0..list_size {
            let elt: jobject = jni_invoke!(jni_env, CallObjectMethod, j, method_get, i as jint);
            r.push(T::to_rust_boxed(jni_env, elt));
        }
        r
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self) -> jobject {
        let class = get_class(jni_env, "java/util/ArrayList");
        let jconstructor = get_method(jni_env, class, "<init>", "(I)V");
        let method_add = get_method(jni_env, class, "add", "(Ljava/lang/Object;)Z");
        let j: jobject = jni_invoke!(jni_env, NewLocalRef, jni_invoke!(
            jni_env, NewObject, class, jconstructor, r.len() as jint));
        for elt in r {
            let jelt = T::from_rust_boxed(jni_env, elt);
            jni_invoke!(jni_env, CallVoidMethod, j, method_add, jelt);
        }
        j
    }

    boxed_call_through!();
}

impl<E: JType + Eq + Hash> JType for HashSet<E> {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        assert!(j != 0 as Self::JniType);
        let hash_set_class = get_class(jni_env, "java/util/HashSet");
        let hash_set_method_size = get_method(jni_env, hash_set_class, "size", "()I");
        let hash_set_method_iterator = get_method(jni_env, hash_set_class, "iterator", "()Ljava/util/Iterator;");
        let iterator_class = get_class(jni_env, "java/util/Iterator");
        let iterator_method_next = get_method(jni_env, iterator_class, "next", "()Ljava/lang/Object;");
        assert!(bool::to_rust(jni_env, jni_invoke!(jni_env, IsInstanceOf, j, hash_set_class)));
        let size = jni_invoke!(jni_env, CallIntMethod, j, hash_set_method_size);
        let mut rust_hash_set = HashSet::<E>::with_capacity(size as usize);
        let it = jni_invoke!(jni_env, CallObjectMethod, j, hash_set_method_iterator);
        for _ in 0..size {
            let entry = jni_invoke!(jni_env, CallObjectMethod, it, iterator_method_next);
            // Todo - exception check
            rust_hash_set.insert(E::to_rust_boxed(jni_env, entry));
        }
        rust_hash_set
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self) -> Self::JniType {
        let hash_set_class = get_class(jni_env, "java/util/HashSet");
        let hash_set_method_constructor = get_method(jni_env, hash_set_class, "<init>", "(I)V");
        let hash_set_method_add = get_method(jni_env, hash_set_class, "add", "(Ljava/lang/Object;)Z");
        assert!(r.len() <= jint::max_value() as usize);
        let size = r.len();
        let java_hash_set = jni_invoke!(jni_env, NewObject, hash_set_class, hash_set_method_constructor, size as jint);
        // Todo - exception check
        for entry in r {
            // Todo - handle local refs properly
            jni_invoke!(jni_env, CallBooleanMethod, java_hash_set, hash_set_method_add, E::from_rust_boxed(jni_env, entry));
            // Todo - exception check
        }
        java_hash_set
    }

    boxed_call_through!();
}

impl<K: JType + Eq + Hash, V: JType> JType for HashMap<K, V> {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        assert!(j != 0 as Self::JniType);
        let hash_map_class = get_class(jni_env, "java/util/HashMap");
        let hash_map_method_size = get_method(jni_env, hash_map_class, "size", "()I");
        let hash_map_method_entry_set = get_method(jni_env, hash_map_class, "entrySet", "()Ljava/util/Set;");
        let entry_set_class = get_class(jni_env, "java/util/Set");
        let entry_set_method_iterator = get_method(jni_env, entry_set_class, "iterator", "()Ljava/util/Iterator;");
        let entry_class = get_class(jni_env, "java/util/Map$Entry");
        let entry_method_get_key = get_method(jni_env, entry_class, "getKey", "()Ljava/lang/Object;");
        let entry_method_get_value = get_method(jni_env, entry_class, "getValue", "()Ljava/lang/Object;");
        let iterator_class = get_class(jni_env, "java/util/Iterator");
        let iterator_method_next = get_method(jni_env, iterator_class, "next", "()Ljava/lang/Object;");
        assert!(bool::to_rust(jni_env, jni_invoke!(jni_env, IsInstanceOf, j, hash_map_class)));
        let size = jni_invoke!(jni_env, CallIntMethod, j, hash_map_method_size);
        let entry_set = jni_invoke!(jni_env, CallObjectMethod, j, hash_map_method_entry_set);
        let mut rust_hash_map = HashMap::<K, V>::with_capacity(size as usize);
        let it = jni_invoke!(jni_env, CallObjectMethod, entry_set, entry_set_method_iterator);
        for _ in 0..size {
            let entry = jni_invoke!(jni_env, CallObjectMethod, it, iterator_method_next);
            let key = jni_invoke!(jni_env, CallObjectMethod, entry, entry_method_get_key);
            let value = jni_invoke!(jni_env, CallObjectMethod, entry, entry_method_get_value);
            // Todo - exception check
            rust_hash_map.insert(K::to_rust_boxed(jni_env, key), V::to_rust_boxed(jni_env, value));
        }
        rust_hash_map
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self) -> Self::JniType {
        let hash_map_class = get_class(jni_env, "java/util/HashMap");
        let hash_map_method_constructor = get_method(jni_env, hash_map_class, "<init>", "(I)V");
        let hash_map_method_put = get_method(jni_env, hash_map_class, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        assert!(r.len() <= jint::max_value() as usize);
        let size = r.len();
        let java_hash_map = jni_invoke!(jni_env, NewObject, hash_map_class, hash_map_method_constructor, size as jint);
        // Todo - exception check
        for (key, value) in r {
            // Todo - handle local refs properly
            jni_invoke!(jni_env, CallObjectMethod, java_hash_map, hash_map_method_put, K::from_rust_boxed(jni_env, key), V::from_rust_boxed(jni_env, value));
            // Todo - exception check
        }
        java_hash_map
    }

    boxed_call_through!();
}

/*
 * Used to help proxy calls from Java to Rust-implemented interfaces
 */

pub trait RustProxyable {
    fn to_handle(self) -> jlong;
    fn from_handle(rust_proxy_handle: jlong) -> Box<Self>;
}

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

/*
 * Utilities for dealing with local and global JNI refs
 */

pub struct GlobalRef {
    global_ref: jobject,
}

impl GlobalRef {
    pub fn new(jni_env: *mut JNIEnv, local_ref: jobject) -> GlobalRef {
        GlobalRef {
            global_ref: jni_invoke!(jni_env, NewGlobalRef, local_ref),
        }
    }

    pub fn get(&self) -> jobject {
        self.global_ref
    }
}

impl Drop for GlobalRef {
    fn drop(&mut self) {
        let jni_env = jni_get_thread_env();
        jni_invoke!(jni_env, DeleteGlobalRef, self.global_ref);
    }
}
