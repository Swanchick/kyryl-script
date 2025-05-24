use std::io;

use crate::parser::statement::Statement;
use crate::parser::data_type::DataType;

use super::interpreter::Interpreter;
use super::return_value::Return;
use super::value::{Value, ValueType};
use super::interpret_expression::InterpretExpression;

pub struct InterpretStatement<'a> {
    interpreter: &'a mut Interpreter
}

impl<'a> InterpretStatement<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> InterpretStatement<'a> {
        InterpretStatement { interpreter: interpreter }
    }

    pub fn interpret_statement(&mut self, statement: Statement) -> io::Result<Return> {        
        match statement {
            Statement::VariableDeclaration { name, data_type, value } => {
                let value = if let Some(expression) = value {
                    self.interpreter.interpret_expression(expression)?
                } else {
                    Value::new(None, ValueType::Null)
                };

                if let Some(data_type) = data_type {
                    let value_data_type = value.get_type().get_data_type();

                    if value_data_type != data_type && !DataType::is_void(&value_data_type) {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "Variable declaration type mismatch!"))
                    }
                }

                self.interpreter.define_variable(name.as_str(), value)?;

                Ok(Return::Nothing)
            },
            Statement::Assigment { name, value } => {
                let value = self.interpreter.interpret_expression(value)?;

                self.interpreter.assign_variable(&name, value)?;

                Ok(Return::Nothing)
            },
            Statement::AssigmentIndex { name, index, value } => {
                let list_value = self.interpreter.get_variable(&name)?;
                let list_value_type = list_value.get_type().clone();
                let value_to_assign = self.interpreter.interpret_expression(value)?;

                let mut indeces: Vec<ValueType> = Vec::new();

                for i in index {
                    let value = self.interpreter.interpret_expression(i)?;
                    let value_type = value.get_type().clone();
                    
                    if value_type.get_data_type() != DataType::Int {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "Wrong type"));
                    }
        
                    indeces.push(value_type);
                }

                match list_value_type {
                    ValueType::List(_) => {
                        self.interpret_assign_list_index(&name, list_value_type, indeces, value_to_assign)?;
                    }
                    ValueType::String(mut str) => {
                        self.interpret_assign_string_index(&name, &mut str, indeces, value_to_assign)?;
                    }
                    _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid data type!"))
                }

                Ok(Return::Nothing)
            }
            Statement::AddValue { name, value } => {
                let value = self.interpreter.interpret_expression(value)?;
                self.interpret_add_equal(&name, value)?;

                Ok(Return::Nothing)
            },
            Statement::RemoveValue { name, value } => {
                let value = self.interpreter.interpret_expression(value)?;
                self.interpret_minus_equal(&name, value)?;

                Ok(Return::Nothing)
            },
            Statement::ReturnStatement { value } => {
                if let Some(expression) = value {
                    Ok(Return::Success(self.interpreter.interpret_expression(expression)?))
                } else {
                    Ok(Return::Success(Value::new(None, ValueType::Null)))
                }
            },
            Statement::IfStatement { condition, body, else_body } => {
                let value = self.interpreter.interpret_expression(condition)?;
                let value_type = value.get_type().clone();
                if let ValueType::Boolean(condition) = value_type {
                    if condition {
                        let value = self.interpret_block(body)?;
                        if let Return::Success(value) = value {
                            return Ok(Return::Success(value));
                        }
                    } else {
                        if let Some(body) = else_body {
                            let value = self.interpret_block(body)?;

                            if let Return::Success(value) = value {
                                return Ok(Return::Success(value));
                            }
                        }
                    }
                    
                    Ok(Return::Nothing)
                } else {
                    Err(io::Error::new(io::ErrorKind::InvalidData, "Not boolean type in if condition"))
                }
            },
            Statement::WhileStatement { condition, body } => {
                let value = self.interpreter.interpret_expression(condition.clone())?;
                let value_type = value.get_type();

                if let ValueType::Boolean(boolean) = value_type {
                    let mut boolean = boolean.clone();
                    
                    while boolean {
                        let return_value = self.interpret_block(body.clone())?;
                        if let Return::Success(return_value) = return_value {
                            return Ok(Return::Success(return_value));
                        }

                        let value = self.interpreter.interpret_expression(condition.clone())?;
                        let value_type = value.get_type();
                        if let ValueType::Boolean(new_boolean) = value_type {
                            boolean = new_boolean.clone();
                        }
                    }
                }

                Ok(Return::Nothing)
            },

            Statement::ForLoopStatement { name, list, body } => {
                let list = self.interpreter.interpret_expression(list)?;
                let list_type = list.get_type();
                
                self.interpret_for_loop(name, list_type, body)?;
                
                Ok(Return::Nothing)
            }
            Statement::Expression { value } => {
                self.interpreter.interpret_expression(value)?;

                Ok(Return::Nothing)
            },
            Statement::Function { name, return_type, parameters, body } => {
                self.interpreter.define_variable(name.clone().as_str(), Value::new(None, ValueType::Function { name: name, return_type: return_type, parameters: parameters, body: body }))?;
                
                Ok(Return::Nothing)
            }
        } 
    }

    fn interpret_for_loop(&mut self, name: String, list_value: &ValueType, body: Vec<Statement>) -> io::Result<()> {
        match list_value {
            ValueType::String(str) => {
                for char in str.chars() {
                    self.interpreter.define_variable(name.as_str(), Value::new(None, ValueType::String(char.to_string())))?;

                    self.interpret_block(body.clone())?;
                }
                
                Ok(())
            },
            ValueType::List(list) => {
                for value in list {
                    self.interpreter.define_variable(name.as_str(), value.clone())?;

                    self.interpret_block(body.clone())?;
                }

                Ok(())
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unsupported type for loop!"))
        }
    }

    fn interpret_assign_string_index(&mut self, name: &str, str: &mut String, indeces: Vec<ValueType>, value_to_assign: Value) -> io::Result<()> {
        if indeces.len() > 1 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Required only one index in string!"));
        }

        let value_type = value_to_assign.get_type();

        if let ValueType::String(c) = value_type {
            if c.len() != 1 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Required only char but not a string to change a char in a string!"));
            }
            
            let index = &indeces[0];
            if let ValueType::Integer(index) = index {
                if *index < 0 || *index > (str.len() - 1) as i32 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Out of bounds!"))
                }

                let index = *index as usize;
                
                str.replace_range(index..(index+1), &c);

                self.interpreter.assign_variable(&name, Value::new(None, ValueType::String(str.to_owned())))?;
            }
        }

        Ok(())
    }

    fn interpret_assign_list_index(&mut self, name: &str, list_value: ValueType, indeces: Vec<ValueType>, value_to_assign: Value) -> io::Result<()> {        
        let mut results: Vec<ValueType> = Vec::new();

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
                    if let ValueType::Integer(n) = current_index {
                        if let ValueType::List(mut list) = value.clone() {
                            if n < 0 || n > (list.len() - 1) as i32 {
                                return Err(io::Error::new(io::ErrorKind::InvalidData, "Out of bounds!"));
                            }
                            
                            list[n as usize] = v.clone();
                            last_index = last_index - 1;
                            result = Some(Value::new(None, ValueType::List(list)));
                        }
                    }
                },
                _ => {
                    result = Some(value_to_assign.clone());
                }
            }
        }

        if let ValueType::List(mut list) = list_value {
            let index = indeces[0].clone();
            
            if let ValueType::Integer(index) = index {
                list[index as usize] = result.unwrap();

                self.interpreter.assign_variable(name, Value::new(None, ValueType::List(list)))?;
            }
        }

        Ok(())
    }

    fn interpret_add_equal(&mut self, name: &str, value: Value) -> io::Result<()> {
        let original_value = self.interpreter.get_variable(&name)?; 
        let reference = original_value.get_reference();

        match (original_value.get_type().clone(), value.get_type().clone()) {
            (ValueType::Integer(n1), ValueType::Integer(n2)) => {
                let value = Value::new(reference, ValueType::Integer(n1 + n2));

                self.interpreter.assign_variable(name, value)?;
            },
            (ValueType::Float(n1), ValueType::Integer(n2)) => {
                let value = Value::new(reference, ValueType::Float(n1 + (n2.clone() as f64)));

                self.interpreter.assign_variable(name, value)?;
            },
            (ValueType::Integer(n1), ValueType::Float(n2)) => {
                let value = Value::new(reference, ValueType::Float((n1.clone() as f64) + n2));

                self.interpreter.assign_variable(name, value)?;
            },
            (ValueType::Float(n1), ValueType::Float(n2)) => {
                let value = Value::new(reference,  ValueType::Float(n1 + n2));

                self.interpreter.assign_variable(name, value)?;
            },
            (ValueType::String(mut str1), ValueType::String(str2)) => {
                str1.push_str(&str2);
                let value = Value::new(reference, ValueType::String(str1));

                self.interpreter.assign_variable(name, value)?;
            },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }

        Ok(())
    }

    fn interpret_minus_equal(&mut self, name: &str, value: Value) -> io::Result<()> {
        let original_value = self.interpreter.get_variable(&name)?; 
        let reference = original_value.get_reference();

        match (original_value.get_type().clone(), value.get_type().clone()) {
            (ValueType::Integer(n1), ValueType::Integer(n2)) => {
                let value = Value::new(reference, ValueType::Integer(n1 - n2));

                self.interpreter.assign_variable(name, value)?;
            },
            (ValueType::Float(n1), ValueType::Integer(n2)) => {
                let value = Value::new(reference, ValueType::Float(n1 - (n2.clone() as f64)));

                self.interpreter.assign_variable(name, value)?;
            },
            (ValueType::Integer(n1), ValueType::Float(n2)) => {
                let value = Value::new(reference, ValueType::Float((n1.clone() as f64) - n2));

                self.interpreter.assign_variable(name, value)?;
            },
            (ValueType::Float(n1), ValueType::Float(n2)) => {
                let value = Value::new(reference,  ValueType::Float(n1 - n2));

                self.interpreter.assign_variable(name, value)?;
            },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }

        Ok(())
    }

    fn interpret_block(&mut self, body: Vec<Statement>) -> io::Result<Return> {
        for statement in body {
            let value = self.interpret_statement(statement)?;

            if let Return::Success(value) = value {
                return Ok(Return::Success(value));
            }
        }

        Ok(Return::Nothing)
    }
}