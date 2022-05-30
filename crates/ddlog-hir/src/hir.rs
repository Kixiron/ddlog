// TODO: Auto-generate this
// TODO: Arenas

use ddlog_diagnostics::IStr;
use stable_hash::StableHash;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub enum Item {
    FuncDef(FuncDef),
    ClauseDef(ClauseDef),
    RelationDef(RelationDef),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub struct ClauseDef {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub struct RelationDef {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub struct FuncDef {
    name: IStr,
    // generics: Vec<GenericParam>,
    params: Vec<FuncParam>,
    body: Vec<Stmt>,
    return_ty: Type,
}

impl FuncDef {
    pub fn new(name: IStr, params: Vec<FuncParam>, body: Vec<Stmt>, return_ty: Type) -> Self {
        Self {
            name,
            params,
            body,
            return_ty,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub struct FuncParam {
    binding: Pattern,
    ty: Type,
}

impl FuncParam {
    pub fn new(binding: Pattern, ty: Type) -> Self {
        Self { binding, ty }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub enum Pattern {
    VarRef(IStr),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub enum Stmt {
    Expr {
        expr: Expr,
        semicolon_terminated: bool,
    },
    VarDecl(VarDecl),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub struct VarDecl {
    binding: Pattern,
    ty: Option<Type>,
    value: Expr,
}

impl VarDecl {
    pub fn new(binding: Pattern, ty: Option<Type>, value: Expr) -> Self {
        Self { binding, ty, value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub enum Expr {
    BinOp(BinExpr),
    Match(Match),
    VarRef(IStr),
    Block(Vec<Stmt>),
    Literal(Literal),
    Return(Box<Self>),
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub struct FunctionCall {
    func: Box<Expr>,
    args: Vec<Expr>,
}

impl FunctionCall {
    pub fn new(func: Box<Expr>, args: Vec<Expr>) -> Self {
        Self { func, args }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub enum Literal {
    Unit,
    Bool(bool),
    Number(u128),
    String(IStr),
    Error(Option<LiteralTypeHint>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub enum LiteralTypeHint {
    Bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub struct Match {
    scrutinee: Box<Expr>,
    arms: Vec<MatchArm>,
}

impl Match {
    pub fn new(scrutinee: Box<Expr>, arms: Vec<MatchArm>) -> Self {
        Self { scrutinee, arms }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub struct MatchArm {
    binding: Pattern,
    guard: Option<Box<Expr>>,
    body: Box<Expr>,
}

impl MatchArm {
    pub fn new(binding: Pattern, guard: Option<Box<Expr>>, body: Box<Expr>) -> Self {
        Self {
            binding,
            guard,
            body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub struct BinExpr {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
    op: BinOp,
}

impl BinExpr {
    pub fn new(lhs: Box<Expr>, rhs: Box<Expr>, op: BinOp) -> Self {
        Self { lhs, rhs, op }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub enum BinOp {
    Or,
    And,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Less,
    LessEq,
    Greater,
    GreaterEq,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub enum Type {
    GenericType(GenericType),
    TupleType(TupleType),
    Unit,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub struct GenericType {
    path: Path,
    generics: Vec<Type>,
}

impl GenericType {
    pub fn new(path: Path, generics: Vec<Type>) -> Self {
        Self { path, generics }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub struct TupleType {
    elements: Vec<Type>,
}

impl TupleType {
    pub fn new(elements: Vec<Type>) -> Self {
        Self { elements }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, StableHash)]
pub struct Path {
    // TODO: TinyVec or maybe even intern these?
    segments: Vec<IStr>,
}

impl Path {
    pub fn new(segments: Vec<IStr>) -> Self {
        Self { segments }
    }
}
