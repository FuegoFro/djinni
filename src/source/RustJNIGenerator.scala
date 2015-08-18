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

  override def generateEnum(origin: String, ident: Ident, doc: Doc, e: Enum) {
    writeFile(ident.name, origin, (w: IndentWriter) => {})
  }

  override def generateRecord(origin: String, ident: Ident, doc: Doc, params: Seq[TypeParam], r: Record) {
    writeFile(ident.name, origin, (w: IndentWriter) => {
      val rustName = idRust.ty(ident)
      val nativeName = "Native" + rustName

      w.wl("#[macro_use]")
      w.wl("use support_lib;")
      w.wl("use support_lib::support::JType;")
      w.wl("use support_lib::jni_ffi::{JNIEnv, jobject};") // This list shouldn't be hardcoded
      w.wl(s"use generated_rust::${idRust.module(ident)}::$rustName;")
      w.wl
      w.wl(s"pub struct $nativeName;")
      w.wl(s"impl JType for $nativeName").braced {
        w.wl(s"type RustType = $rustName;")
        w.wl("type JniType = jobject;")
        w.wl
        w.wl("#[allow(non_snake_case)]")
        w.w("fn to_rust(jni_env: *mut JNIEnv, j: Self::JniType) -> Self::RustType").braced {
          w.wl("// TODO(rustgen): have a local scope here")
          w.wl("// TODO(rustgen): use a helper to get the class/methods so they're cached")
          val classLookup = q(jniMarshal.undecoratedTypename(ident, r))
          w.wl(s"let class = support_lib::support::get_class(jni_env, $classLookup);")
          for (f <- r.fields) {
            val javaFieldName = idJava.field(f.ident)
            val javaSig = q(jniMarshal.fqTypename(f.ty))
            w.wl(s"let field_$javaFieldName = support_lib::support::get_method(jni_env, class, ${q(javaFieldName)}, $javaSig);")
          }
          w.wl
          w.wl("assert!(j != 0 as jobject);")
          w.w(rustName).braced {
            for (f <- r.fields) {
              val rustField = idRust.field(f.ident)
              val fieldId = "field_" + idJava.field(f.ident)
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
          w.wl("// TODO(rustgen): translate backwards")
          w.wl("0 as jobject")
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
    writeFile(ident.name, origin, (w: IndentWriter) => {})
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