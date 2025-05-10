use std::io;

use crate::parser::expression::Expression;
use crate::parser::operator::Operator;
use crate::parser::data_type::DataType;

use super::{interpreter::Interpreter, value::Value};

pub struct InterpretExpression<'a> {
    interpreter: &'a mut Interpreter
}


impl<'a> InterpretExpression<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> InterpretExpression<'a> {
        InterpretExpression { interpreter: interpreter }
    }

    pub fn interpret_expression(&mut self, expression: Expression) -> io::Result<Value> {
        let local = self.interpreter.get_local();
        
        match expression {
            Expression::BinaryOp { left, operator, right } => {
                let left_value = self.interpret_expression(*left)?;
                let right_value  = self.interpret_expression(*right)?;

                self.interpret_binary_operation(left_value, right_value, operator)
            },
            Expression::UnaryOp { expression, operator } => {
                let value = self.interpret_expression(*expression)?;

                self.interpret_unary_operation(value, operator)
            },
            Expression::FrontUnaryOp { expression, operator } => {
                if let Expression::Identifier(name) = *expression {
                    self.interpret_front_unary_operation(&name, operator)
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Operator \"{:?}\" is used only with variable!", operator)
                    ))
                }
            },
            Expression::FunctionCall(name, parameters) => {
                let mut args: Vec<Value> = Vec::new();

                for parameter in parameters {
                    let value = self.interpret_expression(parameter)?;
                    args.push(value);
                }

                self.interpreter.call_function(&name, args)
            },
            Expression::ListLiteral(expressions) => {
                let mut values: Vec<Value> = Vec::new();
                let mut data_type: Option<DataType> = None;

                for expression in expressions {
                    let value = self.interpret_expression(expression)?;

                    if let Some(t) = &data_type {
                        if &value.get_data_type() != t {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, "List has different values. List should consist only of one type!"));
                        }
                    } else {
                        data_type = Some(value.get_data_type().clone())
                    }

                    values.push(value);
                }

                Ok(Value::List(values))
            },
            Expression::IdentifierIndex{ left, index } => {
                let left = self.interpret_expression(*left)?;
                let index = self.interpret_expression(*index)?;

                self.interpret_identifier_index(left, index)
            },
            Expression::IntegerLiteral(value) => {
                Ok(Value::Integer(value.clone()))
            },
            Expression::FloatLiteral(value) => {
                Ok(Value::Float(value.clone()))
            },
            Expression::BooleanLiteral(value) => {
                Ok(Value::Boolean(value.clone()))
            },
            Expression::StringLiteral(value) => {
                Ok(Value::String(value.clone()))
            },
            Expression::Identifier(name) => {
                

                local.borrow().get_variable(&name)
            }
        }
    }    

    pub fn interpret_identifier_index(&self, left: Value, index: Value) -> io::Result<Value> {
        if let Value::Integer(index) = index {
            match left {
                Value::String(str) => {
                    let character = str.chars().nth(index as usize);
                    if let Some(character) = character {
                        let value = Value::String(character.to_string());
    
                        Ok(value)
                    } else {
                        Err(io::Error::new(io::ErrorKind::InvalidData, "Out of bounds in string."))
                    }
                },
    
                Value::List(values) => {
                    let child_value = values.iter().nth(index as usize);
                    if let Some(child_value) = child_value {
                        Ok(child_value.clone())
                    } else {
                        Err(io::Error::new(io::ErrorKind::InvalidData, "Out of bounds!"))
                    }
                },
    
                _ => Err(io::Error::new(
                    io::ErrorKind::InvalidData, 
                    format!("Index operation requires lists or strings to get specific value from it! Instead got {}.", left.get_data_type())
                ))
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Index in list or string requires integer type! Instead got {}.", index.get_data_type()) 
            ))
        }        
    }

    fn interpret_front_unary_operation(&mut self, name: &str, operator: Operator) -> io::Result<Value> {
        match operator {
            Operator::PlusPlus => self.interpret_plus_plus(name),
            Operator::MinusMinus => self.interpret_minus_minus(name),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Front Unary Operation Error"))
        }
    }

    fn interpret_plus_plus(&mut self, name: &str) -> io::Result<Value> {
        let local = self.interpreter.get_local();

        let value = local.borrow().get_variable(name)?;

        match value {
            Value::Integer(number) => {
                let value = Value::Integer(number + 1);

                local.borrow_mut().assign_variable(name, value.clone())?;

                Ok(value)
            }
            Value::Float(number) => {
                let value = Value::Float(number + 1.0);
                local.borrow_mut().assign_variable(name, value.clone())?;

                Ok(value)
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Type \"{}\" is not supported by this operator!", value.get_data_type())
            ))
        }
    }

    fn interpret_minus_minus(&mut self, name: &str) -> io::Result<Value> {
        let local = self.interpreter.get_local();

        let value = local.borrow().get_variable(name)?;

        match value {
            Value::Integer(number) => {
                let value = Value::Integer(number - 1);

                local.borrow_mut().assign_variable(name, value.clone())?;

                Ok(value)
            }
            Value::Float(number) => {
                let value = Value::Float(number - 1.0);
                
                local.borrow_mut().assign_variable(name, value.clone())?;

                Ok(value)
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Type \"{}\" is not supported by this operator!", value.get_data_type())
            ))
        }
    }

    fn interpret_binary_operation(&self, left: Value, right: Value, operator: Operator) -> io::Result<Value> {
        match operator {
            Operator::Plus => self.interpret_plus(left, right),
            Operator::Minus => self.interpret_minus(left, right),
            Operator::Multiply => self.interpret_multiply(left, right),
            Operator::Divide => self.interpret_divide(left, right),
            Operator::EqualEqual => self.interpret_equal_equal(left, right),
            Operator::GreaterEqual => self.interpret_greater_equal(left, right),
            Operator::Greater => self.interpret_greater(left, right),
            Operator::LessEqual => self.interpret_less_equal(left, right),
            Operator::Less => self.interpret_less(left, right),
            Operator::NotEqual => self.interpret_tilde_equal(left, right),
            Operator::And => self.interpret_and(left, right),
            Operator::Or => self.interpret_or(left, right),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unsupported operator!"))
        }
    }

    fn interpret_equal_equal(&self, left: Value, right: Value) -> io::Result<Value> {
        match (left, right) {
            (Value::Integer(n1), Value::Integer(n2)) => {
                let value = Value::Boolean(n1 == n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Float(n2)) => {
                let value = Value::Boolean(n1 == n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Integer(n2)) => {
                let value = Value::Boolean(n1 == (n2 as f64));

                Ok(value)
            },
            (Value::Integer(n1), Value::Float(n2)) => {
                let value = Value::Boolean((n1 as f64) == n2);

                Ok(value)
            },
            (Value::String(str1), Value::String(str2)) => {
                let value = Value::Boolean(str1 == str2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_greater_equal(&self, left: Value, right: Value) -> io::Result<Value> {
        match (left, right) {
            (Value::Integer(n1), Value::Integer(n2)) => {
                let value = Value::Boolean(n1 >= n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Float(n2)) => {
                let value = Value::Boolean(n1 >= n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Integer(n2)) => {
                let value = Value::Boolean(n1 >= (n2 as f64));

                Ok(value)
            },
            (Value::Integer(n1), Value::Float(n2)) => {
                let value = Value::Boolean((n1 as f64) >= n2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_greater(&self, left: Value, right: Value) -> io::Result<Value> {
        match (left, right) {
            (Value::Integer(n1), Value::Integer(n2)) => {
                let value = Value::Boolean(n1 > n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Float(n2)) => {
                let value = Value::Boolean(n1 > n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Integer(n2)) => {
                let value = Value::Boolean(n1 > (n2 as f64));

                Ok(value)
            },
            (Value::Integer(n1), Value::Float(n2)) => {
                let value = Value::Boolean((n1 as f64) > n2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_less_equal(&self, left: Value, right: Value) -> io::Result<Value> {
        match (left, right) {
            (Value::Integer(n1), Value::Integer(n2)) => {
                let value = Value::Boolean(n1 <= n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Float(n2)) => {
                let value = Value::Boolean(n1 <= n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Integer(n2)) => {
                let value = Value::Boolean(n1 <= (n2 as f64));

                Ok(value)
            },
            (Value::Integer(n1), Value::Float(n2)) => {
                let value = Value::Boolean((n1 as f64) <= n2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_less(&self, left: Value, right: Value) -> io::Result<Value> {
        match (left, right) {
            (Value::Integer(n1), Value::Integer(n2)) => {
                let value = Value::Boolean(n1 < n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Float(n2)) => {
                let value = Value::Boolean(n1 < n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Integer(n2)) => {
                let value = Value::Boolean(n1 < (n2 as f64));

                Ok(value)
            },
            (Value::Integer(n1), Value::Float(n2)) => {
                let value = Value::Boolean((n1 as f64) < n2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_tilde_equal(&self, left: Value, right: Value) -> io::Result<Value> {
        match (left, right) {
            (Value::Integer(n1), Value::Integer(n2)) => {
                let value = Value::Boolean(n1 != n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Float(n2)) => {
                let value = Value::Boolean(n1 != n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Integer(n2)) => {
                let value = Value::Boolean(n1 != (n2 as f64));

                Ok(value)
            },
            (Value::Integer(n1), Value::Float(n2)) => {
                let value = Value::Boolean((n1 as f64) != n2);

                Ok(value)
            },
            (Value::String(str1), Value::String(str2)) => {
                let value = Value::Boolean(str1 != str2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_and(&self, left: Value, right: Value) -> io::Result<Value> {
        match (left, right) {
            (Value::Boolean(bool1), Value::Boolean(bool2)) => {
                let value = Value::Boolean(bool1 && bool2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_or(&self, left: Value, right: Value) -> io::Result<Value> {
        match (left, right) {
            (Value::Boolean(bool1), Value::Boolean(bool2)) => {
                let value = Value::Boolean(bool1 || bool2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_unary_operation(&self, value: Value, operator: Operator) -> io::Result<Value> {
        match operator {
            Operator::Minus => {
                self.interpret_negation(value)
            },

            Operator::Tilde => {
                self.interpret_not(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown unary operator!"))
        }
    }

    fn interpret_not(&self, value: Value) -> io::Result<Value> {
        match value {
            Value::Boolean(value) => Ok(Value::Boolean(!value)),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Wrong type for not inverting"))
        }
    }

    fn interpret_negation(&self, value: Value) -> io::Result<Value> {
        match value {
            Value::Integer(value) => Ok(Value::Integer(-value)),
            Value::Float(value) => Ok(Value::Float(-value)),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Wrong type for not negation"))
        }
    }

    fn interpret_plus(&self, left: Value, right: Value) -> io::Result<Value> {
        match (left, right) {
            (Value::Integer(n1), Value::Integer(n2)) => {
                let value = Value::Integer(n1 + n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Float(n2)) => {
                let value = Value::Float(n1 + n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Integer(n2)) => {
                let value = Value::Float(n1 + (n2 as f64));

                Ok(value)
            },
            (Value::Integer(n1), Value::Float(n2)) => {
                let value = Value::Float((n1 as f64) + n2);

                Ok(value)
            },
            (Value::String(mut str1), Value::String(str2)) => {
                str1.push_str(&str2);
                let value = Value::String(str1);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    } 

    fn interpret_minus(&self, left: Value, right: Value) -> io::Result<Value> {
        match (left, right) {
            (Value::Integer(n1), Value::Integer(n2)) => {
                let value = Value::Integer(n1 - n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Float(n2)) => {
                let value = Value::Float(n1 - n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Integer(n2)) => {
                let value = Value::Float(n1 - (n2 as f64));

                Ok(value)
            },
            (Value::Integer(n1), Value::Float(n2)) => {
                let value = Value::Float((n1 as f64) - n2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_multiply(&self, left: Value, right: Value) -> io::Result<Value> {
        match (left, right) {
            (Value::Integer(n1), Value::Integer(n2)) => {
                let value = Value::Integer(n1 * n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Float(n2)) => {
                let value = Value::Float(n1 * n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Integer(n2)) => {
                let value = Value::Float(n1 * (n2 as f64));

                Ok(value)
            },
            (Value::Integer(n1), Value::Float(n2)) => {
                let value = Value::Float((n1 as f64) * n2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_divide(&self, left: Value, right: Value) -> io::Result<Value> {
        match (left, right) {
            (Value::Integer(n1), Value::Integer(n2)) => {
                if n2 == 0 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Division by zero!"))
                }
                
                let value = Value::Float(n1 as f64 / n2 as f64);

                Ok(value)
            },
            (Value::Float(n1), Value::Float(n2)) => {
                if n2 == 0.0 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Division by zero!"))
                }

                let value = Value::Float(n1 / n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Integer(n2)) => {
                if n2 == 0 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Division by zero!"))
                }

                let value = Value::Float(n1 / (n2 as f64));

                Ok(value)
            },
            (Value::Integer(n1), Value::Float(n2)) => {
                if n2 == 0.0 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Division by zero!"))
                }

                let value = Value::Float((n1 as f64) / n2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }
}