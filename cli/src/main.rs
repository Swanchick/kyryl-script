use std::env::args;

use core::kyryl_script::KyrylScript;
use ks_std::ks_register_std;
use core::global::ks_path::KsPath;

fn main() {
    let test_path = KsPath::from(".\\examples\\main.ks").unwrap();
    let parent_directory = test_path.parent();

    println!("{:?}", test_path);
    println!("Is file: {}", test_path.is_file());
    println!("{:?}", parent_directory);
    println!("Is dir: {}", parent_directory.is_dir());
    // println!("Parent directory: {:?}", parent_directory.is_dir());
    
    // let args: Vec<String> = args().collect();
    // let path = args.get(1);

    // if let Some(path) = path {
    //     ks_register_std();

    //     let mut ks = KyrylScript::new();
    //     let ks_result = ks.run_from_file(path);

    //     if let Err(e) = ks_result {
    //         println!("{}", e);
    //     }
    // }
}
