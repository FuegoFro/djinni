use support_lib;
use support_lib::support::{JType, ForVariadic};
use support_lib::jni_ffi::{JNIEnv, jobject};
use generated_rust::assorted_primitives::AssortedPrimitives;

pub struct NativeAssortedPrimitives;
impl JType for NativeAssortedPrimitives {
    type RustType = AssortedPrimitives;
    type JniType = jobject;

    #[allow(non_snake_case)]
    fn to_rust(jni_env: *mut JNIEnv, j: jobject) -> AssortedPrimitives {
        // Todo - have a local scope here
        // Todo - cache the class/methods
        // Todo - class object should have a ref around it
        let class = support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/AssortedPrimitives");
        let field_mB = support_lib::support::get_field(jni_env, class, "mB", "Z");
        let field_mEight = support_lib::support::get_field(jni_env, class, "mEight", "B");
        let field_mSixteen = support_lib::support::get_field(jni_env, class, "mSixteen", "S");
        let field_mThirtytwo = support_lib::support::get_field(jni_env, class, "mThirtytwo", "I");
        let field_mSixtyfour = support_lib::support::get_field(jni_env, class, "mSixtyfour", "J");
        let field_mFthirtytwo = support_lib::support::get_field(jni_env, class, "mFthirtytwo", "F");
        let field_mFsixtyfour = support_lib::support::get_field(jni_env, class, "mFsixtyfour", "D");
        let field_mOB = support_lib::support::get_field(jni_env, class, "mOB", "Ljava/lang/Boolean;");
        let field_mOEight = support_lib::support::get_field(jni_env, class, "mOEight", "Ljava/lang/Byte;");
        let field_mOSixteen = support_lib::support::get_field(jni_env, class, "mOSixteen", "Ljava/lang/Short;");
        let field_mOThirtytwo = support_lib::support::get_field(jni_env, class, "mOThirtytwo", "Ljava/lang/Integer;");
        let field_mOSixtyfour = support_lib::support::get_field(jni_env, class, "mOSixtyfour", "Ljava/lang/Long;");
        let field_mOFthirtytwo = support_lib::support::get_field(jni_env, class, "mOFthirtytwo", "Ljava/lang/Float;");
        let field_mOFsixtyfour = support_lib::support::get_field(jni_env, class, "mOFsixtyfour", "Ljava/lang/Double;");

        assert!(j != 0 as jobject);
        AssortedPrimitives {
                b: support_lib::support::Bool::to_rust(jni_env, jni_invoke!(jni_env, GetBooleanField, j, field_mB)),
                eight: support_lib::support::I8::to_rust(jni_env, jni_invoke!(jni_env, GetByteField, j, field_mEight)),
                sixteen: support_lib::support::I16::to_rust(jni_env, jni_invoke!(jni_env, GetShortField, j, field_mSixteen)),
                thirtytwo: support_lib::support::I32::to_rust(jni_env, jni_invoke!(jni_env, GetIntField, j, field_mThirtytwo)),
                sixtyfour: support_lib::support::I64::to_rust(jni_env, jni_invoke!(jni_env, GetLongField, j, field_mSixtyfour)),
                fthirtytwo: support_lib::support::F32::to_rust(jni_env, jni_invoke!(jni_env, GetFloatField, j, field_mFthirtytwo)),
                fsixtyfour: support_lib::support::F64::to_rust(jni_env, jni_invoke!(jni_env, GetDoubleField, j, field_mFsixtyfour)),
                o_b: support_lib::support::Optional::<support_lib::support::Bool>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_mOB)),
                o_eight: support_lib::support::Optional::<support_lib::support::I8>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_mOEight)),
                o_sixteen: support_lib::support::Optional::<support_lib::support::I16>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_mOSixteen)),
                o_thirtytwo: support_lib::support::Optional::<support_lib::support::I32>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_mOThirtytwo)),
                o_sixtyfour: support_lib::support::Optional::<support_lib::support::I64>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_mOSixtyfour)),
                o_fthirtytwo: support_lib::support::Optional::<support_lib::support::F32>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_mOFthirtytwo)),
                o_fsixtyfour: support_lib::support::Optional::<support_lib::support::F64>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_mOFsixtyfour)),
        }
    }

    fn from_rust(jni_env: *mut JNIEnv, r: AssortedPrimitives) -> jobject {
        // Todo - cache the class/methods
        // Todo - class object should have a ref around it
        let class = support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/AssortedPrimitives");
        let jconstructor = support_lib::support::get_method(jni_env, class, "<init>", "(ZBSIJFDLjava/lang/Boolean;Ljava/lang/Byte;Ljava/lang/Short;Ljava/lang/Integer;Ljava/lang/Long;Ljava/lang/Float;Ljava/lang/Double;)V");

        // Todo - handle local refs correctly.
        jni_invoke!(jni_env, NewLocalRef, jni_invoke!(jni_env, NewObject, class, jconstructor,
            support_lib::support::Bool::from_rust(jni_env, r.b).for_variadic(),
            support_lib::support::I8::from_rust(jni_env, r.eight).for_variadic(),
            support_lib::support::I16::from_rust(jni_env, r.sixteen).for_variadic(),
            support_lib::support::I32::from_rust(jni_env, r.thirtytwo).for_variadic(),
            support_lib::support::I64::from_rust(jni_env, r.sixtyfour).for_variadic(),
            support_lib::support::F32::from_rust(jni_env, r.fthirtytwo).for_variadic(),
            support_lib::support::F64::from_rust(jni_env, r.fsixtyfour).for_variadic(),

            support_lib::support::Optional::<support_lib::support::Bool>::from_rust(jni_env, r.o_b).for_variadic(),
            support_lib::support::Optional::<support_lib::support::I8>::from_rust(jni_env, r.o_eight).for_variadic(),
            support_lib::support::Optional::<support_lib::support::I16>::from_rust(jni_env, r.o_sixteen).for_variadic(),
            support_lib::support::Optional::<support_lib::support::I32>::from_rust(jni_env, r.o_thirtytwo).for_variadic(),
            support_lib::support::Optional::<support_lib::support::I64>::from_rust(jni_env, r.o_sixtyfour).for_variadic(),
            support_lib::support::Optional::<support_lib::support::F32>::from_rust(jni_env, r.o_fthirtytwo).for_variadic(),
            support_lib::support::Optional::<support_lib::support::F64>::from_rust(jni_env, r.o_fsixtyfour).for_variadic()
        ))
    }

    boxed_call_through!();
}