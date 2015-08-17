package djinni

import djinni.ast._
import djinni.generatorTools.Spec

class RustGenerator(spec: Spec) extends Generator(spec) {

  def writeFile(name: String, origin: String) = writeRustFileGeneric(spec.rustOutFolder.get)(name, origin)

  override def generateEnum(origin: String, ident: Ident, doc: Doc, e: Enum) {
    writeFile(ident.name, origin)
  }

  override def generateRecord(origin: String, ident: Ident, doc: Doc, params: Seq[TypeParam], r: Record) {
    writeFile(ident.name, origin)
  }

  override def generateInterface(origin: String, ident: Ident, doc: Doc, typeParams: Seq[TypeParam], i: Interface) {
    writeFile(ident.name, origin)
  }

}