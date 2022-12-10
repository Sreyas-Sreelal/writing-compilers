mod executor;
mod lexer;
mod parser;
use executor::Executor;
use lexer::Lexer;
use parser::parse;
use std::collections::HashMap;
fn main() {
    let code = "
    write \"\ngood morning \";
    data i;
    i=2;
    write i+2 + 2*4;
    
    ";
    let mut lex = Lexer::new(code, HashMap::new(), 0);
    let parsed = parse(code, &mut lex);
    let mut exec = Executor::new(lex.literal_table);
    let _ = exec.execute(&parsed.unwrap());
}
