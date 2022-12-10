mod keywords;
use keywords::Keywords;
use std::collections::HashMap;
use std::str::Chars;

#[derive(Debug, Clone)]
pub enum TokenType {
    Declaration,
    Write,
    InputString,
    InputNumber,
    Assignment,
    Plus,
    Minus,
    Product,
    Divide,
    Modulo,
    SemiColon,
    OpenParantheses,
    CloseParantheses,
    Literal(usize),
    Integer(i64),
    Symbol(usize),
}

pub struct Lexer<'input> {
    chars: Chars<'input>,
    keywords: Keywords,
    pub src: &'input str,
    pub literal_table: HashMap<usize, String>,
    pub symbol_lookup: HashMap<String, usize>,
    pub lookup_count: usize,
    literal_count: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(
        input: &'input str,
        symbol_lookup: HashMap<String, usize>,
        lookup_count: usize,
    ) -> Self {
        Lexer {
            chars: input.chars(),
            keywords: Keywords::new(),
            src: input,
            literal_table: HashMap::new(),
            symbol_lookup,
            literal_count: 0,
            lookup_count,
        }
    }

    fn is_operator_or_special_character(&self, c: char) -> bool {
        c.is_whitespace()
            || c == '\n'
            || c == '\r'
            || c == ';'
            || c == '+'
            || c == '-'
            || c == '*'
            || c == '/'
            || c == '%'
            || c == ')'
            || c == '('
            || c == '='
    }

    fn is_valid_name(&self, c: char) -> bool {
        !self.is_operator_or_special_character(c)
    }
}

impl<'input> Iterator for &mut Lexer<'input> {
    type Item = Result<TokenType, &'static str>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some('=') => return Some(Ok(TokenType::Assignment)),
                Some('+') => return Some(Ok(TokenType::Plus)),
                Some('-') => return Some(Ok(TokenType::Minus)),
                Some('*') => return Some(Ok(TokenType::Product)),
                Some('/') => return Some(Ok(TokenType::Divide)),
                Some('%') => return Some(Ok(TokenType::Modulo)),
                Some(';') => return Some(Ok(TokenType::SemiColon)),
                Some('(') => return Some(Ok(TokenType::OpenParantheses)),
                Some(')') => return Some(Ok(TokenType::CloseParantheses)),
                Some('"') => {
                    let mut ch = ' ';
                    let mut word = String::new();
                    for c in self.chars.by_ref() {
                        ch = c;
                        if c == '"' {
                            break;
                        }
                        word.push(c);
                    }

                    if ch != '"' {
                        return Some(Err("Invalid String"));
                    }
                    self.literal_count += 1;
                    self.literal_table.insert(self.literal_count, word);
                    return Some(Ok(TokenType::Literal(self.literal_count)));
                }

                Some(c) if c.is_alphabetic() => {
                    let mut word = String::new();
                    word.push(c);
                    while let Some(c) = self.chars.clone().peekable().peek() {
                        if self.is_valid_name(*c) {
                            word.push(*c);
                            self.chars.next();
                        } else {
                            break;
                        }
                    }

                    if let Some(keyword) = &self.keywords.list.get(&word) {
                        return Some(Ok((**keyword).clone()));
                    } else if !self.symbol_lookup.contains_key(&word) {
                        self.lookup_count += 1;
                        self.symbol_lookup.insert(word.clone(), self.lookup_count);
                    }
                    return Some(Ok(TokenType::Symbol(
                        *(self.symbol_lookup.get(&word).unwrap()),
                    )));
                }

                Some(c) if c.is_ascii_digit() => {
                    let mut word = String::new();
                    word.push(c);
                    while let Some(c) = self.chars.clone().peekable().peek() {
                        if self.is_operator_or_special_character(*c) {
                            break;
                        }
                        word.push(*c);
                        self.chars.next();
                    }

                    if let Ok(number) = word.parse() {
                        return Some(Ok(TokenType::Integer(number)));
                    } else {
                        return Some(Err("Invalid Integer Constant"));
                    }
                }
                Some(c) => {
                    if c.is_whitespace() || c == '\n' || c == '\r' {
                        continue;
                    } else {
                        return Some(Err("Unknown Token"));
                    }
                }
                None => return None,
            }
        }
    }
}
