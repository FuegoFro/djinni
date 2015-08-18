package djinni

import djinni.ast._
import djinni.generatorTools.Spec
import djinni.writer.IndentWriter

class RustJNIGenerator(spec: Spec) extends Generator(spec) {

  def writeFile(name: String, origin: String, f: IndentWriter => Unit) = writeRustFileGeneric(spec.rustJniOutFolder.get)(name, origin, f)

  override def generateEnum(origin: String, ident: Ident, doc: Doc, e: Enum) {
    writeFile(ident.name, origin, (w: IndentWriter) => {})
  }

  override def generateRecord(origin: String, ident: Ident, doc: Doc, params: Seq[TypeParam], r: Record) {
    writeFile(ident.name, origin, (w: IndentWriter) => {})
  }

  override def generateInterface(origin: String, ident: Ident, doc: Doc, typeParams: Seq[TypeParam], i: Interface) {
    writeFile(ident.name, origin, (w: IndentWriter) => {})
  }

}