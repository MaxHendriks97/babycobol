#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Atomic(Atomic),
    Assign {
        name: String,
        value: Box<Atomic>,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atomic {
    IntegerLiteral(i32),
    StringLiteral(String),
    Identifier(Identifier),
    BooleanLiteral(BooleanExpr),
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
        giving: Option<DivideGiving>,
    },
    Evaluate {
        expression: Expr,
        also: Option<Vec<Expr>>,
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
    value: Atomic,
    delimited_by: DelimitedBy,
    literal: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DelimitedBy {
    Size,
    Space,
    Literal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DivideGiving {
    giving: Vec<Identifier>,
    remainder: Identifier,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WhenClause {
    Expr(WhenExprStruct),
    Other,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhenExprStruct {
    expr: Expr,
    through: Option<Expr>,
    also: Box<WhenExprStruct>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Move {
    Atomic(Atomic),
    HighValues,
    LowValues,
    Spaces,
}
