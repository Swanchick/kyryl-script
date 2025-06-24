use crate::kyryl_script::KyrylScript;
use std::env::args;


mod kyryl_script;
mod lexer;
mod parser;
mod interpreter;
mod native_registry;
mod ks_std;

fn main() {
    let args: Vec<String> = args().collect();
    let path = args.get(1);

    if let Some(path) = path {
        let mut ks = KyrylScript::new();

        let ks_result = ks.run_from_file(path);

        if let Err(e) = ks_result {
            println!("{}", e);
        }
    }
}


#[cfg(test)]
mod tests;

