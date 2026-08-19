#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Atomic(Atomic),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atomic {
    Literal(Literal),
    Identifier(Identifier),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    IntegerLiteral(i32),
    StringLiteral(String),
    BooleanLiteral(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanExpr(pub bool);

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Procedure(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    GreaterThan,
    LessThan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Accept(Vec<Identifier>),
    Add {
        values: Vec<Atomic>,
        target: Atomic,
        giving: Option<Vec<Identifier>>,
    },
    Display {
        values: Vec<DisplayVal>,
        with_no_advancing: bool,
    },
    Divide {
        value: Atomic,
        into: Vec<Atomic>,
        divide_giving: Option<DivideGiving>,
    },
    Evaluate {
        expression: Expr,
        also: Vec<Expr>,
        when: Vec<(WhenClause, Vec<Stmt>)>,
    },
    If {
        val: BooleanExpr,
        then: Vec<Stmt>,
        alt: Option<Vec<Stmt>>,
    },
    Move {
        value: Move,
        to: Vec<Identifier>
    },
    Multiply {
        value: Atomic,
        by: Vec<Atomic>,
        giving: Option<Identifier>,
    },
    Perform {
        target: Procedure,
        through: Option<Procedure>,
        times: Option<Atomic>,
    },
    Stop,
    Subtract {
        value: Vec<Atomic>,
        target: Vec<Atomic>,
        giving: Option<Vec<Identifier>>,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DisplayVal {
    pub value: Atomic,
    pub delimited_by: Option<DelimitedBy>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DelimitedBy {
    Size,
    Space,
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq)]
pub struct DivideGiving {
    pub giving: Vec<Identifier>,
    pub remainder: Option<Identifier>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WhenClause {
    Expr(WhenClauseStruct),
    Other,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhenClauseStruct {
    pub when_exprs: Vec<WhenExprStruct>,
    pub also_exprs: Vec<Vec<WhenExprStruct>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhenExprStruct {
    pub expr: Expr,
    pub through: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Move {
    Atomic(Atomic),
    HighValues,
    LowValues,
    Spaces,
}
