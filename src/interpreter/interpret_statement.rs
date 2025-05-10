use std::io;

use crate::parser::statement::Statement;
use crate::parser::data_type::DataType;

use super::interpreter::Interpreter;
use super::value::Value;
use super::interpret_expression::InterpretExpression;

pub struct InterpretStatement<'a> {
    interpreter: &'a mut Interpreter
}

impl<'a> InterpretStatement<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> InterpretStatement<'a> {
        InterpretStatement { interpreter: interpreter }
    }

    pub fn interpret_statement(&mut self, statement: Statement) -> io::Result<Option<Value>> {
        let local = self.interpreter.get_local();
        
        match statement {
            Statement::VariableDeclaration { name, data_type, value } => {
                let value = if let Some(expression) = value {
                    self.interpreter.interpret_expression(expression)?
                } else {
                    Value::Void
                };

                if let Some(data_type) = data_type {
                    if value.get_data_type() != data_type {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "Variable declaration type mismatch!"))
                    }
                }

                local.borrow_mut().define_variable(name, value);

                Ok(None)
            },
            Statement::Assigment { name, value } => {
                let value = self.interpreter.interpret_expression(value)?;

                local.borrow_mut().assign_variable(&name, value)?;

                Ok(None)
            },
            Statement::AssigmentIndex { name, index, value } => {
                let list_value = local.borrow().get_variable(&name)?;
                let value_to_assign = self.interpreter.interpret_expression(value)?;

                let mut indeces: Vec<Value> = Vec::new();

                for i in index {
                    let value = self.interpreter.interpret_expression(i)?;
                    if value.get_data_type() != DataType::Int {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "Wrong type"));
                    }
        
                    indeces.push(value);
                }

                match list_value {
                    Value::List(_) => {
                        self.interpret_assign_list_index(&name, list_value, indeces, value_to_assign)?;
                    }
                    Value::String(mut str) => {
                        self.interpret_assign_string_index(&name, &mut str, indeces, value_to_assign)?;
                    }
                    _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid data type!"))
                }

                Ok(None)
            }
            Statement::AddValue { name, value } => {
                let value = self.interpreter.interpret_expression(value)?;
                self.interpret_add_equal(&name, value)?;

                Ok(None)
            },
            Statement::RemoveValue { name, value } => {
                let value = self.interpreter.interpret_expression(value)?;
                self.interpret_minus_equal(&name, value)?;

                Ok(None)
            },
            Statement::ReturnStatement { value } => {
                if let Some(expression) = value {
                    Ok(Some(self.interpreter.interpret_expression(expression)?))
                } else {
                    Ok(Some(Value::Void))
                }
            },
            Statement::IfStatement { condition, body, else_body } => {
                let value = self.interpreter.interpret_expression(condition)?;

                if let Value::Boolean(condition) = value {
                    if condition {
                        let value = self.interpret_block(body)?;
                        if let Some(value) = value {
                            return Ok(Some(value));
                        }
                    } else {
                        if let Some(body) = else_body {
                            let value = self.interpret_block(body)?;

                            if let Some(value) = value {
                                return Ok(Some(value));
                            }
                        }
                    }
                    
                    Ok(None)
                } else {
                    Err(io::Error::new(io::ErrorKind::InvalidData, "Not boolean type in if condition"))
                }
            },
            Statement::WhileStatement { condition, body } => {
                let value = self.interpreter.interpret_expression(condition.clone())?;

                if let Value::Boolean(boolean) = value {
                    let mut boolean = boolean;
                    
                    while boolean {
                        let return_value = self.interpret_block(body.clone())?;
                        if let Some(return_value) = return_value {
                            return Ok(Some(return_value));
                        }

                        let value = self.interpreter.interpret_expression(condition.clone())?;
                        if let Value::Boolean(new_boolean) = value {
                            boolean = new_boolean;
                        }
                    }
                }

                Ok(None)
            },

            Statement::ForLoopStatement { name, list, body } => {
                let list = self.interpreter.interpret_expression(list)?;
                
                self.interpret_for_loop(name, list, body)?;
                
                Ok(None)
            }
            Statement::Expression { value } => {
                self.interpreter.interpret_expression(value)?;

                Ok(None)
            }
        } 
    }

    fn interpret_for_loop(&mut self, name: String, list_value: Value, body: Vec<Statement>) -> io::Result<()> {
        let local = self.interpreter.get_local();
        
        match list_value {
            Value::String(str) => {
                for char in str.chars() {
                    local.borrow_mut().define_variable(name.to_string(), Value::String(char.to_string()));

                    self.interpret_block(body.clone())?;
                }
                
                Ok(())
            },
            Value::List(list) => {
                for value in list {
                    local.borrow_mut().define_variable(name.to_string(), value);

                    self.interpret_block(body.clone())?;
                }

                Ok(())
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unsupported type for loop!"))
        }
    }

    fn interpret_assign_string_index(&mut self, name: &str, str: &mut String, indeces: Vec<Value>, value_to_assign: Value) -> io::Result<()> {
        if indeces.len() > 1 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Required only one index in string!"));
        }

        let local = self.interpreter.get_local();

        if let Value::String(c) = value_to_assign {
            if c.len() != 1 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Required only char but not a string to change a char in a string!"));
            }
            
            let index = &indeces[0];
            if let Value::Integer(index) = index {
                if *index < 0 || *index > (str.len() - 1) as i32 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Out of bounds!"))
                }

                let index = *index as usize;
                
                str.replace_range(index..(index+1), &c);

                local.borrow_mut().assign_variable(&name, Value::String(str.to_owned()))?;
            }
        }

        Ok(())
    }

    fn interpret_assign_list_index(&mut self, name: &str, list_value: Value, indeces: Vec<Value>, value_to_assign: Value) -> io::Result<()> {        
        let local = self.interpreter.get_local();
        
        let mut results: Vec<Value> = Vec::new();

        let expression = InterpretExpression::new(self.interpreter);

        for i in 0..indeces.len() {
            let index = indeces[i].clone();
            if results.len() != 0 {
                results.push(expression.interpret_identifier_index(results.last().unwrap().clone(), index.clone())?);
            } else {
                results.push(expression.interpret_identifier_index(list_value.clone(), index.clone())?);
            }
        }

        let mut final_results = vec![list_value.clone()];
        final_results.append(&mut results);
        
        let mut result: Option<Value> = None;
        let mut last_index = final_results.len() - 2;

        for fi in 0..(final_results.len() - 1) {
            let value = &final_results[final_results.len() - 1 - fi];
            
            match result.clone() {
                Some(v) => {
                    let current_index = indeces[last_index as usize].clone();
                    if let Value::Integer(n) = current_index {
                        if let Value::List(mut list) = value.clone() {
                            if n < 0 || n > (list.len() - 1) as i32 {
                                return Err(io::Error::new(io::ErrorKind::InvalidData, "Out of bounds!"));
                            }
                            
                            list[n as usize] = v.clone();
                            last_index = last_index - 1;
                            result = Some(Value::List(list));
                        }
                    }
                },
                _ => {
                    result = Some(value_to_assign.clone());
                }
            }
        }

        if let Value::List(mut list) = list_value {
            let index = indeces[0].clone();
            
            if let Value::Integer(index) = index {
                list[index as usize] = result.unwrap();

                local.borrow_mut().assign_variable(name, Value::List(list))?;
            }
        }

        Ok(())
    }

    fn interpret_add_equal(&mut self, name: &str, value: Value) -> io::Result<()> {
        let local = self.interpreter.get_local();

        let original_value = local.borrow().get_variable(&name)?;

        match (original_value, value) {
            (Value::Integer(n1), Value::Integer(n2)) => {
                let value = Value::Integer(n1 + n2);

                local.borrow_mut().assign_variable(name, value)?;
            },
            (Value::Float(n1), Value::Integer(n2)) => {
                let value = Value::Float(n1 + (n2 as f64));

                local.borrow_mut().assign_variable(name, value)?;
            },
            (Value::Integer(n1), Value::Float(n2)) => {
                let value = Value::Float((n1 as f64) + n2);

                local.borrow_mut().assign_variable(name, value)?;
            },
            (Value::Float(n1), Value::Float(n2)) => {
                let value = Value::Float(n1 + n2);

                local.borrow_mut().assign_variable(name, value)?;
            },
            (Value::String(mut str1), Value::String(str2)) => {
                str1.push_str(&str2);
                let value = Value::String(str1);

                local.borrow_mut().assign_variable(name, value)?;
            },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }

        Ok(())
    }

    fn interpret_minus_equal(&mut self, name: &str, value: Value) -> io::Result<()> {
        let local = self.interpreter.get_local();
        let original_value = local.borrow().get_variable(&name)?;

        match (original_value, value) {
            (Value::Integer(n1), Value::Integer(n2)) => {
                let value = Value::Integer(n1 - n2);

                local.borrow_mut().assign_variable(name, value)?;
            },
            (Value::Float(n1), Value::Integer(n2)) => {
                let value = Value::Float(n1 - (n2 as f64));

                local.borrow_mut().assign_variable(name, value)?;
            },
            (Value::Integer(n1), Value::Float(n2)) => {
                let value = Value::Float((n1 as f64) - n2);

                local.borrow_mut().assign_variable(name, value)?;
            },
            (Value::Float(n1), Value::Float(n2)) => {
                let value = Value::Float(n1 - n2);

                local.borrow_mut().assign_variable(name, value)?;
            },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }

        Ok(())
    }

    fn interpret_block(&mut self, body: Vec<Statement>) -> io::Result<Option<Value>> {
        for statement in body {
            let value = self.interpret_statement(statement)?;

            if let Some(value) = value {
                return Ok(Some(value));
            }
        }

        Ok(None)
    }
}