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
      val rustName = idRust.ty(ident)
      val nativeName = "Native" + rustName

      w.wl("#[macro_use(jni_invoke)]")
      w.wl("use support_lib;")
      w.wl("use support_lib::support::{JType, ForVaridaic};")
      w.wl("use support_lib::jni_ffi::{JNIEnv, jobject};") // This list shouldn't be hardcoded
      w.wl(s"use generated_rust::${idRust.module(ident)}::$rustName;")
      w.wl
      w.wl(s"pub struct $nativeName;")
      w.wl(s"impl JType for $nativeName").braced {
        w.wl(s"type RustType = $rustName;")
        w.wl("type JniType = jobject;")
        w.wl
        w.w("fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self::RustType").braced {
          w.wl("// TODO(rustgen): have a local scope here")
          w.wl("// TODO(rustgen): use a helper to get the class/methods so they're cached")
          val classLookup = q(jniMarshal.undecoratedTypename(ident, r))
          w.wl(s"let class = support_lib::support::get_class(jni_env, $classLookup);")
          for (f <- r.fields) {
            val rustField = idRust.field(f.ident)
            val javaFieldName = idJava.field(f.ident)
            val javaSig = q(jniMarshal.fqTypename(f.ty))
            w.wl(s"let field_$rustField = support_lib::support::get_method(jni_env, class, ${q(javaFieldName)}, $javaSig);")
          }
          w.wl
          w.wl("assert!(j != 0 as jobject);")
          w.w(rustName).braced {
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
        w.wl
        w.w("fn from_rust(jni_env: *mut JNIEnv, r: Self::RustType) -> Self::JniType").braced {
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
        }
        w.wl
        w.w("fn to_rust_boxed(jni_env: *mut JNIEnv, j: jobject) -> Self::RustType").braced {
          w.wl("Self::to_rust(jni_env, j)")
        }
        w.wl
        w.w("fn from_rust_boxed(jni_env: *mut JNIEnv, r: Self::RustType) -> jobject").braced {
          w.wl("Self::from_rust(jni_env, r)")
        }
      }
    })
  }

  override def generateInterface(origin: String, ident: Ident, doc: Doc, typeParams: Seq[TypeParam], i: Interface) {
    writeFile(ident.name, origin, (w: IndentWriter) => {
      val rustModule = idRust.module(ident)
      w.wl("use support_lib::support::JType;")
      w.wl("use support_lib::jni_ffi::{JNIEnv, jobject, jclass};")
      w.wl(s"use $rustModule;")
      w.wl("use generated_rust_jni;")

      if (i.ext.rust) {
        // Generate CEXPORT functions for JNI to call.
        val classIdentMunged = javaMarshal.fqTypename(ident, i)
          .replaceAllLiterally("_", "_1")
          .replaceAllLiterally(".", "_")
        val prefix = "Java_" + classIdentMunged
        def nativeFn(name: String, params: Iterable[Field], ret: Option[TypeRef], f: => Unit) = {
          w.wl
          w.wl("#[no_mangle]")
          w.wl("#[inline(never)]")
          w.wl("#[allow(non_snake_case)]")

          val paramList = params.map(p => "j_" + idJava.local(p.ident) + ": " + jniMarshal.paramType(p.ty)).mkString(", ")
          val jniRetType = ret.fold("")(r => " -> " + jniMarshal.fqReturnType(Some(r)))
          val methodNameMunged = name.replaceAllLiterally("_", "_1")
          w.w(s"""pub extern "C" fn ${prefix}_$methodNameMunged(jni_env: *mut JNIEnv, _class: jclass${preComma(paramList)})$jniRetType""").braced {
            f
          }
        }

        // Put all static methods first, outside of the trait.
        for (m <- i.methods) {
          if (m.static) {
            nativeFn(idJava.method(m.ident), m.params, m.ret, {
              val methodName = idRust.method(m.ident)
              val ret = m.ret.fold("")(r => "let r = ")
              val call = s"$rustModule::$methodName("
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
    })
  }

  def toJniCall(ty: TypeRef, f: String => String): String = toJniCall(ty.resolved, f, false)
  def toJniCall(m: MExpr, f: String => String, needRef: Boolean): String = m.base match {
    case p: MPrimitive => f(if (needRef) "Object" else IdentStyle.camelUpper(p.jName))
    case MString => "(jstring)" + f("Object")
    case MOptional => toJniCall(m.args.head, f, true)
    case MBinary => "(jbyteArray)" + f("Object")
    case _ => f("Object")
  }
}