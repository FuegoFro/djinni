package djinni

import djinni.ast._
import djinni.generatorTools._
import djinni.meta._
import djinni.writer.IndentWriter

import scala.collection.mutable

class RustJNIGenerator(spec: Spec) extends Generator(spec) {

  val jniMarshal = new JNIMarshal(spec)
  val rustMarshal = new RustMarshal(spec)
  val javaMarshal = new JavaMarshal(spec)

  def writeFile(name: String, origin: String, f: IndentWriter => Unit) = writeRustFileGeneric(spec.rustJniOutFolder.get)(name, origin, f)

  private def writeImports(r: Record, w: IndentWriter, others: Set[String]) {
    val rustImports = unionOverRecord(r, rustMarshal.imports)
    val jniImports = unionOverRecord(r, jniMarshal.rustImports)
    (rustImports | jniImports | others).map(w.wl)
  }

  private def writeImports(i: Interface, w: IndentWriter, others: Set[String]): Unit = {
    val rustImports = unionOverInterface(i, rustMarshal.imports)
    val jniImports = unionOverInterface(i, jniMarshal.rustImports)
    (rustImports | jniImports | others).map(w.wl)
  }

  private def writeImports(e: Enum, w: IndentWriter, others: Set[String]): Unit = {
    others.map(w.wl)
  }

  def generateModule(idl: Seq[TypeDecl]): Unit = {
    createFile(spec.rustJniOutFolder.get, "mod.rs", (w: IndentWriter) => {
      for (td <- idl.collect{ case itd: InternTypeDecl => itd }) {
        if (!rustSkipGeneration(td)) w.wl(s"pub mod ${idRust.module(td.ident)};")
      }
    })
  }

  class RustJNIRefs(name: String) {
    var ffi = mutable.TreeSet[String]()
    var refs = mutable.TreeSet[String]()

    def find(ty: TypeRef) { find(ty.resolved) }
    def find(tm: MExpr) {
      tm.args.foreach(find)
      find(tm.base)
    }
    def find(m: Meta) = m match {
      case o: MOpaque =>
      case d: MDef =>
      case e: MExtern =>
      case _ =>
    }
  }

  override def generateEnum(origin: String, ident: Ident, doc: Doc, e: Enum) {
    writeFile(ident.name, origin, (w: IndentWriter) => {
      writeImports(e, w, Set(
        "use support_lib::support::JType;",
        "use support_lib::support::get_class;",
        "use support_lib::support::get_method;",
        "use support_lib::support::get_static_method;",
        "use support_lib::jni_ffi::JNIEnv;",
        "use support_lib::jni_ffi::jobject;",
        "use support_lib::jni_ffi::jint;"
      ) ++ rustMarshal.imports(MExpr(MDef(ident, 0, DEnum, e), Seq.empty)))
      w.wl

      val rustName = idRust.ty(ident)
      val className = jniMarshal.undecoratedTypename(ident, e)
      jTypeImpl(rustName, w, toRust = (w: IndentWriter) => {
        // TODO(rustgen) - proper nullity exception messages
        w.wl(s"assert!(j != 0 as Self::JniType, ${q("Unexpectedly null value")});")
        w.wl(s"let class = get_class(jni_env, ${q(className)});")
        w.wl("""let method_ordinal = get_method(jni_env, class, "ordinal", "()I");""")
        w.wl("let ordinal = jni_invoke!(jni_env, CallIntMethod, j, method_ordinal);")
        w.w("match ordinal").braced {
          for ((o, i) <- e.options.view.zipWithIndex) {
            w.wl(s"$i => $rustName::${idRust.enum(o.ident)},")
          }
          w.wl(s"""_ => panic!("Unknown ordinal value for enum $rustName: {}", ordinal),""")
        }
      }, fromRust = (w: IndentWriter) => {
        w.wl(s"let class = get_class(jni_env, ${q(className)});")
        w.wl(s"""let method_values = get_static_method(jni_env, class, "values", "()[L$className;");""")
        w.wl("let values = jni_invoke!(jni_env, CallStaticObjectMethod, class, method_values);")
        w.w("let ordinal = match r").bracedSemi {
          for ((o, i) <- e.options.view.zipWithIndex) {
            w.wl(s"$rustName::${idRust.enum(o.ident)} => $i,")
          }
        }
        w.wl("jni_invoke!(jni_env, GetObjectArrayElement, values, ordinal as jint)")
      })
    })
  }

  override def generateRecord(origin: String, ident: Ident, doc: Doc, params: Seq[TypeParam], r: Record) {
    if (rustSkipGeneration(r)) {
      return
    }
    writeFile(ident.name, origin, (w: IndentWriter) => {
      val fqRustName = s"::generated_rust::${idRust.module(ident)}::${idRust.ty(ident)}"

      w.wl("#[macro_use(jni_invoke)]")
      w.wl("use support_lib;")
      writeImports(r, w, Set(
        "use support_lib::support::JType;",
        "use support_lib::support::ForVariadic;",
        "use support_lib::support::get_class;",
        "use support_lib::support::get_field;",
        "use support_lib::support::get_method;",
        "use support_lib::jni_ffi::JNIEnv;",
        "use support_lib::jni_ffi::jobject;"
      ) ++ rustMarshal.imports(MExpr(MDef(ident, 0, DRecord, r), Seq.empty)))
      w.wl
      jTypeImpl(fqRustName, w, toRust = (w: IndentWriter) => {
        w.wl("// TODO(rustgen): have a local scope here")
        w.wl("// TODO(rustgen): use a helper to get the class/methods so they're cached")
        val classLookup = q(jniMarshal.undecoratedTypename(ident, r))
        w.wl(s"let class = get_class(jni_env, $classLookup);")
        for (f <- r.fields) {
          val rustField = idRust.field(f.ident)
          val javaFieldName = idJava.field(f.ident)
          val javaSig = q(jniMarshal.fqTypename(f.ty))
          w.wl(s"let field_$rustField = get_field(jni_env, class, ${q(javaFieldName)}, $javaSig);")
        }
        w.wl
        // TODO(rustgen) - proper nullity exception messages
        w.wl(s"assert!(j != 0 as Self::JniType, ${q("Unexpectedly null value")});")
        w.w(fqRustName)
        if (r.fields.nonEmpty) {
          w.braced {
            for (f <- r.fields) {
              val rustField = idRust.field(f.ident)
              val fieldId = "field_" + rustField
              val jniFieldAccess = toJniCall(f.ty, (jt: String) => s"jni_invoke!(jni_env, Get${jt}Field, j, $fieldId)")
              try {
                val toRust = jniMarshal.toRust(f.ty, jniFieldAccess)
                w.wl(s"$rustField: $toRust,")
              } catch {
                case e: AssertionError => w.wl(s"// would grab $fieldId, but ${e.getMessage}")
              }
            }
          }
        }
      }, fromRust = (w: IndentWriter) => {
        w.wl("// TODO(rustgen): cache the class/methods")
        w.wl("// TODO(rustgen): class object should have a ref around it")
        val classLookup = q(jniMarshal.undecoratedTypename(ident, r))
        w.wl(s"let class = get_class(jni_env, $classLookup);")
        val constructorSig = q(jniMarshal.javaMethodSignature(r.fields, None))
        w.wl(s"let jconstructor = get_method(jni_env, class, ${q("<init>")}, $constructorSig);")
        w.wl
        w.wl("// TODO(rustgen): handle local refs correctly")
        val call = "jni_invoke!(jni_env, NewLocalRef, jni_invoke!("
        w.w(call + "jni_env, NewObject, class, jconstructor")
        if (r.fields.nonEmpty) {
          w.wl(",")
          writeAlignedCall(w, " " * call.length, r.fields, "))", f => {
            val name = idRust.field(f.ident)
            try {
              jniMarshal.fromRust(f.ty, s"r.$name") + ".for_variadic()"
            } catch {
              case e: AssertionError => s"// would grab $name, but ${e.getMessage}"
            }
          })
          w.wl
        } else {
          w.wl("))")
        }
      })
    })
  }

  override def generateInterface(origin: String, ident: Ident, doc: Doc, typeParams: Seq[TypeParam], i: Interface) {
    if (rustSkipGeneration(i)) {
      println(s"Skipping interface $ident}")
      return
    }
    writeFile(ident.name, origin, (w: IndentWriter) => {
      writeImports(i, w, Set(
        "use std::mem;",
        "use std::thread;",
        "use std::ptr::Unique;",
        "use support_lib::support::JType;",
        "use support_lib::support::jni_get_thread_env;",
        "use support_lib::support::get_class;",
        "use support_lib::support::get_field;",
        "use support_lib::support::get_method;",
        "use support_lib::support::throw_runtime_exception;",
        "use support_lib::support::GlobalRef;",
        "use support_lib::support::RustProxyable;",
        "use support_lib::support::ForVariadic;",
        "use support_lib::jni_ffi::JNIEnv;",
        "use support_lib::jni_ffi::jclass;",
        "use support_lib::jni_ffi::jobject;",
        "use support_lib::jni_ffi::jlong;",
        "use generated_rust_jni;"
      ) ++ rustMarshal.imports(MExpr(MDef(ident, 0, DInterface, i), Seq.empty)))

      val rustModule = idRust.module(ident)
      // Generate CEXPORT functions for JNI to call.
      val classIdentMunged = javaMarshal.fqTypename(ident, i)
        .replaceAllLiterally("_", "_1")
        .replaceAllLiterally(".", "_")
      val prefix = "Java_" + classIdentMunged
      def nativeFn(name: String, static: Boolean, params: Iterable[Field], ret: Option[TypeRef], f: => Unit) = {
        w.wl
        w.wl("#[no_mangle]")
        w.wl("#[inline(never)]")
        w.wl("#[allow(non_snake_case)]")

        val paramList = params.map(p => "j_" + idJava.local(p.ident) + ": " + jniMarshal.paramType(p.ty)).mkString(", ")
        val jniRetType = ret.fold("")(r => " -> " + jniMarshal.fqReturnType(Some(r)))
        val methodNameMunged = name.replaceAllLiterally("_", "_1")
        val declaration = if (static) {
          s"""pub extern "C" fn ${prefix}_$methodNameMunged(jni_env: *mut JNIEnv, _class: jclass${preComma(paramList)})$jniRetType"""
        } else {
          s"""pub extern "C" fn ${prefix}_00024CppProxy_$methodNameMunged(jni_env: *mut JNIEnv, _this: jobject, native_ref: jlong${preComma(paramList)})$jniRetType"""
        }

        w.w(declaration).braced {
          // Need to wrap up pointers into a Unique object so they are Send/Sync and passable to thread::catch_panic
          w.wl("let jni_env_wrapper: Unique<JNIEnv> = unsafe { Unique::new(jni_env) };")
          val paramsToWrap = params
            .filter(param => jniMarshal.isJavaHeapObject(param.ty))
            .map(param => s"j_${idJava.local(param.ident)}")
          paramsToWrap.map(paramName => w.wl(s"let $paramName: Unique<()> = unsafe { Unique::new($paramName) };"))

          w.w("let result = thread::catch_panic(move ||").bracedEnd(");") {
            // Once inside, unwrap them back to the original type.
            w.wl("let jni_env = *jni_env_wrapper;")
            paramsToWrap.map(paramName => w.wl(s"let $paramName = *$paramName;"))
            // Then print out the function
            f
          }
          // Now handle a panic if one occured
          w.w("match result").braced {
            if (ret.isDefined) {
              w.wl("Ok(value) => { return value; }")
            } else {
              w.wl("Ok(..) => {}")
            }
            w.w("Err(err) =>").braced {
              val defaultMessage = q("Caught panic in rust function")
              w.wl(s"let message: &str = err.downcast_ref::<&str>().unwrap_or(&$defaultMessage);")
              w.wl("// TODO(rustgen) - actually translate and preserve the panic, if possible")
              w.wl("throw_runtime_exception(jni_env, message);")
              if (ret.isDefined) {
                w.wl("// This value is ignored by JNI")
                w.wl(s"return 0 as ${jniMarshal.fqReturnType(ret)};")
              }
            }
          }
        }
      }

      if (i.ext.rust) {
        // Put all static methods first; they don't make use of the native proxy.
        // First we put the commented out function signatures, then we put the
        // actual implementations. This makes the static functions easier to implement.
        val hasStaticMethods = i.methods.exists(_.static)
        if (hasStaticMethods) {
          w.wl
          w.wl("/*")
            for (m <- i.methods if m.static) {
              val methodName = idRust.method(m.ident)
              val rustParams = m.params.map(p => s"${idRust.local(p.ident)}: ${rustMarshal.paramType(p.ty)}").mkString(", ")
              val rustReturn = rustMarshal.returnType(m.ret)
              w.w(s"pub fn $methodName($rustParams)$rustReturn").braced {
                w.wl // Blank line
              }
              w.wl
            }
          w.wl("*/")
        }
        for (m <- i.methods if m.static) {
          nativeFn(idJava.method(m.ident), true, m.params, m.ret, {
            val methodName = idRust.method(m.ident)
            val ret = m.ret.fold("")(r => "let r = ")
            val call = s"::$rustModule::$methodName("
            try {
              writeAlignedCall(w, ret + call, m.params, ");", p => jniMarshal.toRust(p.ty, "j_" + idJava.local(p.ident)))
            } catch {
              case e: AssertionError => w.wl(s"// tried to call through to static method ${m.ident.name}, but ${e.getMessage}")
            }
            w.wl
            try {
              m.ret.fold()(r => w.wl(jniMarshal.fromRust(r, "r")))
            } catch {
              case e: AssertionError => w.wl(s"// tried return from static method ${m.ident.name}, but ${e.getMessage}")
            }
          })
        }
      }

      w.wl
      val rustTrait = idRust.ty(ident)
      val boxedRustType = rustMarshal.interfaceName(ident)
      val javaProxy = rustTrait + "JavaProxy"
      val rustProxy = rustTrait + "CppProxy" // Named like this because that's the native-backed Java class we generate right now
      val classLookup = jniMarshal.undecoratedTypename(ident, i)
      w.wl("// TODO(rustgen): correct strong/weak Java references")
      w.wl("// TODO(rustgen): cache the proxies")
      val javaProxyClass: String = q(classLookup + "$CppProxy")
      jTypeImpl(boxedRustType, w, toRust = (w: IndentWriter) => {
        // TODO(rustgen) - proper nullity exception messages
        w.wl(s"assert!(j != 0 as Self::JniType, ${q("Unexpectedly null value")});")

        if (i.ext.rust) {
          w.wl(s"let proxy_class = get_class(jni_env, $javaProxyClass);")
          w.wl("let object_class = jni_invoke!(jni_env, GetObjectClass, j);")
          w.wl("let is_proxy = bool::to_rust(jni_env, jni_invoke!(jni_env, IsSameObject, proxy_class, object_class));")
        }
        if (i.ext.rust && i.ext.java) {
          w.wl("if is_proxy {")
          w.increase()
        }
        if (i.ext.rust) {
          w.wl("assert!(is_proxy);")
          w.wl("""let native_ref_field = get_field(jni_env, proxy_class, "nativeRef", "J");""")
          w.wl("let handle = jni_invoke!(jni_env, GetLongField, j, native_ref_field);")
          w.wl("*Self::from_handle(handle)")
        }
        if (i.ext.rust && i.ext.java) {
          w.decrease()
          w.wl("} else {")
          w.increase()
        }
        if (i.ext.java) {
          w.wl(s"Arc::new(Box::new($javaProxy { java_ref: GlobalRef::new(jni_env, j) }))")
        }
        if (i.ext.rust && i.ext.java) {
          w.decrease()
          w.wl("}")
        }
      }, fromRust = (w: IndentWriter) => {
        if (i.ext.java) {
          w.w(s"if let Some(proxy) = r.downcast_ref::<$javaProxy>()").braced {
            w.wl("return proxy.java_ref.get_local_ref(jni_env);")
          }
          if (!i.ext.rust) {
            w.wl(s"""panic!("Expected to get a $javaProxy passed in from Java")""")
          }
        }
        if (i.ext.rust) {
          w.wl("// Is not a Java proxy, need to create a new CppProxy")
          w.wl("// TODO(rustgen) - cache the CppProxys")
          w.wl(s"let class = ::support_lib::support::get_class(jni_env, $javaProxyClass);")
          w.wl("let constructor = ::support_lib::support::get_method(jni_env, class, \"<init>\", \"(J)V\");")
          w.wl("let handle = Self::to_handle(r.clone());")
          w.wl("jni_invoke!(jni_env, NewObject, class, constructor, handle)")
        }
      })
      if (i.ext.rust) {
        w.wl
        w.w(s"impl RustProxyable for Arc<Box<$rustTrait>>").braced {
          w.w("fn to_handle(self) -> jlong").braced {
            w.wl("// Convert our box into a pointer, leaving the memory there and not running")
            w.wl("// the destructor on the contents. We can't use Box::into_raw since it's unstable.")
            w.wl("unsafe { mem::transmute(Box::new(self)) }")
          }
          w.w("fn from_handle(rust_proxy_handle: jlong) -> Box<Self>").braced {
            w.wl("// Convert our pointer back into a box. We can't use Box::from_raw since it's unstable.")
            w.wl("// Letting the result of this destruct means the the rust_proxy_handle that was passed in")
            w.wl("// to this function is no longer valid.")
            w.wl("unsafe { mem::transmute(rust_proxy_handle as *mut Arc<Box<Self>>) }")
          }
          w.w("fn copy_from_handle(rust_proxy_handle: jlong) -> Self").braced {
            w.wl("// Returns another Arc pointer to the object referrenced by the rust_proxy_handle. It")
            w.wl("// is fine to let the result of this destruct; the rust_proxy_handle passed in will")
            w.wl("// remain valid.")
            w.wl("let rust_ref = Self::from_handle(rust_proxy_handle);")
            w.wl("let copy: Self = (*rust_ref).clone();")
            w.wl("// Forget the memory because we don't want the destructor to run yet.")
            w.wl("mem::forget(rust_ref);")
            w.wl("copy")
          }
        }
      }
      if (i.ext.java) {
        w.wl
        w.w(s"struct $javaProxy").braced {
          w.wl("java_ref: GlobalRef")
        }
        w.wl
        w.w(s"impl $rustTrait for $javaProxy").braced {
          for (m <- i.methods if !m.static) {
            val rustMethodName = idRust.method(m.ident)
            val javaMethodName = idJava.method(m.ident)
            try {
              def paramFormat(param: Field) = "r_" + idRust.field(param.ident) + ": " + rustMarshal.paramType(param.ty)
              var params = "&self" + preComma(m.params.map(paramFormat).mkString(", "))
              val returnType = rustMarshal.returnType(m.ret)
              w.w(s"fn $rustMethodName($params)$returnType").braced {
                w.wl("let jni_env = jni_get_thread_env();")
                w.wl("// TODO(rustgen): local scope")
                w.wl("// TODO(rustgen): use helper to cache class object and method IDs")
                w.wl(s"let class = get_class(jni_env, ${q(classLookup)});")
                val methodSig = q(jniMarshal.javaMethodSignature(m.params, m.ret))
                w.wl(s"let jmethod = get_method(jni_env, class, ${q(javaMethodName)}, $methodSig);")
                w.wl("// TODO(rustgen): handle local refs correctly")
                val call = m.ret.fold("jni_invoke!(jni_env, CallVoidMethod")(r => "let jret = " + toJniCall(r, (jt: String) => s"jni_invoke!(jni_env, Call${jt}Method"))
                w.w(call)
                w.w(", self.java_ref.get_local_ref(jni_env), jmethod")
                if (m.params.nonEmpty) {
                  w.wl(",")
                  writeAlignedCall(w, " " * "jni_invoke!(".length, m.params, ")", p => {
                    jniMarshal.fromRust(p.ty, "r_" + idRust.field(p.ident)) + ".for_variadic()"
                  })
                }
                else w.w(")")
                w.wl(";")
                m.ret.fold()(r => {
                  w.wl(jniMarshal.toRust(r, "jret"))
                })
              }
            } catch {
              case e: AssertionError => w.wl(s"// would be $rustMethodName, but ${e.getMessage}")
            }
          }
        }
      }

      if (i.ext.rust) {
        w.wl
        w.w(s"struct $rustProxy").braced {
          w.wl(s"rust_ref: $boxedRustType")
        }
        nativeFn("nativeDestroy", false, Seq.empty, None, {
          w.wl(s"let _to_delete: Box<Arc<Box<$rustTrait>>> = Arc::<Box<$rustTrait>>::from_handle(native_ref);")
          w.wl("// Let the destructor run on the Box and its Arc when _to_delete goes out of scope.")
        })
        for (m <- i.methods if !m.static) {
          try {
            nativeFn("native_" + idJava.method(m.ident), false, m.params, m.ret, {
              w.wl(s"let rust_ref: Arc<Box<$rustTrait>> = Arc::<Box<$rustTrait>>::copy_from_handle(native_ref);")
              val methodName = idRust.method(m.ident)
              val ret = m.ret.fold("")(r => "let r = ")
              val call = s"rust_ref.$methodName("
              try {
                writeAlignedCall(w, ret + call, m.params, ");", p => jniMarshal.toRust(p.ty, "j_" + idJava.local(p.ident)))
              } catch {
                case e: AssertionError => w.wl(s"// tried to call through to method ${m.ident.name}, but ${e.getMessage}")
              }
              w.wl
              try {
                m.ret.fold()(r => w.wl(jniMarshal.fromRust(r, "r")))
              } catch {
                case e: AssertionError => w.wl(s"// tried return from method ${m.ident.name}, but ${e.getMessage}")
              }
            })
          } catch {
            case e: AssertionError => w.wl(s"// tried method ${m.ident.name}, but ${e.getMessage}")
          }
        }
      }
    })
  }

  private def jTypeImpl(rustType: String, w: IndentWriter, toRust: IndentWriter => Unit, fromRust: IndentWriter => Unit): Unit = {
    w.w(s"impl JType for $rustType").braced {
      w.wl("type JniType = jobject;")
      w.wl
      w.w("fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self").braced {
        toRust(w)
      }
      w.wl
      w.w("fn from_rust(jni_env: *mut JNIEnv, r: Self) -> Self::JniType").braced {
        fromRust(w)
      }
      w.wl
      w.w("fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> Self").braced {
        w.wl("Self::to_rust(jni_env, j)")
      }
      w.wl
      w.w("fn from_rust_boxed(jni_env: *mut JNIEnv, r: Self) -> jobject").braced {
        w.wl("Self::from_rust(jni_env, r)")
      }
    }
  }

  def toJniCall(ty: TypeRef, f: String => String): String = toJniCall(ty.resolved, f, false)
  def toJniCall(m: MExpr, f: String => String, needRef: Boolean): String = m.base match {
    case p: MPrimitive => f(if (needRef) "Object" else IdentStyle.camelUpper(p.jName))
    case MString => f("Object")// + " as jstring"
    case MOptional => toJniCall(m.args.head, f, true)
    case MBinary => f("Object")// + " as jbyteArray"
    case _ => f("Object")
  }
}
