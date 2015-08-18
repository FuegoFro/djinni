#[macro_use]
use support_lib;
use support_lib::support::JType;
use support_lib::jni_ffi::{JNIEnv, jobject};
use generated_rust::assorted_primitives::AssortedPrimitives;

pub struct NativeAssortedPrimitives;
impl JType for NativeAssortedPrimitives {
    type RustType = AssortedPrimitives;
    type JniType = jobject;

    #[allow(non_snake_case)]
    pub fn to_rust(jni_env: *mut JNIEnv, j: jobject) -> AssortedPrimitives {
        // Todo - have a local scope here
        // Todo - use a helper to get the class/methods
        let class = support_lib::support::get_class(jni_env, "com.dropbox.djinni.test.AssortedPrimitives");
        let field_mB = support_lib::support::get_method(jni_env, class, "mB", "Z");
        let field_mEight = support_lib::support::get_method(jni_env, class, "mEight", "B");
        let field_mSixteen = support_lib::support::get_method(jni_env, class, "mSixteen", "S");
        let field_mThirtytwo = support_lib::support::get_method(jni_env, class, "mThirtytwo", "I");
        let field_mSixtyfour = support_lib::support::get_method(jni_env, class, "mSixtyfour", "J");
        let field_mFthirtytwo = support_lib::support::get_method(jni_env, class, "mFthirtytwo", "F");
        let field_mFsixtyfour = support_lib::support::get_method(jni_env, class, "mFsixtyfour", "D");
        let field_mOB = support_lib::support::get_method(jni_env, class, "mOB", "Ljava/lang/Boolean;");
        let field_mOEight = support_lib::support::get_method(jni_env, class, "mOEight", "Ljava/lang/Byte;");
        let field_mOSixteen = support_lib::support::get_method(jni_env, class, "mOSixteen", "Ljava/lang/Short;");
        let field_mOThirtytwo = support_lib::support::get_method(jni_env, class, "mOThirtytwo", "Ljava/lang/Integer;");
        let field_mOSixtyfour = support_lib::support::get_method(jni_env, class, "mOSixtyfour", "Ljava/lang/Long;");
        let field_mOFthirtytwo = support_lib::support::get_method(jni_env, class, "mOFthirtytwo", "Ljava/lang/Float;");
        let field_mOFsixtyfour = support_lib::support::get_method(jni_env, class, "mOFsixtyfour", "Ljava/lang/Double;");

        assert!(j != 0 as jobject);
        AssortedPrimitives::new(
                support_lib::support::Bool::to_rust(jni_env, f!(jni_env, GetBooleanField, j, field_mB)),
                support_lib::support::I8::to_rust(jni_env, f!(jni_env, GetByteField, j, field_mEight)),
                support_lib::support::I16::to_rust(jni_env, f!(jni_env, GetShortField, j, field_mSixteen)),
                support_lib::support::I32::to_rust(jni_env, f!(jni_env, GetIntField, j, field_mThirtytwo)),
                support_lib::support::I64::to_rust(jni_env, f!(jni_env, GetLongField, j, field_mSixtyfour)),
                support_lib::support::F32::to_rust(jni_env, f!(jni_env, GetFloatField, j, field_mFthirtytwo)),
                support_lib::support::F64::to_rust(jni_env, f!(jni_env, GetDoubleField, j, field_mFsixtyfour)),
                support_lib::support::Optional::<support_lib::support::Bool>::to_rust(jni_env, f!(jni_env, GetObjectField, j, field_mOB)),
                support_lib::support::Optional::<support_lib::support::I8>::to_rust(jni_env, f!(jni_env, GetObjectField, j, field_mOEight)),
                support_lib::support::Optional::<support_lib::support::I16>::to_rust(jni_env, f!(jni_env, GetObjectField, j, field_mOSixteen)),
                support_lib::support::Optional::<support_lib::support::I32>::to_rust(jni_env, f!(jni_env, GetObjectField, j, field_mOThirtytwo)),
                support_lib::support::Optional::<support_lib::support::I64>::to_rust(jni_env, f!(jni_env, GetObjectField, j, field_mOSixtyfour)),
                support_lib::support::Optional::<support_lib::support::F32>::to_rust(jni_env, f!(jni_env, GetObjectField, j, field_mOFthirtytwo)),
                support_lib::support::Optional::<support_lib::support::F64>::to_rust(jni_env, f!(jni_env, GetObjectField, j, field_mOFsixtyfour)),
        )
    }

    pub fn from_rust(jni_env: *mut JNIEnv, r: AssortedPrimitives) -> jobject {
        0 as jobject
    }

    boxed_call_through!();
}