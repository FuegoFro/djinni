use support_lib;
use support_lib::support::{JType, ForVariadic};
use support_lib::jni_ffi::{JNIEnv, jobject};
use generated_rust::assorted_primitives::AssortedPrimitives;

impl JType for AssortedPrimitives {
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
                b: bool::to_rust(jni_env, jni_invoke!(jni_env, GetBooleanField, j, field_mB)),
                eight: i8::to_rust(jni_env, jni_invoke!(jni_env, GetByteField, j, field_mEight)),
                sixteen: i16::to_rust(jni_env, jni_invoke!(jni_env, GetShortField, j, field_mSixteen)),
                thirtytwo: i32::to_rust(jni_env, jni_invoke!(jni_env, GetIntField, j, field_mThirtytwo)),
                sixtyfour: i64::to_rust(jni_env, jni_invoke!(jni_env, GetLongField, j, field_mSixtyfour)),
                fthirtytwo: f32::to_rust(jni_env, jni_invoke!(jni_env, GetFloatField, j, field_mFthirtytwo)),
                fsixtyfour: f64::to_rust(jni_env, jni_invoke!(jni_env, GetDoubleField, j, field_mFsixtyfour)),
                o_b: Option::<bool>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_mOB)),
                o_eight: Option::<i8>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_mOEight)),
                o_sixteen: Option::<i16>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_mOSixteen)),
                o_thirtytwo: Option::<i32>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_mOThirtytwo)),
                o_sixtyfour: Option::<i64>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_mOSixtyfour)),
                o_fthirtytwo: Option::<f32>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_mOFthirtytwo)),
                o_fsixtyfour: Option::<f64>::to_rust(jni_env, jni_invoke!(jni_env, GetObjectField, j, field_mOFsixtyfour)),
        }
    }

    fn from_rust(jni_env: *mut JNIEnv, r: AssortedPrimitives) -> jobject {
        // Todo - cache the class/methods
        // Todo - class object should have a ref around it
        let class = support_lib::support::get_class(jni_env, "com/dropbox/djinni/test/AssortedPrimitives");
        let jconstructor = support_lib::support::get_method(jni_env, class, "<init>", "(ZBSIJFDLjava/lang/Boolean;Ljava/lang/Byte;Ljava/lang/Short;Ljava/lang/Integer;Ljava/lang/Long;Ljava/lang/Float;Ljava/lang/Double;)V");

        // Todo - handle local refs correctly.
        jni_invoke!(jni_env, NewLocalRef, jni_invoke!(jni_env, NewObject, class, jconstructor,
            bool::from_rust(jni_env, r.b).for_variadic(),
            i8::from_rust(jni_env, r.eight).for_variadic(),
            i16::from_rust(jni_env, r.sixteen).for_variadic(),
            i32::from_rust(jni_env, r.thirtytwo).for_variadic(),
            i64::from_rust(jni_env, r.sixtyfour).for_variadic(),
            f32::from_rust(jni_env, r.fthirtytwo).for_variadic(),
            f64::from_rust(jni_env, r.fsixtyfour).for_variadic(),

            Option::<bool>::from_rust(jni_env, r.o_b).for_variadic(),
            Option::<i8>::from_rust(jni_env, r.o_eight).for_variadic(),
            Option::<i16>::from_rust(jni_env, r.o_sixteen).for_variadic(),
            Option::<i32>::from_rust(jni_env, r.o_thirtytwo).for_variadic(),
            Option::<i64>::from_rust(jni_env, r.o_sixtyfour).for_variadic(),
            Option::<f32>::from_rust(jni_env, r.o_fthirtytwo).for_variadic(),
            Option::<f64>::from_rust(jni_env, r.o_fsixtyfour).for_variadic()
        ))
    }

    boxed_call_through!();
}