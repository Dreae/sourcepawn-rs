use sourcepawn_lexer::Symbol;

pub struct Program {
    pub statements: Vec<TopLevelStatement>
}

pub struct Lit {
    kind: LitKind
}

pub enum LitKind {
    Int(i32),
    Float(f32),
    Str(Symbol),
    Bool(bool),
    Char(char)
}

pub struct Function {
    statements: Vec<Statement>,
    params: Vec<Param>
}

pub struct FunctionDecl {
    params: Vec<Param>
}

pub struct TopLevelStatement {
    kind: TopLevelStatementKind
}

pub enum TopLevelStatementKind {
    FunctionDecl(FunctionDecl),
    Function(Function),
    Local(Local),
}

pub struct Param {
    name: Symbol,
    ty: Symbol,
    default: Option<Lit>
}

pub struct Statement {
    kind: StatementKind
}

pub enum StatementKind {
    Local(Local),
    Expression(Expression)
}

pub struct Expression {

}

pub struct Local {
    kind: LocalKind,
    name: Symbol
}

pub enum LocalKind {
    Decl,
    Init(Expression)
}
