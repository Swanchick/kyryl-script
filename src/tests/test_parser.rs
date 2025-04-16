use std::vec;

use crate::*;
use lexer::lexer::Lexer;
use lexer::token::Token;
use parser::parser::Parser;
use parser::function::Function;
use parser::data_type::DataType;
use parser::parameter::Parameter;
use parser::operator::Operator;
use parser::expression::Expression;
use parser::statement::Statement;

#[test]
fn test_function_parse_name() {
    let source = concat!(
        "function add(a: int, b: int): int {\n",
        "}\n"
    );

    let test_function = Function {
        name: String::from("add"),
        return_type: DataType::Int,
        parameters: vec![
            Parameter {
                name: String::from("a"),
                data_type: DataType::Int
            },
            Parameter {
                name: String::from("b"),
                data_type: DataType::Int
            }
        ],
        body: Vec::new()
    };

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let function = parser.parse_function().unwrap();

    assert_eq!(function.name, test_function.name);
}

#[test]
fn test_parser_parameter() {
    let source = concat!(
        "function add(test: int): int {\n",
        "}\n"
    );

    let test_function = Function {
        name: String::from("add"),
        return_type: DataType::Int,
        parameters: vec![
            Parameter {
                name: String::from("test"),
                data_type: DataType::Int
            },
        ],
        body: Vec::new()
    };

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let function = parser.parse_function().unwrap();

    assert_eq!(function.name, test_function.name);
}

#[test]
fn test_parser_parameters() {
    let source = concat!(
        "function test_function(test1: int, test2: int): int {\n",
        "}\n"
    );
    
    let function_test = Function {
        name: String::from("test_function"),
        return_type: DataType::Int,
        parameters: vec![
            Parameter {
                name: String::from("test1"),
                data_type: DataType::Int
            },
            Parameter {
                name: String::from("test2"),
                data_type: DataType::Int
            }
        ],
        body: Vec::new()
    };

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let function = parser.parse_function().unwrap();

    assert_eq!(function.parameters, function_test.parameters);
}

#[test]
fn test_parser_function_type() {
    let source = concat!(
        "function test_function(test1: float, test2: float): float {\n",
        "}\n"
    );
    
    let function_test = Function {
        name: String::from("test_function"),
        return_type: DataType::Float,
        parameters: vec![
            Parameter {
                name: String::from("test1"),
                data_type: DataType::Int
            },
            Parameter {
                name: String::from("test2"),
                data_type: DataType::Int
            }
        ],
        body: Vec::new()
    };

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let function = parser.parse_function().unwrap();

    assert_eq!(function.return_type, function_test.return_type);
}

#[test]
fn test_parser_function_no_type() {
    let source = concat!(
        "function test_function(test1: float, test2: float) {\n",
        "}\n"
    );
    
    let function_test = Function {
        name: String::from("test_function"),
        return_type: DataType::Void,
        parameters: vec![
            Parameter {
                name: String::from("test1"),
                data_type: DataType::Int
            },
            Parameter {
                name: String::from("test2"),
                data_type: DataType::Int
            }
        ],
        body: Vec::new()
    };

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let function = parser.parse_function().unwrap();

    assert_eq!(function.return_type, function_test.return_type);
}

#[test]
fn test_function_parse() {
    let source = concat!(
        "function test_function(test1: float, test2: float) {\n",
        "}\n"
    );
    
    let function_test = Function {
        name: String::from("test_function"),
        return_type: DataType::Void,
        parameters: vec![
            Parameter {
                name: String::from("test1"),
                data_type: DataType::Float
            },
            Parameter {
                name: String::from("test2"),
                data_type: DataType::Float
            }
        ],
        body: Vec::new()
    };

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let function = parser.parse_function().unwrap();

    assert_eq!(function, function_test);
}


#[test]
fn test_expression() {
    let test_expression = Expression::BinaryOp {
        left: Box::new(
            Expression::BinaryOp {
                left: Box::new(Expression::IntegerLiteral(10)),
                operator: Operator::Plus,
                right: Box::new(Expression::IntegerLiteral(20))
            }
        ),
        operator: Operator::Plus,
        right: Box::new(Expression::IntegerLiteral(30))
    };

    let tokens = vec![
        Token::IntegerLiteral(10),
        Token::Plus,
        Token::IntegerLiteral(20),
        Token::Plus,
        Token::IntegerLiteral(30)
    ];

    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_complex_expression() {
    let test_expression = Expression::BinaryOp {
        left: Box::new(Expression::BinaryOp {
            left: Box::new(Expression::IntegerLiteral(3)),
            operator: Operator::Plus,
            right: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::IntegerLiteral(2)),
                operator: Operator::Multiply,
                right: Box::new(Expression::IntegerLiteral(3))
            })
        }),
        operator: Operator::Minus,
        right: Box::new(Expression::IntegerLiteral(8))
    };

    // 3 + 2 * 3 - 8

    let tokens = vec![
        Token::IntegerLiteral(3),
        Token::Plus,
        Token::IntegerLiteral(2),
        Token::Multiply,
        Token::IntegerLiteral(3),
        Token::Minus,
        Token::IntegerLiteral(8)
    ];

    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_complex_even_more_complex_expression() {
    let test_expression = Expression::BinaryOp {
        left: Box::new(Expression::BinaryOp {
            left: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::IntegerLiteral(2)),
                operator: Operator::Plus,
                right: Box::new(Expression::IntegerLiteral(2))
            }),
            operator: Operator::Divide,
            right: Box::new(Expression::IntegerLiteral(3))
        }),
        operator: Operator::Plus,
        right: Box::new(Expression::BinaryOp {
            left: Box::new(Expression::IntegerLiteral(4)),
            operator: Operator::Multiply,
            right: Box::new(Expression::IntegerLiteral(8))
        })
    };

    // (2 + 2) / 3 + 4 * 8

    let tokens = vec![
        Token::LeftParenthesis,
        Token::IntegerLiteral(2),
        Token::Plus,
        Token::IntegerLiteral(2),
        Token::RightParenthesis,
        Token::Divide,
        Token::IntegerLiteral(3),
        Token::Plus,
        Token::IntegerLiteral(4),
        Token::Multiply,
        Token::IntegerLiteral(8)
    ];

    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    println!("{:?}", expression);

    assert_eq!(expression, test_expression);
}

#[test]
fn test_single_expression() {
    let test_expression = Expression::IntegerLiteral(10);

    let tokens = vec![
        Token::IntegerLiteral(10),
    ];

    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_variable_declaration_statement() {
    // let a: int = 10;

    let tokens = vec![
        Token::Keyword(String::from("let")),
        Token::Identifier(String::from("a")),
        Token::Colon,
        Token::Keyword(String::from("int")),
        Token::Equal,
        Token::IntegerLiteral(10),
        Token::Semicolon
    ];

    let test_statement = Statement::VariableDeclaration {
        name: String::from("a"),
        data_type: Some(DataType::Int),
        value: Some(Expression::IntegerLiteral(10))
    };

    let mut parser = Parser::new(tokens);
    let statement = parser.parse_statement().unwrap();

    assert_eq!(statement, test_statement);
}

#[test]
fn test_expression_boolean_parse() {
    // a == 22 + 33 && b == 23 || c ~= 123

    let test_expression = Expression::BinaryOp {
        left: Box::new(Expression::BinaryOp {
            left: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::Identifier(String::from("a"))),
                operator: Operator::EqualEqual,
                right: Box::new(Expression::BinaryOp {
                    left: Box::new(Expression::IntegerLiteral(22)),
                    operator: Operator::Plus,
                    right: Box::new(Expression::IntegerLiteral(33))
                })
            }),
            operator: Operator::And,
            right: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::Identifier(String::from("b"))),
                operator: Operator::EqualEqual,
                right: Box::new(Expression::IntegerLiteral(23))
            })
        }),
        operator: Operator::Or,
        right: Box::new(Expression::BinaryOp {
            left: Box::new(Expression::Identifier(String::from("c"))),
            operator: Operator::NotEqual,
            right: Box::new(Expression::IntegerLiteral(123))
        })
    };

    let tokens = vec![
        Token::Identifier(String::from("a")),
        Token::EqualEqual,
        Token::IntegerLiteral(22),
        Token::Plus,
        Token::IntegerLiteral(33),
        Token::AmpersandAmpersand,
        Token::Identifier(String::from("b")),
        Token::EqualEqual,
        Token::IntegerLiteral(23),
        Token::PipePipe,
        Token::Identifier(String::from("c")),
        Token::TildeEqual,
        Token::IntegerLiteral(123)
    ];


    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_expression_in_parenthesis() {
    let test_expression = Expression::BinaryOp {
        left: Box::new(Expression::IntegerLiteral(2)),
        operator: Operator::Plus,
        right: Box::new(Expression::IntegerLiteral(2))
    };

    let tokens = vec![
        Token::LeftParenthesis,
        Token::IntegerLiteral(2),
        Token::Plus,
        Token::IntegerLiteral(2),
        Token::RightParenthesis
    ];

    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}


#[test]
fn test_assigment_statement() {
    let source = "a = \"Hello World\";";

    let test_statement = Statement::Assigment {
        name: String::from("a"),
        value: Expression::StringLiteral(String::from("Hello World"))
    };

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let statement = parser.parse_statement().unwrap();


    assert_eq!(statement, test_statement);
}

#[test]
fn test_function_call_statement() {
    let source = "print(add(20, 10), 20);";

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let test_expression = Expression::FunctionCall(String::from("print"), vec![
        Expression::FunctionCall(String::from("add"), vec![Expression::IntegerLiteral(20), Expression::IntegerLiteral(10)]),
        Expression::IntegerLiteral(20)
    ]);

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}


#[test]
fn test_parse_all_functions() {
    let source = concat!(
        "function add(a: int, b: int): int {\n",
        "   return a + b;\n",
        "}\n",
        "function main() {\n",
        "}\n"
    );

    let mut lexer = Lexer::new(String::from(source));
    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let functions = parser.parse_functions().unwrap();

    assert_eq!(functions.len(), 2);
}

#[test]
fn test_parser_front_unary_op() {
    let mut lexer = Lexer::new(String::from("100 - i++"));
    lexer.lexer().unwrap();

    let test_expression = Expression::BinaryOp {
        left: Box::new(Expression::IntegerLiteral(100)),
        operator: Operator::Minus,
        right: Box::new(Expression::FrontUnaryOp {
            expression: Box::new(Expression::Identifier(String::from("i"))),
            operator: Operator::PlusPlus
        })
    };

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_parser_list_expression() {
    let mut lexer = Lexer::new(String::from("[100 + 20, \"Hello\", 230]"));
    lexer.lexer().unwrap();

    let test_expression = Expression::ListLiteral(vec![
        Expression::BinaryOp { left: Box::new(Expression::IntegerLiteral(100)), operator: Operator::Plus, right: Box::new(Expression::IntegerLiteral(20)) },
        Expression::StringLiteral(String::from("Hello")),
        Expression::IntegerLiteral(230)
    ]);

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}


#[test]
fn test_parser_list_index_1() {
    let mut lexer = Lexer::new(String::from("some_list[10]"));
    lexer.lexer().unwrap();

    let test_expression = Expression::IdentifierIndex {
        left: Box::new(Expression::Identifier(String::from("some_list"))),
        index: Box::new(Expression::IntegerLiteral(10))
    };

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_parser_list_index_2() {
    let mut lexer = Lexer::new(String::from("[10, 10, 10, 20, 40, 50, 40][2]"));
    lexer.lexer().unwrap();

    let test_expression = Expression::IdentifierIndex {
        left: Box::new(Expression::ListLiteral(vec![
            Expression::IntegerLiteral(10),
            Expression::IntegerLiteral(10),
            Expression::IntegerLiteral(10),
            Expression::IntegerLiteral(20),
            Expression::IntegerLiteral(40),
            Expression::IntegerLiteral(50),
            Expression::IntegerLiteral(40),
        ])),
        index: Box::new(Expression::IntegerLiteral(2))
    };

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_parser_list_index_3() {
    let mut lexer = Lexer::new(String::from("[[10, 20], [20, 10]][1][0]"));
    lexer.lexer().unwrap();

    let test_expression = Expression::IdentifierIndex {
        left: Box::new(Expression::IdentifierIndex {
            left: Box::new(Expression::ListLiteral(vec![
                Expression::ListLiteral(vec![
                    Expression::IntegerLiteral(10),
                    Expression::IntegerLiteral(20),
                ]),
                Expression::ListLiteral(vec![
                    Expression::IntegerLiteral(20),
                    Expression::IntegerLiteral(10),
                ]),
            ])),
            index: Box::new(Expression::IntegerLiteral(1))
        }),
        index: Box::new(Expression::IntegerLiteral(0))
    };

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_parser_string_index_1() {
    let mut lexer = Lexer::new(String::from("\"Hello worlda asdasd asd\"[10]"));
    lexer.lexer().unwrap();

    let test_expression = Expression::IdentifierIndex {
        left: Box::new(Expression::StringLiteral(String::from("Hello worlda asdasd asd"))),
        index: Box::new(Expression::IntegerLiteral(10))
    };

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_parser_index_assingment_statment() {
    let mut lexer = Lexer::new(String::from("some_list[10][20] = 20;"));
    lexer.lexer().unwrap();

    let test_statement = Statement::AssigmentIndex { 
        name: String::from("some_list"), 
        index: vec![Expression::IntegerLiteral(10)],
        value: Expression::IntegerLiteral(20)
    };

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let statement = parser.parse_statement().unwrap();

    assert_eq!(statement, test_statement);
}

