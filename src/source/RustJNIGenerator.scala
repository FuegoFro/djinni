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
    writeFile(ident.name, origin, (w: IndentWriter) => {})
  }

  override def generateRecord(origin: String, ident: Ident, doc: Doc, params: Seq[TypeParam], r: Record) {
    writeFile(ident.name, origin, (w: IndentWriter) => {
      val fqRustName = s"::generated_rust::${idRust.module(ident)}::${idRust.ty(ident)}"

      w.wl("#[macro_use(jni_invoke)]")
      w.wl("use support_lib;")
      w.wl("use support_lib::support::{JType, ForVaridaic};")
      w.wl("use support_lib::jni_ffi::{JNIEnv, jobject};") // This list shouldn't be hardcoded
      w.wl
      jTypeImpl(fqRustName, w, toRust = (w: IndentWriter) => {
        w.wl("// TODO(rustgen): have a local scope here")
        w.wl("// TODO(rustgen): use a helper to get the class/methods so they're cached")
        val classLookup = q(jniMarshal.undecoratedTypename(ident, r))
        w.wl(s"let class = support_lib::support::get_class(jni_env, $classLookup);")
        for (f <- r.fields) {
          val rustField = idRust.field(f.ident)
          val javaFieldName = idJava.field(f.ident)
          val javaSig = q(jniMarshal.fqTypename(f.ty))
          w.wl(s"let field_$rustField = support_lib::support::get_field(jni_env, class, ${q(javaFieldName)}, $javaSig);")
        }
        w.wl
        w.wl("assert!(j != 0 as jobject);")
        w.w(fqRustName).braced {
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
      }, fromRust = (w: IndentWriter) => {
        w.wl("// TODO(rustgen): cache the class/methods")
        w.wl("// TODO(rustgen): class object should have a ref around it")
        val classLookup = q(jniMarshal.undecoratedTypename(ident, r))
        w.wl(s"let class = support_lib::support::get_class(jni_env, $classLookup);")
        val constructorSig = q(jniMarshal.javaMethodSignature(r.fields, None))
        w.wl(s"let jconstructor = support_lib::support::get_method(jni_env, class, ${q("<init>")}, $constructorSig);")
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
    writeFile(ident.name, origin, (w: IndentWriter) => {
      w.wl("use support_lib::support::JType;")
      w.wl("use support_lib::jni_ffi::{JNIEnv, jobject, jclass};")
      w.wl("use generated_rust_jni;")

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
        if (static) {
          w.w(s"""pub extern "C" fn ${prefix}_$methodNameMunged(jni_env: *mut JNIEnv, _class: jclass${preComma(paramList)})$jniRetType""").braced {
            w.wl("let jni_env = ::support_lib::support::jni_get_thread_env();")
            f
          }
        } else {
          w.w(s"""pub extern "C" fn ${prefix}_00024CppProxy_$methodNameMunged(jni_env: *mut JNIEnv, _: jobject, nativeRef: jlong${preComma(paramList)})""").braced {
            w.wl("let jni_env = ::support_lib::support::jni_get_thread_env();")
            f
          }
        }
      }

      if (i.ext.rust) {
        // Put all static methods first, outside of the trait.
        for (m <- i.methods) {
          if (m.static) {
            nativeFn(idJava.method(m.ident), true, m.params, m.ret, {
              val methodName = idRust.method(m.ident)
              val ret = m.ret.fold("")(r => "let r = ")
              val call = s"::$rustModule::$methodName("
              try {
                writeAlignedCall(w, ret + call, m.params, ")", p => jniMarshal.toRust(p.ty, "j_" + idJava.local(p.ident)))
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
      }

      w.wl
      val rustTrait = idRust.ty(ident)
      val boxedRustType = rustMarshal.interfaceName(ident)
      val javaProxy = rustTrait + "JavaProxy"
      val rustProxy = rustTrait + "CppProxy" // Named like this because that's the native-backed Java class we generate right now
      jTypeImpl(boxedRustType, w, toRust = (w: IndentWriter) => {
        w.wl(s"Arc::new(Box::new($javaProxy { javaRef: j }))")
      }, fromRust = (w: IndentWriter) => {
        w.wl("// TODO(rustgen): this")
        w.wl("0 as jobject")
      })
      w.wl
      if (i.ext.java) {
        w.w(s"struct $javaProxy").braced {
          w.wl("javaRef: jobject")
        }
        w.wl
        w.w(s"impl $rustTrait for $javaProxy").braced {
          for (m <- i.methods) {
            if (!m.static) {
              val rustMethodName = idRust.method(m.ident)
              val javaMethodName = idJava.method(m.ident)
              try {
                def paramFormat(param: Field) = "r_" + idRust.field(param.ident) + ": " + rustMarshal.paramType(param.ty)
                var params = "&self" + preComma(m.params.map(paramFormat).mkString(", "))
                val returnType = rustMarshal.returnType(m.ret)
                w.w(s"fn $rustMethodName($params)$returnType").braced {
                  w.wl("let jni_env = ::support_lib::support::jni_get_thread_env();")
                  w.wl("// TODO(rustgen): local scope")
                  w.wl("// TODO(rustgen): use helper to cache class object and method IDs")
                  val classLookup = q(jniMarshal.undecoratedTypename(ident, i))
                  w.wl(s"let class = ::support_lib::support::get_class(jni_env, $classLookup);")
                  val methodSig = q(jniMarshal.javaMethodSignature(m.params, m.ret))
                  w.wl(s"let jmethod = ::support_lib::support::get_method(jni_env, class, ${q(javaMethodName)}, $methodSig);")
                  w.wl("// TODO(rustgen): handle local refs correctly")
                  val call = m.ret.fold("jni_invoke!(jni_env, CallVoidMethod")(r => "let jret = " + toJniCall(r, (jt: String) => s"jni_invoke!(jni_env, Call${jt}Method"))
                  w.w(call)
                  w.w(", self.javaRef, jmethod")
                  if (m.params.nonEmpty) {
                    w.wl(",")
                    writeAlignedCall(w, " " * "jni_invoke!(".length, m.params, ")", p => {
                      jniMarshal.fromRust(p.ty, "r_" + idRust.field(p.ident))
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
      }

      if (i.ext.rust) {
        w.w(s"struct $rustProxy").braced {
          w.wl(s"rustRef: $boxedRustType")
        }
        for (m <- i.methods) {
          if (!m.static) {
            try {
              nativeFn(idJava.method(m.ident), false, m.params, m.ret, {
                w.wl(s"let rustRef = support_lib::support::CppProxyHandle::<$rustTrait>::get(nativeRef);")
                val methodName = idRust.method(m.ident)
                val ret = m.ret.fold("")(r => "let r = ")
                val call = s"rustRef.$methodName("
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
      w.w("fn from_rust(jni_env: *mut JNIEnv, r: Self)").braced {
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