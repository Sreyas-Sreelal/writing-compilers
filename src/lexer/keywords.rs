use crate::lexer::TokenType;
use std::collections::HashMap;

pub struct Keywords {
    pub list: HashMap<String, TokenType>,
}

macro_rules! keywordize {
    ($( $words:expr => $func:expr ), *) => {{
        let mut list = HashMap::new();
        $( for &word in &($words) { list.insert(word.to_string(), $func); } )*
        list
    }};
}

impl Keywords {
    pub fn new() -> Self {
        let list = keywordize!(
            ["data"] => TokenType::Declaration,
            ["input_string"] => TokenType::InputString,
            ["input_number"] => TokenType::InputNumber,
            ["write"] => TokenType::Write
        );

        Self { list }
    }
}
