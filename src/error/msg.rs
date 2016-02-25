use self::Msg::*;
use class::ClassId;
use ctxt::Context;
use interner::Name;
use lexer::position::Position;
use ty::BuiltinType;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Msg {
    Unimplemented,
    UnknownType(String),
    UnknownIdentifier(Name),
    UnknownFunction(String),
    UnknownProp(String, BuiltinType),
    UnknownMethod(BuiltinType, Name, Vec<BuiltinType>),
    MethodExists(BuiltinType, Name, Vec<BuiltinType>, Position),
    IdentifierExists(String),
    ShadowFunction(String),
    ShadowParam(String),
    ShadowType(String),
    ShadowProp(String),
    VarNeedsTypeInfo(String),
    ParamTypesIncompatible(Name, Vec<BuiltinType>, Vec<BuiltinType>),
    WhileCondType(BuiltinType),
    IfCondType(BuiltinType),
    ReturnType(BuiltinType, BuiltinType),
    LvalueExpected,
    AssignType(String, BuiltinType, BuiltinType),
    AssignProp(Name, ClassId, BuiltinType, BuiltinType),
    UnOpType(String, BuiltinType),
    BinOpType(String, BuiltinType, BuiltinType),
    OutsideLoop,
    NoReturnValue,
    MainNotFound,
    WrongMainDefinition,
    ThisInFunction
}

impl Msg {
    pub fn message(&self, ctxt: &Context) -> String {
        match *self {
            Unimplemented => format!("feature not implemented yet."),
            UnknownType(ref name) => format!("no type with name `{}` known.", name),
            UnknownIdentifier(name) => {
                let name = ctxt.interner.str(name).to_string();
                format!("unknown identifier `{}`.", name)
            },
            UnknownFunction(ref name) => format!("unknown function `{}`", name),
            UnknownMethod(cls, name, ref args) => {
                let name = ctxt.interner.str(name).to_string();
                let cls = cls.name(ctxt);
                let args = args.iter().map(|a| a.name(ctxt)).collect::<Vec<String>>().connect(", ");

                format!("no method with definition `{}({})` in class `{}`.", name, args, cls)
            },
            MethodExists(cls, name, ref args, pos) => {
                let name = ctxt.interner.str(name).to_string();
                let cls = cls.name(ctxt);
                let args = args.iter().map(|a| a.name(ctxt)).collect::<Vec<String>>().connect(", ");

                format!(
                    "method with definition `{}({})` already exists in class `{}` at line {}.",
                    name, args, cls, pos)
            },
            UnknownProp(ref prop, ref ty) =>
                format!("unknown property `{}` for type `{}`", prop, ty.name(ctxt)),
            IdentifierExists(ref name) => format!("can not redefine identifier `{}`.", name),
            ShadowFunction(ref name) => format!("can not shadow function `{}`.", name),
            ShadowParam(ref name) => format!("can not shadow param `{}`.", name),
            ShadowType(ref name) => format!("can not shadow type `{}`.", name),
            ShadowProp(ref name) => format!("property with name `{}` already exists.", name),
            VarNeedsTypeInfo(ref name) =>
                format!("variable `{}` needs either type declaration or expression.", name),
            ParamTypesIncompatible(name, ref def, ref expr) => {
                let name = ctxt.interner.str(name).to_string();
                let def = def.iter().map(|a| a.name(ctxt)).collect::<Vec<String>>().connect(", ");
                let expr = expr.iter().map(|a| a.name(ctxt)).collect::<Vec<String>>().connect(", ");

                format!("function `{}({})` cannot be called as `{}({})`",
                    name, def, name, expr)
            },
            WhileCondType(ref ty) =>
                format!("`while` expects condition of type `bool` but got `{}`.", ty.name(ctxt)),
            IfCondType(ref ty) =>
                format!("`if` expects condition of type `bool` but got `{}`.", ty.name(ctxt)),
            ReturnType(ref def, ref expr) =>
                format!("`return` expects value of type `{}` but got `{}`.",
                    def.name(ctxt), expr.name(ctxt)),
            LvalueExpected => format!("lvalue expected for assignment"),
            AssignType(ref name, ref def, ref expr) =>
                format!("cannot assign `{}` to variable `{}` of type `{}`.",
                        &expr.name(ctxt), name, &def.name(ctxt)),
            AssignProp(name, clsid, ref def, ref expr) => {
                let cls = ctxt.cls_by_id(clsid);
                let cls_name = ctxt.interner.str(cls.name).to_string();
                let prop_name = ctxt.interner.str(name).to_string();

                format!("cannot assign `{}` to property `{}`.`{}` of type `{}`.",
                        &expr.name(ctxt), cls_name, prop_name, &def.name(ctxt))
            },
            UnOpType(ref op, ref expr) =>
                format!("unary operator `{}` can not handle value of type `{} {}`.", op, op,
                    &expr.name(ctxt)),
            BinOpType(ref op, ref lhs, ref rhs) =>
                format!("binary operator `{}` can not handle expression of type `{} {} {}`",
                    op, &lhs.name(ctxt), op, &rhs.name(ctxt)),
            OutsideLoop => "statement only allowed inside loops".into(),
            NoReturnValue => "function does not return a value in all code paths".into(),
            MainNotFound => "no `main` function found in the program".into(),
            WrongMainDefinition => "`main` function has wrong definition".into(),
            ThisInFunction => "`this` can only be used in methods not functions".into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MsgWithPos {
    pub msg: Msg,
    pub pos: Position,
}

impl MsgWithPos {
    pub fn new(pos: Position, msg: Msg) -> MsgWithPos {
        MsgWithPos {
            pos: pos,
            msg: msg
        }
    }

    pub fn message(&self, ctxt: &Context) -> String {
        format!("error at {}: {}", self.pos, self.msg.message(ctxt))
    }
}
