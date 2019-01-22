use std::env;
use std::io::{stdin,stdout,Write};

use deimos_core;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // goes into REPL MODE automatically
    if args.len() == 1 {
        option_v();
        option_i();
    }

    let mut i = 1;
    loop {
        if i >= args.len() { break; }
        match args[i].as_str() {
            "-v" => option_v(),
            "-i" => option_i(),
            "-e" => { option_e(&args[i+1]); i += 1; }
            "-l" => { option_l(&args[i+1]); i += 1; }
            _ => ()
        }
        i += 1;
    }
}

fn option_v() {
    println!("{}",app_string());
}

fn get_prompt() -> String {
    format!(">")
}

fn option_i() {
    let prompt = get_prompt();
    let mut input = String::new();

    loop {
        print!("{}",&prompt);
        let _ = stdout().flush();
        stdin().read_line(&mut input);
    }
}

fn option_e(code : &str) {
    println!("run this code: '{}'",code);
}
fn option_l(code : &str) {
    println!("include this library: '{}'",code);
}

fn app_string() -> String {
    format!("Deimos {}, Copyright © 2019 SNSVRNO, released under the MIT LICENSE",env!("CARGO_PKG_VERSION"))
}