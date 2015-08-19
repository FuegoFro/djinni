// AUTOGENERATED FILE - DO NOT MODIFY!
// This file generated by Djinni from test.djinni

use support_lib::support::JType;
use support_lib::jni_ffi::{JNIEnv, jobject, jclass};
use test_helpers;
use generated_rust_jni;

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_getSetRecord(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = test_helpers::get_set_record()
    generated_rust_jni::set_record::NativeSetRecord::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkSetRecord(jni_env: *mut JNIEnv, _class: jclass, j_rec: jobject) -> jboolean {
    let r = test_helpers::check_set_record(generated_rust_jni::set_record::NativeSetRecord::to_rust(jni_env, j_rec))
    support_lib::support::Bool::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_getPrimitiveList(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = test_helpers::get_primitive_list()
    generated_rust_jni::primitive_list::NativePrimitiveList::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkPrimitiveList(jni_env: *mut JNIEnv, _class: jclass, j_pl: jobject) -> jboolean {
    let r = test_helpers::check_primitive_list(generated_rust_jni::primitive_list::NativePrimitiveList::to_rust(jni_env, j_pl))
    support_lib::support::Bool::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_getNestedCollection(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = test_helpers::get_nested_collection()
    generated_rust_jni::nested_collection::NativeNestedCollection::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkNestedCollection(jni_env: *mut JNIEnv, _class: jclass, j_nc: jobject) -> jboolean {
    let r = test_helpers::check_nested_collection(generated_rust_jni::nested_collection::NativeNestedCollection::to_rust(jni_env, j_nc))
    support_lib::support::Bool::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_getMap(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = test_helpers::get_map()
    support_lib::support::Map::<support_lib::support::String, support_lib::support::I64>::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkMap(jni_env: *mut JNIEnv, _class: jclass, j_m: jobject) -> jboolean {
    let r = test_helpers::check_map(support_lib::support::Map::<support_lib::support::String, support_lib::support::I64>::to_rust(jni_env, j_m))
    support_lib::support::Bool::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_getEmptyMap(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = test_helpers::get_empty_map()
    support_lib::support::Map::<support_lib::support::String, support_lib::support::I64>::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkEmptyMap(jni_env: *mut JNIEnv, _class: jclass, j_m: jobject) -> jboolean {
    let r = test_helpers::check_empty_map(support_lib::support::Map::<support_lib::support::String, support_lib::support::I64>::to_rust(jni_env, j_m))
    support_lib::support::Bool::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_getMapListRecord(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = test_helpers::get_map_list_record()
    generated_rust_jni::map_list_record::NativeMapListRecord::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkMapListRecord(jni_env: *mut JNIEnv, _class: jclass, j_m: jobject) -> jboolean {
    let r = test_helpers::check_map_list_record(generated_rust_jni::map_list_record::NativeMapListRecord::to_rust(jni_env, j_m))
    support_lib::support::Bool::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkClientInterfaceAscii(jni_env: *mut JNIEnv, _class: jclass, j_i: jobject) {
    test_helpers::check_client_interface_ascii(generated_rust_jni::client_interface::NativeClientInterface::to_rust(jni_env, j_i))
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkClientInterfaceNonascii(jni_env: *mut JNIEnv, _class: jclass, j_i: jobject) {
    test_helpers::check_client_interface_nonascii(generated_rust_jni::client_interface::NativeClientInterface::to_rust(jni_env, j_i))
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkEnumMap(jni_env: *mut JNIEnv, _class: jclass, j_m: jobject) {
    test_helpers::check_enum_map(support_lib::support::Map::<generated_rust_jni::color::NativeColor, support_lib::support::String>::to_rust(jni_env, j_m))
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkEnum(jni_env: *mut JNIEnv, _class: jclass, j_c: jobject) {
    test_helpers::check_enum(generated_rust_jni::color::NativeColor::to_rust(jni_env, j_c))
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_tokenId(jni_env: *mut JNIEnv, _class: jclass, j_t: jobject) -> jobject {
    let r = test_helpers::token_id(support_lib::support::Optional::<generated_rust_jni::user_token::NativeUserToken>::to_rust(jni_env, j_t))
    support_lib::support::Optional::<generated_rust_jni::user_token::NativeUserToken>::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_createCppToken(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = test_helpers::create_cpp_token()
    generated_rust_jni::user_token::NativeUserToken::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkCppToken(jni_env: *mut JNIEnv, _class: jclass, j_t: jobject) {
    test_helpers::check_cpp_token(generated_rust_jni::user_token::NativeUserToken::to_rust(jni_env, j_t))
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_cppTokenId(jni_env: *mut JNIEnv, _class: jclass, j_t: jobject) -> jlong {
    let r = test_helpers::cpp_token_id(generated_rust_jni::user_token::NativeUserToken::to_rust(jni_env, j_t))
    support_lib::support::I64::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkTokenType(jni_env: *mut JNIEnv, _class: jclass, j_t: jobject, j_type: jstring) {
    test_helpers::check_token_type(generated_rust_jni::user_token::NativeUserToken::to_rust(jni_env, j_t),
                                   support_lib::support::String::to_rust(jni_env, j_type))
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_returnNone(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = test_helpers::return_none()
    support_lib::support::Optional::<support_lib::support::I32>::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_assortedPrimitivesId(jni_env: *mut JNIEnv, _class: jclass, j_i: jobject) -> jobject {
    let r = test_helpers::assorted_primitives_id(generated_rust_jni::assorted_primitives::NativeAssortedPrimitives::to_rust(jni_env, j_i))
    generated_rust_jni::assorted_primitives::NativeAssortedPrimitives::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_idBinary(jni_env: *mut JNIEnv, _class: jclass, j_b: jbyteArray) -> jbyteArray {
    let r = test_helpers::id_binary(support_lib::support::Binary::to_rust(jni_env, j_b))
    support_lib::support::Binary::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_getNullityInterface(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = test_helpers::get_nullity_interface()
    generated_rust_jni::nullity_interface::NativeNullityInterface::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkInterfaceNullityParameters(jni_env: *mut JNIEnv, _class: jclass, j_i: jobject) {
    test_helpers::check_interface_nullity_parameters(generated_rust_jni::nullity_interface::NativeNullityInterface::to_rust(jni_env, j_i))
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkInterfaceNullityReturn(jni_env: *mut JNIEnv, _class: jclass, j_i: jobject) {
    test_helpers::check_interface_nullity_return(generated_rust_jni::nullity_interface::NativeNullityInterface::to_rust(jni_env, j_i))
}
