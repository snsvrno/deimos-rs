use std::env;
use std::io::{stdin,stdout,Write,prelude::*};
use std::fs::File;

use deimos_core;

pub struct Options {
    pub show_every_result : bool,
    pub interactive_mode : bool,
    pub run_file : Option<String>,
    pub load_file : Vec<String>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            show_every_result : false,
            interactive_mode : false,
            run_file : None,
            load_file : Vec::new(),
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
            "-i" => options.interactive_mode = true,
            "-l" => { options.load_file.push(args[i+1].to_string()); i += 1; }
            "-e" => { options.run_file = Some(args[i+1].to_string()); i += 1; }
            _ => ()
        }
        i += 1;
    }
    
    process_args(&options);
}

fn process_args(options : &Options) {
    for i in 0 .. options.load_file.len() {
        println!("load file: {}",options.load_file[i]);
    }
    
    if options.interactive_mode {
        interactive_mode(options);
        return;
    }

    if let Some(ref file) = options.run_file {
        run_file(&file,options);
        return;
    }
}

fn print_version_string() {
    println!("{}",app_string());
}

fn get_prompt() -> String {
    format!(">")
}

fn run_file(file_path : &str, options : &Options) {
    match File::open(file_path) {
        Err(error) => println!("ERROR : {}",error),
        Ok(mut file) => {
            let mut buffer : String = String::new();
            match file.read_to_string(&mut buffer) {
                Err(error) => println!("ERROR : {}",error),
                Ok(_) => {
                    match deimos_core::run(&buffer) {
                        Err(error) => println!("ERROR : {}",error),
                        Ok(result) => if !result.is_empty() { println!("{}",result); },
                    }
                }
            }
        }
    }
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

fn app_string() -> String {
    format!("Deimos {}, Copyright © 2019 SNSVRNO, released under the MIT LICENSE",env!("CARGO_PKG_VERSION"))
}
