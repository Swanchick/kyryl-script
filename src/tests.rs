use crate::parser::expression::Expression;
use crate::parser::statement::Statement;

use super::*;
use lexer::lexer::Lexer;
use lexer::token::Token;
use parser::parser::Parser;
use parser::function::Function;
use parser::data_type::DataType;
use parser::parameter::Parameter;
use parser::operator::Operator;

#[test]
fn test_lexer() {
    let mut lexer = Lexer::load("test.kys").unwrap();

    let expected_tokens: Vec<Token> = vec![
        Token::Identifier("print".to_string()),
        Token::LeftParenthesis,
        Token::StringLiteral("Hello World".to_string()),
        Token::RightParenthesis,
        Token::Semicolon
    ];

    let line = String::from("print(\"Hello World\");");

    let tokens = lexer.lex_line(&line).unwrap();

    assert_eq!(tokens, expected_tokens);
}


#[test]
fn test_lexer_from_file() {
    let source = concat!(
        "function main() {\n",
        "    let value: float = 10.2f;\n",
        "    let value2: int = 10;\n",
        "    print(\"Hello World\");\n",
        "}\n"
    );

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let expected_tokens: Vec<Token> = vec![
        Token::Keyword("function".to_string()),
        Token::Identifier("main".to_string()),
        Token::LeftParenthesis,
        Token::RightParenthesis,
        Token::LeftBrace,
        Token::Keyword("let".to_string()),
        Token::Identifier("value".to_string()),
        Token::Colon,
        Token::Keyword("float".to_string()),
        Token::Equal,
        Token::FloatLiteral(10.2),
        Token::Semicolon,
        Token::Keyword("let".to_string()),
        Token::Identifier("value2".to_string()),
        Token::Colon,
        Token::Keyword("int".to_string()),
        Token::Equal,
        Token::IntegerLiteral(10),
        Token::Semicolon,
        Token::Identifier("print".to_string()),
        Token::LeftParenthesis,
        Token::StringLiteral("Hello World".to_string()),
        Token::RightParenthesis,
        Token::Semicolon,
        Token::RightBrace
    ];

    let tokens = lexer.get_tokens();

    assert_eq!(tokens, &expected_tokens);
}


#[test]
fn test_lexer_identefier_underscore() {
    let source = concat!(
        "function test_function() {\n",
        "}\n"
    );

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let expected_tokens: Vec<Token> = vec![
        Token::Keyword("function".to_string()),
        Token::Identifier("test_function".to_string()),
        Token::LeftParenthesis,
        Token::RightParenthesis,
        Token::LeftBrace,
        Token::RightBrace
    ];

    let tokens = lexer.get_tokens();

    assert_eq!(tokens, &expected_tokens);
}


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

    for token in lexer.get_tokens().iter() {
        println!("Token: {}", token);
    }

    println!("====================");

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

    let test_statement = Statement::VarableDeclaration {
        name: String::from("a"),
        data_type: Some(DataType::Int),
        value: Some(Expression::IntegerLiteral(10))
    };

    let mut parser = Parser::new(tokens);
    let statement = parser.parse_statement().unwrap();

    assert_eq!(statement, test_statement);
}
