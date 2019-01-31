use std::env;
use std::io::{stdin,stdout,Write};

use deimos_core;

pub struct Options {
    pub show_every_result : bool,
    pub interactive_mode : bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            show_every_result : false,
            interactive_mode : true,
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut options : Options = Options::default();

    // goes into REPL MODE automatically
    if args.len() == 1 {
        options.interactive_mode = true;
    }

    let mut i = 1;
    loop {
        if i >= args.len() { break; }
        match args[i].as_str() {
            "-d" => { options.show_every_result = true; },
            "-v" => { print_version_string(); break; },
            "-i" => options.interactive_mode = true, // option_i(),
            "-e" => { option_e(&args[i+1]); i += 1; }
            "-l" => { option_l(&args[i+1]); i += 1; }
            _ => ()
        }
        i += 1;
    }

    process_args(&options);
}

fn process_args(options : &Options) {
    if options.interactive_mode {
        interactive_mode(options);
    }
}

fn print_version_string() {
    println!("{}",app_string());
}

fn get_prompt() -> String {
    format!(">")
}

fn interactive_mode(options : &Options) {
    print_version_string();
    
    let prompt = get_prompt();
    let mut input = String::new();
    let mut repl = deimos_core::Repl::new();

    loop {
        print!("{}",&prompt);
        let _ = stdout().flush();
        stdin().read_line(&mut input);

        match repl.add(&input) {
            Err(error) => println!("ERROR : {}",error),
            Ok(result) => { 
                if options.show_every_result {
                    if let Some(text) = result.as_user_output() {
                        println!("d: {}",text); 
                    }
                }
            },
        }
        input = String::new();
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
