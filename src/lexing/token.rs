use strum_macros::EnumIter;

#[derive(Debug, Clone, EnumIter)]
pub enum Token {
    // keywords
    Accept(String),
    Add(String),
    Giving(String),
    Display(String),
    Delimited(String),
    By(String),
    Size(String),
    Space(String),
    With(String),
    No(String),
    Advancing(String),
    Divide(String),
    Into(String),
    Remainder(String),
    Evaluate(String),
    Also(String),
    When(String),
    Through(String),
    Other(String),
    End(String),
    If(String),
    Then(String),
    Else(String),
    Move(String),
    HighValues(String),
    LowValues(String),
    Spaces(String),
    Multiply(String),
    Perform(String),
    Times(String),
    Stop(String),
    Subtract(String),
    From(String),

    // literals
    IntegerLiteral(i32),
    StringLiteral(String),
    BooleanLiteral(bool),

    // identifier
    Identifier(String),

    // operators
    Assign(String),

    // punctuation
    Period(String),
    LeftParen(String),
    RightParen(String),

    // logical operator

    // comment
    Comment(String),
}

impl Token {
    pub fn get_token_old(token_type: &str, value: Option<&str>) -> Token {
        match token_type {
            // Keywords
            "ACCEPT" => Token::Accept("ACCEPT".to_string()),
            "ADD" => Token::Add("ADD".to_string()),
            "GIVING" => Token::Giving("GIVING".to_string()),
            "DISPLAY" => Token::Display("DISPLAY".to_string()),
            "DELIMITED" => Token::Delimited("DELIMITED".to_string()),
            "BY" => Token::By("BY".to_string()),
            "SIZE" => Token::Size("SIZE".to_string()),
            "SPACE" => Token::Space("SPACE".to_string()),
            "WITH" => Token::With("WITH".to_string()),
            "NO" => Token::No("NO".to_string()),
            "Advancing" => Token::Advancing("ADVANCING".to_string()),
            "DIVIDE" => Token::Divide("DIVIDE".to_string()),
            "INTO" => Token::Into("INTO".to_string()),
            "REMAINDER" => Token::Remainder("REMAINDER".to_string()),
            "EVALUATE" => Token::Evaluate("EVALUATE".to_string()),
            "ALSO" => Token::Also("ALSO".to_string()),
            "WHEN" => Token::When("WHEN".to_string()),
            "THROUGH" => Token::Through("THROUGH".to_string()),
            "OTHER" => Token::Other("OTHER".to_string()),
            "END" => Token::End("END".to_string()),
            "IF" => Token::If("IF".to_string()),
            "THEN" => Token::Then("THEN".to_string()),
            "ELSE" => Token::Else("ELSE".to_string()),
            "MOVE" => Token::Move("MOVE".to_string()),
            "HIGH-VALUES" => Token::HighValues("HIGH-VALUES".to_string()),
            "LOW-VALUES" => Token::LowValues("LOW-VALUES".to_string()),
            "SPACES" => Token::Spaces("SPACES".to_string()),
            "MULTIPLY" => Token::Multiply("MULTIPLY".to_string()),
            "PERFORM" => Token::Perform("PERFORM".to_string()),
            "TIMES" => Token::Times("TIMES".to_string()),
            "STOP" => Token::Stop("STOP".to_string()),
            "SUBTRACT" => Token::Subtract("SUBTRACT".to_string()),
            "FROM" => Token::From("FROM".to_string()),

            // literals
            "IntegerLiteral" => Token::IntegerLiteral(value.unwrap().parse::<i32>().unwrap()),
            "StringLiteral" => Token::StringLiteral(value.unwrap().to_string()),
            "BooleanLiteral" => {
                let val = match value.unwrap() {
                    "TRUE" => true,
                    "FALSE" => false,
                    _ => panic!("Invalid boolean literal"),
                };
                Token::BooleanLiteral(val)
            },

            // identifiers
            "IDENTIFIER" => Token::Identifier(value.unwrap().to_string()),

            // operators
            "ASSIGN" => Token::Assign("=".to_string()),

            // punctuation
            "PERIOD" => Token::Period(".".to_string()),
            "LEFTPAREN"  => Token::LeftParen("(".to_string()),
            "RIGHTPAREN" => Token::RightParen(")".to_string()),

            // logical operators

            _ => panic!("Invalid token type {}", token_type),
        }
    }

    pub fn get_token(&self, value: &str) -> Token {
        match self {
            Token::Accept(_) => Token::Accept(value.to_string()),
            Token::Add(_) => Token::Add(value.to_string()),
            Token::Giving(_) => Token::Giving(value.to_string()),
            Token::Display(_) => Token::Display(value.to_string()),
            Token::Delimited(_) => Token::Delimited(value.to_string()),
            Token::By(_) => Token::By(value.to_string()),
            Token::Size(_) => Token::Size(value.to_string()),
            Token::Space(_) => Token::Space(value.to_string()),
            Token::With(_) => Token::With(value.to_string()),
            Token::No(_) => Token::No(value.to_string()),
            Token::Advancing(_) => Token::Advancing(value.to_string()),
            Token::Divide(_) => Token::Divide(value.to_string()),
            Token::Into(_) => Token::Into(value.to_string()),
            Token::Remainder(_) => Token::Remainder(value.to_string()),
            Token::Evaluate(_) => Token::Evaluate(value.to_string()),
            Token::Also(_) => Token::Also(value.to_string()),
            Token::When(_) => Token::When(value.to_string()),
            Token::Through(_) => Token::Through(value.to_string()),
            Token::Other(_) => Token::Other(value.to_string()),
            Token::End(_) => Token::End(value.to_string()),
            Token::If(_) => Token::If(value.to_string()),
            Token::Then(_) => Token::Then(value.to_string()),
            Token::Else(_) => Token::Else(value.to_string()),
            Token::Move(_) => Token::Move(value.to_string()),
            Token::HighValues(_) => Token::HighValues(value.to_string()),
            Token::LowValues(_) => Token::LowValues(value.to_string()),
            Token::Spaces(_) => Token::Spaces(value.to_string()),
            Token::Multiply(_) => Token::Multiply(value.to_string()),
            Token::Perform(_) => Token::Perform(value.to_string()),
            Token::Times(_) => Token::Times(value.to_string()),
            Token::Stop(_) => Token::Stop(value.to_string()),
            Token::Subtract(_) => Token::Subtract(value.to_string()),
            Token::From(_) => Token::From(value.to_string()),

            // literals
            Token::IntegerLiteral(_) => Token::IntegerLiteral(value.parse::<i32>().unwrap()),
            Token::StringLiteral(_) => Token::StringLiteral(value.to_string()),
            Token::BooleanLiteral(_) => {
                let val = match value {
                    "TRUE" => true,
                    "FALSE" => false,
                    _ => panic!("Invalid boolean literal"),
                };
                Token::BooleanLiteral(val)
            },

            // identifiers
            Token::Identifier(_) => Token::Identifier(value.to_string()),

            // operators
            Token::Assign(_) => Token::Assign(value.to_string()),

            // punctuation
            Token::Period(_) => Token::Period(value.to_string()),
            Token::LeftParen(_)  => Token::LeftParen(value.to_string()),
            Token::RightParen(_) => Token::RightParen(value.to_string()),

            // logical operators

            // comment
            Token::Comment(_) => Token::Comment(value.to_string()),
        }
    }

    pub fn get_token_regex(token_type: &str) -> String {
        match token_type {
            "ACCEPT" => r"ACCEPT",
            "ADD" => r"ADD",
            "GIVING" => r"GIVING",
            "DISPLAY" => r"DISPLAY",
            "DELIMITED" => r"DELIMITED",
            "BY" => r"BY",
            "SIZE" => r"SIZE",
            "SPACE" => r"SPACE",
            "WITH" => r"WITH",
            "NO" => r"NO",
            "Advancing" => r"Advancing",
            "DIVIDE" => r"DIVIDE",
            "INTO" => r"INTO",
            "REMAINDER" => r"REMAINDER",
            "EVALUATE" => r"EVALUATE",
            "ALSO" => r"ALSO",
            "WHEN" => r"WHEN",
            "THROUGH" => r"THROUGH",
            "OTHER" => r"OTHER",
            "END" => r"END",
            "IF" => r"IF",
            "THEN" => r"THEN",
            "ELSE" => r"ELSE",
            "MOVE" => r"MOVE",
            "HIGH-VALUES" => r"HIGH-VALUES",
            "LOW-VALUES" => r"LOW-VALUES",
            "SPACES" => r"SPACES",
            "MULTIPLY" => r"MULTIPLY",
            "PERFORM" => r"PERFORM",
            "TIMES" => r"TIMES",
            "STOP" => r"STOP",
            "SUBTRACT" => r"SUBTRACT",
            "FROM" => r"FROM",

            // literals
            "IntegerLiteral" => r"\\d+",
            "StringLiteral" => r#"\\".*\\""#,
            "BooleanLiteral" => r"\\b(?:true|false)\\b",

            // identifiers
            "IDENTIFIER" => r"[a-zA-Z_][a-zA-Z0-9_]*",

            // operators
            "ASSIGN" => r"=",

            // punctuation
            "PERIOD" => r".",
            "LEFTPAREN"  => r"\(",
            "RIGHTPAREN" => r"\)",

            // logical operator

            // comment

            _ => panic!("Invalid token type: {}", token_type),
        }.to_string()
    }
    pub fn get_regex_from_token(&self) -> String {
        match self {
            Token::Accept(_) => r"ACCEPT",
            Token::Add(_) => r"ADD",
            Token::Giving(_) => r"GIVING",
            Token::Display(_) => r"DISPLAY",
            Token::Delimited(_) => r"DELIMITED",
            Token::By(_) => r"BY",
            Token::Size(_) => r"SIZE",
            Token::Space(_) => r"SPACE",
            Token::With(_) => r"WITH",
            Token::No(_) => r"NO",
            Token::Advancing(_) => r"Advancing",
            Token::Divide(_) => r"DIVIDE",
            Token::Into(_) => r"INTO",
            Token::Remainder(_) => r"REMAINDER",
            Token::Evaluate(_) => r"EVALUATE",
            Token::Also(_) => r"ALSO",
            Token::When(_) => r"WHEN",
            Token::Through(_) => r"THROUGH",
            Token::Other(_) => r"OTHER",
            Token::End(_) => r"END",
            Token::If(_) => r"IF",
            Token::Then(_) => r"THEN",
            Token::Else(_) => r"ELSE",
            Token::Move(_) => r"MOVE",
            Token::HighValues(_) => r"HIGH-VALUES",
            Token::LowValues(_) => r"LOW-VALUES",
            Token::Spaces(_) => r"SPACES",
            Token::Multiply(_) => r"MULTIPLY",
            Token::Perform(_) => r"PERFORM",
            Token::Times(_) => r"TIMES",
            Token::Stop(_) => r"STOP",
            Token::Subtract(_) => r"SUBTRACT",
            Token::From(_) => r"FROM",

            // literals
            Token::IntegerLiteral(_) => r"\d+",
            Token::StringLiteral(_) => r#"\".*\""#,
            Token::BooleanLiteral(_) => r"\b(?:true|false)\b",

            // identifiers
            Token::Identifier(_) => r"[a-zA-Z_][a-zA-Z0-9_]*",

            // operators
            Token::Assign(_) => r"=",

            // punctuation
            Token::Period(_) => r"\.",
            Token::LeftParen(_)  => r"\(",
            Token::RightParen(_) => r"\)",

            // logical operator

            // comment
            Token::Comment(_) => r"\*.*",
        }.to_string()
    }
}
