use generated_rust::color::Color;
use support_lib::support::JType;
use support_lib::support::{get_class, get_method, get_static_method};
use support_lib::jni_ffi::{JNIEnv, jobject, jint};

impl JType for Color {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        let class = get_class(jni_env, "com/dropbox/djinni/test/Color");
        let method_ordinal = get_method(jni_env, class, "ordinal", "()I");
        let ordinal = jni_invoke!(jni_env, CallIntMethod, j, method_ordinal);
        match ordinal {
            0 => Color::Red,
            1 => Color::Orange,
            2 => Color::Yellow,
            3 => Color::Green,
            4 => Color::Blue,
            5 => Color::Indigo,
            6 => Color::Violet,
            _ => panic!("Unknown ordinal value for enum Color: {}", ordinal),
        }
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self) -> Self::JniType {
        let class = get_class(jni_env, "com/dropbox/djinni/test/Color");
        let method_values = get_static_method(jni_env, class, "values", "()[Lcom/dropbox/djinni/test/Color;");
        let values = jni_invoke!(jni_env, CallStaticObjectMethod, class, method_values);
        let ordinal = match r {
            Color::Red => 0,
            Color::Orange => 1,
            Color::Yellow => 2,
            Color::Green => 3,
            Color::Blue => 4,
            Color::Indigo => 5,
            Color::Violet => 6,
        };
        jni_invoke!(jni_env, GetObjectArrayElement, values, ordinal as jint)
    }

    boxed_call_through!();
}