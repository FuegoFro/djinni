use std::sync::Arc;
use std::boxed::Box;
use support_lib::support::JType;
use support_lib::jni_ffi::{JNIEnv, jobject, jclass, jstring, jboolean};
use test_helpers;
use generated_rust;

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_assortedPrimitivesId(jni_env: *mut JNIEnv, _class: jclass, j_i: jobject) -> jobject {
    let r = test_helpers::assorted_primitives_id(generated_rust::assorted_primitives::AssortedPrimitives::to_rust(jni_env, j_i));
    generated_rust::assorted_primitives::AssortedPrimitives::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkTokenType(jni_env: *mut JNIEnv, _class: jclass, j_t: jobject, j_type: jstring) -> jboolean {
    let ret = test_helpers::check_token_type(Arc::<Box<generated_rust::user_token::UserToken>>::to_rust(jni_env, j_t),
                                             String::to_rust(jni_env, j_type));
    bool::from_rust(jni_env, ret)
}
