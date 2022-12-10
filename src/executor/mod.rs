pub mod ast;
mod datatypes;
use ast::*;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

use crate::executor::datatypes::DataTypes;
use crate::lexer::TokenType;

pub struct Executor {
    symbol_table: HashMap<usize, DataTypes>,
    literal_table: HashMap<usize, String>,
}

impl Executor {
    pub fn new(literal_table: HashMap<usize, String>) -> Self {
        Executor {
            symbol_table: HashMap::new(),
            literal_table,
        }
    }

    pub fn execute(&mut self, unit: &SourceUnit) -> Result<(), String> {
        for x in &unit.0 {
            let SourceUnitPart::Statement(stmt) = x;
            match stmt {
                Statement::Declaration(symbol) => {
                    if let Expression::Symbol(TokenType::Symbol(address)) = symbol {
                        if self.symbol_table.get(address).is_some() {
                            return Err("Symbol already defined!".to_string());
                        } else {
                            self.symbol_table.insert(*address, DataTypes::Unknown);
                        }
                    }
                }

                Statement::Write(expr) => {
                    print!("{}", self.eval_arithmetic_logic_expression(expr)?);
                    let _ = stdout().flush();
                }

                Statement::Assignment(l, r) => {
                    if let Expression::Symbol(TokenType::Symbol(address)) = l {
                        if !self.symbol_table.contains_key(address) {
                            return Err("Symbol already defined".to_string());
                        }
                        let data = self.eval_arithmetic_logic_expression(r)?;
                        self.symbol_table.insert(*address, data);
                    } else {
                        return Err("Invalid Assignment".to_string());
                    }
                }
            }
        }
        Ok(())
    }

    fn eval_arithmetic_logic_expression(&mut self, expr: &Expression) -> Result<DataTypes, String> {
        match expr {
            Expression::Add(l, r) => Ok(self.eval_arithmetic_logic_expression(&**l)?
                + self.eval_arithmetic_logic_expression(&**r)?),

            Expression::Multiply(l, r) => Ok(self.eval_arithmetic_logic_expression(&**l)?
                * self.eval_arithmetic_logic_expression(&**r)?),

            Expression::Subtract(l, r) => Ok(self.eval_arithmetic_logic_expression(&**l)?
                - self.eval_arithmetic_logic_expression(&**r)?),

            Expression::Divide(l, r) => Ok(self.eval_arithmetic_logic_expression(&**l)?
                / self.eval_arithmetic_logic_expression(&**r)?),

            Expression::Modulo(l, r) => Ok(self.eval_arithmetic_logic_expression(&**l)?
                % self.eval_arithmetic_logic_expression(&**r)?),

            Expression::Integer(TokenType::Integer(number)) => Ok(DataTypes::Integer(*number)),

            Expression::Symbol(TokenType::Symbol(address)) => {
                if !self.symbol_table.contains_key(address) {
                    return Err("Undefined Symbol".to_string());
                }

                match self.symbol_table.get(address) {
                    Some(DataTypes::Integer(number)) => Ok(DataTypes::Integer(*number)),
                    Some(DataTypes::String(data)) => Ok(DataTypes::String(data.to_string())),
                    _ => Err("UnIntialized Data".to_string()),
                }
            }

            Expression::StringLiteral(value) => {
                if let TokenType::Literal(address) = value {
                    Ok(DataTypes::String(self.literal_table[address].to_string()))
                } else {
                    Err("Invalid Expression".to_string())
                }
            }

            Expression::InputNumber => {
                let mut input = String::new();

                if stdin().read_line(&mut input).is_err() {
                    return Err("Error reading from stdin".to_string());
                }

                if let Ok(data) = input.trim().parse() {
                    Ok(DataTypes::Integer(data))
                } else {
                    Err("Invalid number entered!".to_string())
                }
            }

            Expression::InputString => {
                let mut input = String::new();
                stdin().read_line(&mut input).expect("Unable to read input");
                Ok(DataTypes::String(input))
            }
            _ => Err("Invalid Expression".to_string()),
        }
    }
}
