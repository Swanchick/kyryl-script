use std::cell::RefCell;
use std::rc::Rc;
use std::io;

use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::interpreter::enviroment::Environment;
use crate::interpreter::interpreter::Interpreter;

use crate::ks_std::register_standart_library;
use crate::parser::statement::Statement;

pub struct KyrylScript {
    global: Rc<RefCell<Environment>>,
    interpreter: Rc<RefCell<Interpreter>>,
}

impl KyrylScript {
    pub fn new() -> KyrylScript {
        let global = Rc::new(RefCell::new(Environment::new()));
        let interpreter = Rc::new(RefCell::new(Interpreter::new(global.clone())));
        
        KyrylScript {
            global,
            interpreter
        }
    }

    pub fn with_global(global: Rc<RefCell<Environment>>) -> KyrylScript {
        let interpreter = Rc::new(RefCell::new(Interpreter::new(global.clone())));
        
        KyrylScript {
            global: global.clone(),
            interpreter
        }
    }

    pub fn run_from_file(&mut self, path: &str) -> io::Result<()> {
        register_standart_library();

        let mut lexer = Lexer::load(path)?;
        lexer.lexer()?;

        let tokens = lexer.get_tokens().clone();
        let token_pos = lexer.get_token_pos().clone();

        let mut parser = Parser::new(tokens, token_pos);
        let block = parser.start();

        if let Err(e) = block {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("KyrylScript Parser Layer: \n{}", e.to_string())
            ));
        }

        let block = block?;
        let mut interpreter = self.interpreter.borrow_mut();

        let interpreter_result = interpreter.interpret_statements(block);

        if let Err(e) = interpreter_result {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("KyrylScript Runtime Layer: \n{}", e.to_string())
            ));
        }

        Ok(())
    }
}
