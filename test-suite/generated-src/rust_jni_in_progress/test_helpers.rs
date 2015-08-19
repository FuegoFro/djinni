// AUTOGENERATED FILE - DO NOT MODIFY!
// This file generated by Djinni from test.djinni

use support_lib::support::JType;
use support_lib::jni_ffi::{JNIEnv, jobject, jclass};
use generated_rust_jni;

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_getSetRecord(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = ::test_helpers::get_set_record()
    SetRecord::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkSetRecord(jni_env: *mut JNIEnv, _class: jclass, j_rec: jobject) -> jboolean {
    let r = ::test_helpers::check_set_record(SetRecord::to_rust(jni_env, j_rec))
    bool::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_getPrimitiveList(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = ::test_helpers::get_primitive_list()
    PrimitiveList::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkPrimitiveList(jni_env: *mut JNIEnv, _class: jclass, j_pl: jobject) -> jboolean {
    let r = ::test_helpers::check_primitive_list(PrimitiveList::to_rust(jni_env, j_pl))
    bool::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_getNestedCollection(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = ::test_helpers::get_nested_collection()
    NestedCollection::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkNestedCollection(jni_env: *mut JNIEnv, _class: jclass, j_nc: jobject) -> jboolean {
    let r = ::test_helpers::check_nested_collection(NestedCollection::to_rust(jni_env, j_nc))
    bool::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_getMap(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = ::test_helpers::get_map()
    HashMap::<String, i64>::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkMap(jni_env: *mut JNIEnv, _class: jclass, j_m: jobject) -> jboolean {
    let r = ::test_helpers::check_map(HashMap::<String, i64>::to_rust(jni_env, j_m))
    bool::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_getEmptyMap(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = ::test_helpers::get_empty_map()
    HashMap::<String, i64>::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkEmptyMap(jni_env: *mut JNIEnv, _class: jclass, j_m: jobject) -> jboolean {
    let r = ::test_helpers::check_empty_map(HashMap::<String, i64>::to_rust(jni_env, j_m))
    bool::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_getMapListRecord(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = ::test_helpers::get_map_list_record()
    MapListRecord::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkMapListRecord(jni_env: *mut JNIEnv, _class: jclass, j_m: jobject) -> jboolean {
    let r = ::test_helpers::check_map_list_record(MapListRecord::to_rust(jni_env, j_m))
    bool::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkClientInterfaceAscii(jni_env: *mut JNIEnv, _class: jclass, j_i: jobject) {
    ::test_helpers::check_client_interface_ascii(Arc::<Box<ClientInterface>>::to_rust(jni_env, j_i))
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkClientInterfaceNonascii(jni_env: *mut JNIEnv, _class: jclass, j_i: jobject) {
    ::test_helpers::check_client_interface_nonascii(Arc::<Box<ClientInterface>>::to_rust(jni_env, j_i))
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkEnumMap(jni_env: *mut JNIEnv, _class: jclass, j_m: jobject) {
    ::test_helpers::check_enum_map(HashMap::<Color, String>::to_rust(jni_env, j_m))
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkEnum(jni_env: *mut JNIEnv, _class: jclass, j_c: jobject) {
    ::test_helpers::check_enum(Color::to_rust(jni_env, j_c))
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_tokenId(jni_env: *mut JNIEnv, _class: jclass, j_t: jobject) -> jobject {
    let r = ::test_helpers::token_id(Option::<Arc<Box<UserToken>>>::to_rust(jni_env, j_t))
    Option::<Arc<Box<UserToken>>>::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_createCppToken(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = ::test_helpers::create_cpp_token()
    Arc::<Box<UserToken>>::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkCppToken(jni_env: *mut JNIEnv, _class: jclass, j_t: jobject) {
    ::test_helpers::check_cpp_token(Arc::<Box<UserToken>>::to_rust(jni_env, j_t))
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_cppTokenId(jni_env: *mut JNIEnv, _class: jclass, j_t: jobject) -> jlong {
    let r = ::test_helpers::cpp_token_id(Arc::<Box<UserToken>>::to_rust(jni_env, j_t))
    i64::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkTokenType(jni_env: *mut JNIEnv, _class: jclass, j_t: jobject, j_expectedType: jstring) -> jboolean {
    let r = ::test_helpers::check_token_type(Arc::<Box<UserToken>>::to_rust(jni_env, j_t),
                                             String::to_rust(jni_env, j_expectedType))
    bool::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_returnNone(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = ::test_helpers::return_none()
    Option::<i32>::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_assortedPrimitivesId(jni_env: *mut JNIEnv, _class: jclass, j_i: jobject) -> jobject {
    let r = ::test_helpers::assorted_primitives_id(AssortedPrimitives::to_rust(jni_env, j_i))
    AssortedPrimitives::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_idBinary(jni_env: *mut JNIEnv, _class: jclass, j_b: jbyteArray) -> jbyteArray {
    let r = ::test_helpers::id_binary(Box<[u8]>::to_rust(jni_env, j_b))
    Box<[u8]>::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_getNullityInterface(jni_env: *mut JNIEnv, _class: jclass) -> jobject {
    let r = ::test_helpers::get_nullity_interface()
    Arc::<Box<NullityInterface>>::from_rust(jni_env, r)
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkInterfaceNullityParameters(jni_env: *mut JNIEnv, _class: jclass, j_i: jobject) {
    ::test_helpers::check_interface_nullity_parameters(Arc::<Box<NullityInterface>>::to_rust(jni_env, j_i))
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_dropbox_djinni_test_TestHelpers_checkInterfaceNullityReturn(jni_env: *mut JNIEnv, _class: jclass, j_i: jobject) {
    ::test_helpers::check_interface_nullity_return(Arc::<Box<NullityInterface>>::to_rust(jni_env, j_i))
}

impl JType for Arc<Box<TestHelpers>> {
    type JniType = jobject;

    fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self {
        Arc::new(Box::new(TestHelpersJavaProxy { javaRef: j }))
    }

    fn from_rust(jni_env: *mut JNIEnv, r: Self {
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

struct TestHelpersCppProxy {
    rustRef: Arc<Box<TestHelpers>>
}
