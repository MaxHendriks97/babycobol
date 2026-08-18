use regex::Regex;
use strum::IntoEnumIterator;
use crate::lexing::token::Token;

pub fn lex(input: &str) -> Vec<Token> {
    let mut token_vec: Vec<Token> = Vec::new();
    let mut match_vec: Vec<(&Token, usize, usize)> = Vec::new();
    let current_input = input;

    let tokens: Vec<Token> = Token::iter().collect();

    for token in tokens.iter() {
        let token_regex = token.get_regex_from_token();
        let re = Regex::new(&token_regex).unwrap();
        for m in re.find_iter(current_input) {
            match_vec.push((token, m.start(), m.end()));
        }
    }

    // Sort matched tokens by length
    match_vec.sort_by(|a, b| {
        a.1.cmp(&b.1) // Sort by start
            .then_with(|| (b.2 - b.1).cmp(&(a.2 - a.1))) // Longer match wins
    });

    let mut last_end = 0;
    for (token, start, end) in match_vec {
        if start < last_end {
            continue; // Skip overlapping tokens
        }
        last_end = end;
        let lexeme = &current_input[start..end];
        token_vec.push(token.get_token(lexeme))
    }

    token_vec
}
