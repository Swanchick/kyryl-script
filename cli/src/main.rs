use std::env::args;

use core::kyryl_script::KyrylScript;
use ks_std::ks_register_std;

fn main() {
    let args: Vec<String> = args().collect();
    let path = args.get(1);

    if let Some(path) = path {
        ks_register_std();

        let mut ks = KyrylScript::new();
        let ks_result = ks.run_from_file(path);

        if let Err(e) = ks_result {
            println!("{}", e);
        }
    }
}
