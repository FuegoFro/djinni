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

  private def toRustType(tm: MExpr): String = {
      tm.base match {
        case MOptional =>
          assert(tm.args.size == 1)
          val arg = toRustType(tm.args.head)
          s"Optional<$arg>"
        case e: MExtern => throw new AssertionError("MExtern not implemented")
        case o =>
          val base = o match {
            case p: MPrimitive => p.rustName
            case MString => "String"
            case MDate => throw new AssertionError("MDate not implemented")
            case MBinary => "Box<[u8]>"
            case MOptional => throw new AssertionError("optional should have been special cased")
            case MList => throw new AssertionError("MList not implemented")
            case MSet => throw new AssertionError("MSet not implemented")
            case MMap => throw new AssertionError("MMap not implemented")
            case d: MDef => throw new AssertionError("MDef not implemented")
            case e: MExtern => throw new AssertionError("extern should have been special cased")
            case p: MParam => throw new AssertionError("MParam not implemented")
          }
          base
      }
  }
}