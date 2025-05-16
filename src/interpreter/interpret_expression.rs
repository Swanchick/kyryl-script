use std::io;

use crate::parser::expression::Expression;
use crate::parser::operator::Operator;
use crate::parser::data_type::DataType;

use super::interpreter::Interpreter;
use super::value::{Value, ValueType};

pub struct InterpretExpression<'a> {
    interpreter: &'a mut Interpreter
}


impl<'a> InterpretExpression<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> InterpretExpression<'a> {
        InterpretExpression { interpreter: interpreter }
    }

    pub fn interpret_expression(&mut self, expression: Expression) -> io::Result<Value> {        
        match expression {
            Expression::BinaryOp { left, operator, right } => {
                let left_value = self.interpret_expression(*left)?;
                let left_value = left_value.get_type().clone();
                let right_value  = self.interpret_expression(*right)?;
                let right_value = right_value.get_type().clone();

                let value_type = self.interpret_binary_operation(left_value, right_value, operator)?;
                let value = Value::new(None, value_type);

                Ok(value)
            },
            Expression::UnaryOp { expression, operator } => {
                let value = self.interpret_expression(*expression)?;
                let value_type = value.get_type();

                let value_type = self.interpret_unary_operation(value_type.clone(), operator)?;
                let value = Value::new(None, value_type);

                Ok(value)
            },
            Expression::FrontUnaryOp { expression, operator } => {
                if let Expression::Identifier(name) = *expression {
                    self.interpret_front_unary_operation(&name, operator)?;

                    Ok(Value::new(None, ValueType::Void))
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
                    let value_type = value.get_type();

                    if let Some(t) = &data_type {
                        if &value.get_type().get_data_type() != t {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, "List has different values. List should consist only of one type!"));
                        }
                    } else {
                        data_type = Some(value_type.get_data_type().clone())
                    }

                    values.push(value.clone());
                }

                Ok(Value::new(None, ValueType::List(values)))
            },
            Expression::IdentifierIndex{ left, index } => {
                let left = self.interpret_expression(*left)?;
                let index = self.interpret_expression(*index)?;

                let value_type = self.interpret_identifier_index(left.get_type().clone(), index.get_type().clone())?;
                let value = Value::new(None, value_type);
                Ok(value)
            },
            Expression::IntegerLiteral(value) => {
                let value = Value::new(None, ValueType::Integer(value));

                Ok(value)
            },
            Expression::FloatLiteral(value) => {
                let value = Value::new(None, ValueType::Float(value));

                Ok(value)
            },
            Expression::BooleanLiteral(value) => {
                let value = Value::new(None, ValueType::Boolean(value));

                Ok(value)
            },
            Expression::StringLiteral(value) => {
                let value = Value::new(None, ValueType::String(value));
                
                Ok(value)
            },
            Expression::Identifier(name) => {
                self.interpreter.get_variable(name.as_str())
            }
        }
    }    

    pub fn interpret_identifier_index(&self, left: ValueType, index: ValueType) -> io::Result<ValueType> {
        if let ValueType::Integer(index) = index {
            match left {
                ValueType::String(str) => {
                    let character = str.chars().nth(index as usize);
                    if let Some(character) = character {
                        let value = ValueType::String(character.to_string());
    
                        Ok(value)
                    } else {
                        Err(io::Error::new(io::ErrorKind::InvalidData, "Out of bounds in string."))
                    }
                },
    
                ValueType::List(values) => {
                    let child_value = values.iter().nth(index as usize);
                    if let Some(child_value) = child_value {
                        Ok(child_value.get_type().clone())
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

    fn interpret_front_unary_operation(&mut self, name: &str, operator: Operator) -> io::Result<ValueType> {
        match operator {
            Operator::PlusPlus => self.interpret_plus_plus(name),
            Operator::MinusMinus => self.interpret_minus_minus(name),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Front Unary Operation Error"))
        }
    }

    fn interpret_plus_plus(&mut self, name: &str) -> io::Result<ValueType> {
        let value_type = {
            let value = self.interpreter.get_variable(name)?;
            let value_type = value.get_type();

            let new_value_type = match value_type {
                ValueType::Integer(number) => ValueType::Integer(number + 1),
                ValueType::Float(number) => ValueType::Float(number + 1.0),
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Type \"{}\" is not supported by this operator!", value_type.get_data_type()),
                    ));
                }
            };

            let new_value = Value::new(value.get_reference(), new_value_type.clone());
            self.interpreter.assign_variable(name, new_value)?;

            new_value_type
        };

        Ok(value_type)
    }

    fn interpret_minus_minus(&mut self, name: &str) -> io::Result<ValueType> {
        let value_type = {
            let value = self.interpreter.get_variable(name)?;
            let value_type = value.get_type();

            let new_value_type = match value_type {
                ValueType::Integer(number) => ValueType::Integer(number - 1),
                ValueType::Float(number) => ValueType::Float(number - 1.0),
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Type \"{}\" is not supported by this operator!", value_type.get_data_type()),
                    ));
                }
            };

            let new_value = Value::new(value.get_reference(), new_value_type.clone());
            self.interpreter.assign_variable(name, new_value)?;

            new_value_type
        };

        Ok(value_type)
    }

    fn interpret_binary_operation(&self, left: ValueType, right: ValueType, operator: Operator) -> io::Result<ValueType> {
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

    fn interpret_equal_equal(&self, left: ValueType, right: ValueType) -> io::Result<ValueType> {
        match (left, right) {
            (ValueType::Integer(n1), ValueType::Integer(n2)) => {
                let value = ValueType::Boolean(n1 == n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Float(n2)) => {
                let value = ValueType::Boolean(n1 == n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Integer(n2)) => {
                let value = ValueType::Boolean(n1 == (n2 as f64));

                Ok(value)
            },
            (ValueType::Integer(n1), ValueType::Float(n2)) => {
                let value = ValueType::Boolean((n1 as f64) == n2);

                Ok(value)
            },
            (ValueType::String(str1), ValueType::String(str2)) => {
                let value = ValueType::Boolean(str1 == str2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_greater_equal(&self, left: ValueType, right: ValueType) -> io::Result<ValueType> {
        match (left, right) {
            (ValueType::Integer(n1), ValueType::Integer(n2)) => {
                let value = ValueType::Boolean(n1 >= n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Float(n2)) => {
                let value = ValueType::Boolean(n1 >= n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Integer(n2)) => {
                let value = ValueType::Boolean(n1 >= (n2 as f64));

                Ok(value)
            },
            (ValueType::Integer(n1), ValueType::Float(n2)) => {
                let value = ValueType::Boolean((n1 as f64) >= n2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_greater(&self, left: ValueType, right: ValueType) -> io::Result<ValueType> {
        match (left, right) {
            (ValueType::Integer(n1), ValueType::Integer(n2)) => {
                let value = ValueType::Boolean(n1 > n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Float(n2)) => {
                let value = ValueType::Boolean(n1 > n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Integer(n2)) => {
                let value = ValueType::Boolean(n1 > (n2 as f64));

                Ok(value)
            },
            (ValueType::Integer(n1), ValueType::Float(n2)) => {
                let value = ValueType::Boolean((n1 as f64) > n2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_less_equal(&self, left: ValueType, right: ValueType) -> io::Result<ValueType> {
        match (left, right) {
            (ValueType::Integer(n1), ValueType::Integer(n2)) => {
                let value = ValueType::Boolean(n1 <= n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Float(n2)) => {
                let value = ValueType::Boolean(n1 <= n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Integer(n2)) => {
                let value = ValueType::Boolean(n1 <= (n2 as f64));

                Ok(value)
            },
            (ValueType::Integer(n1), ValueType::Float(n2)) => {
                let value = ValueType::Boolean((n1 as f64) <= n2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_less(&self, left: ValueType, right: ValueType) -> io::Result<ValueType> {
        match (left, right) {
            (ValueType::Integer(n1), ValueType::Integer(n2)) => {
                let value = ValueType::Boolean(n1 < n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Float(n2)) => {
                let value = ValueType::Boolean(n1 < n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Integer(n2)) => {
                let value = ValueType::Boolean(n1 < (n2 as f64));

                Ok(value)
            },
            (ValueType::Integer(n1), ValueType::Float(n2)) => {
                let value = ValueType::Boolean((n1 as f64) < n2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_tilde_equal(&self, left: ValueType, right: ValueType) -> io::Result<ValueType> {
        match (left, right) {
            (ValueType::Integer(n1), ValueType::Integer(n2)) => {
                let value = ValueType::Boolean(n1 != n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Float(n2)) => {
                let value = ValueType::Boolean(n1 != n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Integer(n2)) => {
                let value = ValueType::Boolean(n1 != (n2 as f64));

                Ok(value)
            },
            (ValueType::Integer(n1), ValueType::Float(n2)) => {
                let value = ValueType::Boolean((n1 as f64) != n2);

                Ok(value)
            },
            (ValueType::String(str1), ValueType::String(str2)) => {
                let value = ValueType::Boolean(str1 != str2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_and(&self, left: ValueType, right: ValueType) -> io::Result<ValueType> {
        match (left, right) {
            (ValueType::Boolean(bool1), ValueType::Boolean(bool2)) => {
                let value = ValueType::Boolean(bool1 && bool2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_or(&self, left: ValueType, right: ValueType) -> io::Result<ValueType> {
        match (left, right) {
            (ValueType::Boolean(bool1), ValueType::Boolean(bool2)) => {
                let value = ValueType::Boolean(bool1 || bool2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_unary_operation(&self, value: ValueType, operator: Operator) -> io::Result<ValueType> {
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

    fn interpret_not(&self, value: ValueType) -> io::Result<ValueType> {
        match value {
            ValueType::Boolean(value) => Ok(ValueType::Boolean(!value)),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Wrong type for not inverting"))
        }
    }

    fn interpret_negation(&self, value: ValueType) -> io::Result<ValueType> {
        match value {
            ValueType::Integer(value) => Ok(ValueType::Integer(-value)),
            ValueType::Float(value) => Ok(ValueType::Float(-value)),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Wrong type for not negation"))
        }
    }

    fn interpret_plus(&self, left: ValueType, right: ValueType) -> io::Result<ValueType> {
        match (left, right) {
            (ValueType::Integer(n1), ValueType::Integer(n2)) => {
                let value = ValueType::Integer(n1 + n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Float(n2)) => {
                let value = ValueType::Float(n1 + n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Integer(n2)) => {
                let value = ValueType::Float(n1 + (n2 as f64));

                Ok(value)
            },
            (ValueType::Integer(n1), ValueType::Float(n2)) => {
                let value = ValueType::Float((n1 as f64) + n2);

                Ok(value)
            },
            (ValueType::String(mut str1), ValueType::String(str2)) => {
                str1.push_str(&str2);
                let value = ValueType::String(str1);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    } 

    fn interpret_minus(&self, left: ValueType, right: ValueType) -> io::Result<ValueType> {
        match (left, right) {
            (ValueType::Integer(n1), ValueType::Integer(n2)) => {
                let value = ValueType::Integer(n1 - n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Float(n2)) => {
                let value = ValueType::Float(n1 - n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Integer(n2)) => {
                let value = ValueType::Float(n1 - (n2 as f64));

                Ok(value)
            },
            (ValueType::Integer(n1), ValueType::Float(n2)) => {
                let value = ValueType::Float((n1 as f64) - n2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_multiply(&self, left: ValueType, right: ValueType) -> io::Result<ValueType> {
        match (left, right) {
            (ValueType::Integer(n1), ValueType::Integer(n2)) => {
                let value = ValueType::Integer(n1 * n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Float(n2)) => {
                let value = ValueType::Float(n1 * n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Integer(n2)) => {
                let value = ValueType::Float(n1 * (n2 as f64));

                Ok(value)
            },
            (ValueType::Integer(n1), ValueType::Float(n2)) => {
                let value = ValueType::Float((n1 as f64) * n2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }

    fn interpret_divide(&self, left: ValueType, right: ValueType) -> io::Result<ValueType> {
        match (left, right) {
            (ValueType::Integer(n1), ValueType::Integer(n2)) => {
                if n2 == 0 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Division by zero!"))
                }
                
                let value = ValueType::Float(n1 as f64 / n2 as f64);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Float(n2)) => {
                if n2 == 0.0 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Division by zero!"))
                }

                let value = ValueType::Float(n1 / n2);

                Ok(value)
            },
            (ValueType::Float(n1), ValueType::Integer(n2)) => {
                if n2 == 0 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Division by zero!"))
                }

                let value = ValueType::Float(n1 / (n2 as f64));

                Ok(value)
            },
            (ValueType::Integer(n1), ValueType::Float(n2)) => {
                if n2 == 0.0 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Division by zero!"))
                }

                let value = ValueType::Float((n1 as f64) / n2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    }
}