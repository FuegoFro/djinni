use support_lib::jni_ffi::{JNIEnv, jobject, jclass};
use test_helpers;
use generated_rust_jni;

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_assortedPrimitivesId(jni_env: *mut JNIEnv, _class: jclass, i: jobject) -> jobject {
    let r = test_helpers::assorted_primitives_id(generated_rust_jni::assorted_primitives::NativeAssortedPrimitives::to_rust(jni_env, i));
    return generated_rust_jni::assorted_primitives::NativeAssortedPrimitives::from_rust(jni_env, r);
}
