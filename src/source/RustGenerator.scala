package djinni

import djinni.ast._
import djinni.generatorTools.Spec
import djinni.writer.IndentWriter

class RustGenerator(spec: Spec) extends Generator(spec) {

  val marshal = new RustMarshal(spec)

  def writeFile(name: String, origin: String, f: IndentWriter => Unit) = writeRustFileGeneric(spec.rustOutFolder.get)(name, origin, f)

  override def generateEnum(origin: String, ident: Ident, doc: Doc, e: Enum) {
    writeFile(ident.name, origin, (w: IndentWriter) => {})
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
            case e: AssertionError => w.wl(s"// field $fieldName not implemented")
          }
        }
      }
    })
  }

  override def generateInterface(origin: String, ident: Ident, doc: Doc, typeParams: Seq[TypeParam], i: Interface) {
    writeFile(ident.name, origin, (w: IndentWriter) => {})
  }

}