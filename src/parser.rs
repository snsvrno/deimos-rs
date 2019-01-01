use enums::token::Token;
use enums::operator::Operator;
use enums::value::Value;
use enums::eresult::EResult;

use std::collections::HashMap;

use structs::branch::Branch;
use structs::tree::Tree;
use structs::env::Env;

use failure::Error;

#[derive(Eq,PartialEq)]
enum Mode {
    First,
    String,
    None,
}

pub struct Parser<'a> {
    // stuff used for operation
    raw_code : &'a str,
    tree : Vec<Tree>,
    variables : HashMap<String,Value>,
    // stuff use for building
    cursor_pos : usize,
    code_segment_start : usize,
    mode : Mode,
}

impl<'a> Parser<'a> {
    pub fn new(code : &'a str) -> Parser {
        Parser {
            raw_code : code,
            tree : Vec::new(),
            variables : HashMap::new(),

            cursor_pos : 0,
            code_segment_start : 0,
            mode : Mode::None,
        }
    }

    fn next_token(&mut self) -> Token {

        // at the end of the file / code string
        if self.cursor_pos == self.raw_code.len() {
            return Token::EOF;
        }

        // gets the slice of the next char
        let char = &self.raw_code[self.cursor_pos .. self.cursor_pos + 1];
        self.cursor_pos +=1;

        let mut sending_token = self.as_token(char);

        loop {
            if self.cursor_pos == self.raw_code.len() {
                return Token::EOF;
            }

            let char = &self.raw_code[self.cursor_pos .. self.cursor_pos + 1];
            let next_token = self.as_token(char);
            if Token::can_combine(&sending_token,&next_token) {
                sending_token.combine_into(next_token);
                self.cursor_pos += 1;
            } else {
                break;
            }
            
        }
        
        self.mode = Mode::None;
        sending_token
    }

    fn as_token(&mut self, char : &str) -> Token {

        // checks if its an int
        if let Ok(int) = char.parse::<i32>() {
            self.mode = Mode::None;
            return Token::Int(int);
        } 

        // checks if its a word
        if Token::valid_word_char(char,self.mode == Mode::None) {
            self.mode = Mode::None;
            return Token::Word(char.to_string());
        }

        // checks if its an operator
        match char {
            " " => Token::WhiteSpace(1),
            ";" | "\n" => Token::EOL,
            "+" => Token::Operator(Operator::Plus),
            "-" => Token::Operator(Operator::Minus),
            "=" => Token::Operator(Operator::Equals),
            _ => Token::None,
        }
        
    }

    fn end_of_line(&mut self,current_branch : Branch, assignment_branch : Option<Branch>) {
        let mut new_tree = Tree::new();
        new_tree.set_range(self.code_segment_start,self.cursor_pos);

        match assignment_branch {
            Some(mut branch) => {
                branch.add_child(current_branch);
                new_tree.add_branch(branch);
                self.tree.push(new_tree);
            },
            None => {
                match current_branch.is_none() {
                    true => (),
                    false => {
                        new_tree.add_branch(current_branch);  
                        self.tree.push(new_tree)
                    },
                }
            }
        }
    }

    fn build_tree(&mut self) -> Result<(),Error> {
        let mut current_branch : Branch = Branch::new(Token::None);
        let mut assignment_branch : Option<Branch> = None;

        loop {
            let token = self.next_token();
            println!("token: {:?}",token);
            match token {
                Token::EOF => break,
                Token::WhiteSpace(_) => {
                    
                },
                Token::Word(word) => {
                    let branch = Branch::new(Token::Word(word));

                    if !current_branch.is_none() {
                        current_branch.add_child(branch);
                    } else {
                        current_branch = branch;
                    }
                },
                Token::String(string) => {
                    let branch = Branch::new(Token::String(string));

                    if !current_branch.is_none() {
                        current_branch.add_child(branch);
                    } else {
                        current_branch = branch;
                    }
                },
                Token::EOL => {
                        self.end_of_line(current_branch, assignment_branch);
                        assignment_branch = None;
                        current_branch = Branch::new(Token::None);

                }
                Token::Int(int) => {
                    let branch = Branch::new(Token::Int(int));

                    if !current_branch.is_none() {
                        current_branch.add_child(branch);
                    } else {
                        current_branch = branch;
                    }
                },
                Token::Operator(op) => {
                    let mut branch = Branch::new(Token::Operator(op.clone()));
                    branch.add_child(current_branch);

                    if op == Operator::Equals {
                        assignment_branch = Some(branch);
                        current_branch = Branch::new(Token::None);
                    } else {
                        current_branch = branch;
                    }
                },
                Token::None => {
                    return Err(format_err!("Cannot have a 'Start' token inside a code block."));
                }
            }
        }

        self.end_of_line(current_branch,assignment_branch);

        Ok(())
    }

    pub fn eval(&mut self) -> Result<Value,Error> {
        self.build_tree()?;

        for branch in self.tree.iter() {
            //println!("==");
            //branch.pretty(None);
            let mut env = Env::from(&mut self.variables);
            match branch.eval(&mut env)? {
                EResult::Assignment(variable_name,value) => { self.variables.insert(variable_name, value); }, 
                _ => (),
            }
        }

        Ok(Value::Bool(true))
    }

    pub fn value_of(&'a self, variable_name : &str) -> Option<&'a Value> {
        self.variables.get(variable_name)
    }
}