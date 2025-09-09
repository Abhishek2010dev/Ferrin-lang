pub enum Expression {}

pub enum Statement {
    Let { name: String, value: Expression },
}

pub struct Program {
    statements: Vec<Statement>,
}
