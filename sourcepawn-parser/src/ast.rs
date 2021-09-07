use sourcepawn_lexer::Token;

pub trait ASTNode { }

#[derive(Debug)]
pub struct TopLevelStatement {
    kind: TopLevelStatementKind,
}

#[derive(Debug)]
pub enum TopLevelStatementKind {
    FunctionDeclaration { name: Token, visibility: FunctionVisibility, statements: Vec<Statement> },
}

#[derive(Debug)]
pub enum FunctionVisibility {
    VisibilityPublic,
    VisibilityPrivate,
}

#[derive(Debug)]
pub enum Statement {
    Assignment
}
