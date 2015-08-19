package djinni

import djinni.ast.TypeRef
import djinni.generatorTools.Spec
import djinni.meta._

class RustMarshal(spec: Spec) extends Marshal(spec) {

  override def typename(tm: MExpr): String = toRustType(tm)
  override def fqTypename(tm: MExpr): String = throw new AssertionError("not implemented")

  override def paramType(tm: MExpr): String = toRustType(tm)
  override def fqParamType(tm: MExpr): String = throw new AssertionError("not implemented")

  // Returns "" if there's no return type, " -> Type" if there is.
  override def returnType(ret: Option[TypeRef]): String = ret.fold("")(ty => " -> " + toRustType(ty.resolved))
  override def fqReturnType(ret: Option[TypeRef]): String = throw new AssertionError("not implemented")

  override def fieldType(tm: MExpr): String = toRustType(tm)
  override def fqFieldType(tm: MExpr): String = throw new AssertionError("not implemented")

  def toRustType(scoped: Boolean)(tm: MExpr): String = {
    def args(tm: MExpr) =
      if (tm.args.isEmpty) ""
      else tm.args.map(toRustType(false)).mkString(if (scoped) "::<" else "<", ", ", ">")
    tm.base match {
      case MOptional =>
        assert(tm.args.size == 1)
        "Option" + args(tm)
      case e: MExtern => throw new AssertionError("MExtern not implemented")
      case o =>
        val base = o match {
          case p: MPrimitive => p.rustName
          case MString => "String"
          case MDate => "Tm"
          case MBinary => if (scoped) "Box::<[u8]>" else "Box<[u8]>"
          case MOptional => throw new AssertionError("optional should have been special cased")
          case MList => "Vec"
          case MSet => "HashSet"
          case MMap => "HashMap"
          case d: MDef =>
            d.defType match {
              case DEnum => "::generated_rust::" + idRust.ty(d.name)
              case DRecord => "::generated_rust::" + idRust.ty(d.name)
              case DInterface => interfaceName(d.name, scoped)
            }
          case e: MExtern => throw new AssertionError("extern should have been special cased")
          case p: MParam => throw new AssertionError("MParam not implemented")
        }
        base + args(tm)
      }
  }
  def toRustType(tm: MExpr): String = toRustType(false)(tm)

  def interfaceName(name: String, scoped: Boolean = false): String =
    s"Arc${if(scoped)"::" else ""}<Box<::generated_rust::${idRust.ty(name)}>>"

}