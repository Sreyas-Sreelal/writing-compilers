mod grammar;
use crate::executor::ast;
use crate::lexer::Lexer;

pub fn parse<'a>(src: &'a str, mut tokens: &mut Lexer<'a>) -> Result<ast::SourceUnit, String> {
    match grammar::SourceUnitParser::new().parse(src, &mut tokens) {
        Ok(parsed) => Ok(parsed),
        Err(err) => Err(format!("Invalid syntax! {:?}", err)),
    }
}
