package djinni

import djinni.ast.Record.DerivingType
import djinni.ast._
import djinni.generatorTools._
import djinni.writer.IndentWriter

class RustGenerator(spec: Spec) extends Generator(spec) {

  val marshal = new RustMarshal(spec)

  def writeFile(name: String, origin: String, f: IndentWriter => Unit) = writeRustFileGeneric(spec.rustOutFolder.get)(name, origin, f)

  def generateModule(idl: Seq[TypeDecl]): Unit = {
    createFile(spec.rustOutFolder.get, "mod.rs", (w: IndentWriter) => {
      for (td <- idl.collect{ case itd: InternTypeDecl => itd }) {
        w.wl(s"pub mod ${idRust.module(td.ident)};")
      }
    })
  }

  override def generateEnum(origin: String, ident: Ident, doc: Doc, e: Enum) {
    writeFile(ident.name, origin, (w: IndentWriter) => {
      val rustName = idRust.ty(ident)
      w.w(s"pub enum $rustName").braced {
        for (o <- e.options) {
          // TODO(rustgen): write documentation
          w.wl(idRust.enum(o.ident) + ",")
        }
      }
    })
  }

  override def generateRecord(origin: String, ident: Ident, doc: Doc, params: Seq[TypeParam], r: Record) {
    writeFile(ident.name, origin, (w: IndentWriter) => {
      val rustName = idRust.ty(ident)
      w.w(s"pub struct $rustName").braced {
        for (f <- r.fields) {
          // TODO(rustgen): write documentation
          val fieldName = idRust.field(f.ident)
          try {
            val fieldType = marshal.fieldType(f.ty)
            w.wl(s"pub $fieldName: $fieldType,")
          } catch {
            case e: AssertionError => w.wl(s"// would be $fieldName, but " + e.getMessage)
          }
        }
      }
      if (r.derivingTypes.contains(DerivingType.Eq)) {
        w.wl
        w.wl("// TODO(rustgen): deriving eq")
      }
      if (r.derivingTypes.contains(DerivingType.Ord)) {
        w.wl
        w.wl("// TODO(rustgen): deriving ord")
      }
    })
  }

  override def generateInterface(origin: String, ident: Ident, doc: Doc, typeParams: Seq[TypeParam], i: Interface) {
    writeFile(ident.name, origin, (w: IndentWriter) => {
      val rustName = idRust.ty(ident)
      // TODO(rustgen): imports
      w.w(s"pub trait $rustName").braced {
        for (f <- i.methods) {
          val methodName = idRust.method(f.ident)
          try {
            def paramFormat(param: Field) = idRust.field(param.ident) + ": " + marshal.paramType(param.ty)
            var params = f.params.map(paramFormat).mkString(", ")
            params = if (f.static) params else "&self" + preComma(params)
            val returnType = marshal.returnType(f.ret)
            w.wl(s"fn $methodName($params)$returnType;")
          } catch {
            case e: AssertionError => w.wl(s"// would be $methodName, but " + e.getMessage)
          }
        }
        if (i.methods.nonEmpty && i.consts.nonEmpty) w.wl
        for (c <- i.consts) {
          val constName = idRust.const(c.ident)
          w.wl(s"// const $constName goes here")
        }
      }
    })
  }

}