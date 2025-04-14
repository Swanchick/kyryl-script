use std::rc::Rc;
use std::cell::RefCell;
use std::io;

use crate::parser::data_type::DataType;
use crate::parser::expression::Expression;
use crate::parser::function::Function;
use crate::parser::operator::Operator;
use crate::parser::statement::Statement;

use super::enviroment::Environment;
use super::value::Value;

pub struct Interpreter {
    global: Rc<RefCell<Environment>>,
    local: Rc<RefCell<Environment>>
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let global = Rc::new(RefCell::new(Environment::new()));

        Interpreter {
            global: global.clone(),
            local: global
        }
    }

    pub fn get_local(&self) -> Rc<RefCell<Environment>> {
        self.local.clone()
    }

    pub fn interpret_program(&mut self, functions: Vec<Function>) -> io::Result<()> {
        for function in &functions {
            self.global.borrow_mut().define_variable(
                function.name.clone(),
                Value::Function(Rc::new(RefCell::new(function.clone())))
            );
        }
        
        let main = self.global.borrow().get_variable("main");

        match main {
            Ok(Value::Function(_)) => self.call_function("main", Vec::new()),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Entry point not found!"))
        }?;

        Ok(())
    }

    pub fn register_rust_function(&mut self, name: &str, function: fn(args: Vec<Value>) -> io::Result<Value>) {
        let value = Value::RustFunction(function);

        self.global.borrow_mut().define_variable(name.to_string(), value);
    }

    fn call_function(&mut self, name: &str, args: Vec<Value>) -> io::Result<Value> {
        let function_value = self.global.borrow().get_variable(name)?;

        if let Value::RustFunction(function) = function_value {
            let return_value = function(args)?;

            return Ok(return_value);
        }

        let mut return_value = Value::Void;

        if let Value::Function(function) = function_value {
            let previous_enviroment = self.local.clone();
            self.local = Rc::new(RefCell::new(Environment::with_parent(self.global.clone())));

            if function.borrow().parameters.len() != args.len() {
                return Err(
                    io::Error::new(
                        io::ErrorKind::InvalidInput, 
                        format!("Expected {} amount of parameters, got {}", function.borrow().parameters.len(), args.len()))
                );
            }

            for (parameter, arg) in function.borrow().parameters.iter().zip(args) {
                let arg_type = arg.get_data_type();
                
                if arg_type != parameter.data_type {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Expected argument type {}, instead got {}", parameter.data_type, arg_type)
                    ));
                }
                
                self.local.borrow_mut().define_variable(parameter.name.clone(), arg);
            }

            for statement in function.borrow().body.iter() {
                match self.interpret_statement(statement.clone()) {
                    Ok(Some(value)) => {
                        return_value = value;

                        break;
                    },
                    Err(e) => return Err(e),
                    _ => {}
                } 
            }

            let return_data_type = return_value.get_data_type();
            let expected_data_type = function.borrow().return_type.clone();

            if return_data_type != expected_data_type {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Expected to return {}, but instead got {}", return_data_type, expected_data_type)
                ));
            }

            self.local = previous_enviroment.clone();
        }
        
        Ok(return_value)
    }

    pub fn interpret_statement(&mut self, statement: Statement) -> io::Result<Option<Value>> {
        match statement {
            Statement::VariableDeclaration { name, data_type, value } => {
                let value = if let Some(expression) = value {
                    self.interpret_expression(expression)?
                } else {
                    Value::Void
                };

                if let Some(data_type) = data_type {
                    if value.get_data_type() != data_type {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "Variable declaration type mismatch!"))
                    }
                }

                self.local.borrow_mut().define_variable(name, value);

                Ok(None)
            },
            Statement::Assigment { name, value } => {
                let value = self.interpret_expression(value)?;

                self.local.borrow_mut().assign_variable(&name, value)?;

                Ok(None)
            },
            Statement::AssigmentIndex { name, index, value } => {
                todo!()
            }
            Statement::AddValue { name, value } => {
                let value = self.interpret_expression(value)?;
                self.interpret_add_equal(&name, value)?;

                Ok(None)
            },
            Statement::RemoveValue { name, value } => {
                let value = self.interpret_expression(value)?;
                self.interpret_minus_equal(&name, value)?;

                Ok(None)
            },
            Statement::ReturnStatement { value } => {
                if let Some(expression) = value {
                    Ok(Some(self.interpret_expression(expression)?))
                } else {
                    Ok(Some(Value::Void))
                }
            },
            Statement::IfStatement { condition, body, else_body } => {
                let value = self.interpret_expression(condition)?;

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
                let value = self.interpret_expression(condition.clone())?;

                if let Value::Boolean(boolean) = value {
                    let mut boolean = boolean;
                    
                    while boolean {
                        let return_value = self.interpret_block(body.clone())?;
                        if let Some(return_value) = return_value {
                            return Ok(Some(return_value));
                        }

                        let value = self.interpret_expression(condition.clone())?;
                        if let Value::Boolean(new_boolean) = value {
                            boolean = new_boolean;
                        }
                    }
                }

                Ok(None)
            },
            Statement::Expression { value } => {
                self.interpret_expression(value)?;

                Ok(None)
            }
        } 
    }

    fn interpret_add_equal(&mut self, name: &str, value: Value) -> io::Result<()> {
        let original_value = self.local.borrow().get_variable(&name)?;

        match (original_value, value) {
            (Value::Integer(n1), Value::Integer(n2)) => {
                let value = Value::Integer(n1 + n2);

                self.local.borrow_mut().assign_variable(name, value)?;
            },
            (Value::Float(n1), Value::Integer(n2)) => {
                let value = Value::Float(n1 + (n2 as f64));

                self.local.borrow_mut().assign_variable(name, value)?;
            },
            (Value::Integer(n1), Value::Float(n2)) => {
                let value = Value::Float((n1 as f64) + n2);

                self.local.borrow_mut().assign_variable(name, value)?;
            },
            (Value::Float(n1), Value::Float(n2)) => {
                let value = Value::Float(n1 + n2);

                self.local.borrow_mut().assign_variable(name, value)?;
            },
            (Value::String(mut str1), Value::String(str2)) => {
                str1.push_str(&str2);
                let value = Value::String(str1);

                self.local.borrow_mut().assign_variable(name, value)?;
            },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }

        Ok(())
    }

    fn interpret_minus_equal(&mut self, name: &str, value: Value) -> io::Result<()> {
        let original_value = self.local.borrow().get_variable(&name)?;

        match (original_value, value) {
            (Value::Integer(n1), Value::Integer(n2)) => {
                let value = Value::Integer(n1 - n2);

                self.local.borrow_mut().assign_variable(name, value)?;
            },
            (Value::Float(n1), Value::Integer(n2)) => {
                let value = Value::Float(n1 - (n2 as f64));

                self.local.borrow_mut().assign_variable(name, value)?;
            },
            (Value::Integer(n1), Value::Float(n2)) => {
                let value = Value::Float((n1 as f64) - n2);

                self.local.borrow_mut().assign_variable(name, value)?;
            },
            (Value::Float(n1), Value::Float(n2)) => {
                let value = Value::Float(n1 - n2);

                self.local.borrow_mut().assign_variable(name, value)?;
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

    pub fn interpret_expression(&mut self, expression: Expression) -> io::Result<Value> {
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

                self.call_function(&name, args)
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
            }
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
                self.local.borrow().get_variable(&name)
            }
        }
    }

    fn interpret_identifier_index(&self, left: Value, index: Value) -> io::Result<Value> {
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
                        Err(io::Error::new(io::ErrorKind::InvalidData, "Out of bounds in string."))
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
        let value = self.local.borrow().get_variable(name)?;

        match value {
            Value::Integer(number) => {
                let value = Value::Integer(number + 1);

                self.local.borrow_mut().assign_variable(name, value.clone())?;

                Ok(value)
            }
            Value::Float(number) => {
                let value = Value::Float(number + 1.0);
                self.local.borrow_mut().assign_variable(name, value.clone())?;

                Ok(value)
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Type \"{}\" is not supported by this operator!", value.get_data_type())
            ))
        }
    }

    fn interpret_minus_minus(&mut self, name: &str) -> io::Result<Value> {
        let value = self.local.borrow().get_variable(name)?;

        match value {
            Value::Integer(number) => {
                let value = Value::Integer(number - 1);

                self.local.borrow_mut().assign_variable(name, value.clone())?;

                Ok(value)
            }
            Value::Float(number) => {
                let value = Value::Float(number - 1.0);
                
                self.local.borrow_mut().assign_variable(name, value.clone())?;

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
