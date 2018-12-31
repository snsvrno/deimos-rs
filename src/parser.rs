use enums::token::Token;
use enums::operator::Operator;

use structs::branch::Branch;

use failure::Error;

pub struct Parser<'a> {
    raw_code : &'a str,
    cursor_pos : usize,
    tree : Option<Branch>,
}

impl<'a> Parser<'a> {
    pub fn new(code : &'a str) -> Parser {
        Parser {
            raw_code : code,
            cursor_pos : 0,
            tree : None,
        }
    }

    pub fn next_token(&mut self) -> Token {

        // at the end of the file / code string
        if self.cursor_pos == self.raw_code.len() {
            return Token::EOF;
        }

        // gets the slice of the next char
        let char = &self.raw_code[self.cursor_pos .. self.cursor_pos + 1];
        self.cursor_pos += 1;

        // checks if its an int
        if let Ok(int) = char.parse::<i32>() {
            return Token::Int(int);
        }

        // checks if its an operator
        match char {
            "+" => Token::Operator(Operator::Plus),
            "-" => Token::Operator(Operator::Minus),
            _ => Token::None,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens : Vec<Token> = Vec::new();
        let mut tokens_combined : Vec<Token> = Vec::new();
        
        loop {
            let token = self.next_token();
            if let Token::EOF = token { break; }
            else { tokens.push(token); }
        }


        tokens.reverse();
        let mut last_token : Option<Token> = None;
        loop {
            match tokens.pop() {
                None => {
                    if let Some(last) = last_token {
                        tokens_combined.push(last);
                    }
                    break;
                },
                Some(token) => {
                    if last_token.is_none() {
                        last_token = Some(token);
                    } else {
                        let mut combined = false;
                        if let Some(ref mut last) = last_token {
                            if Token::can_combine(last,&token) {
                                let _result = last.combine_into(token.clone());
                                combined = true;
                            } 
                        }
                        if !combined {
                            if let Some(ref last) = last_token {
                                tokens_combined.push(last.clone());
                            }
                            last_token = Some(token);
                        }
                    }
                }
            }
        }

        println!("{:?}",tokens_combined);

        tokens_combined
    }

    pub fn build_tree(&mut self) -> Result<(),Error> {
        let mut current_branch : Branch = Branch::new(Token::None);
        for token in self.tokenize() {
            match token {
                Token::EOF => {
                    
                },
                Token::EOL => {
                    
                },
                Token::Int(int) => {
                    let branch = Branch::new(Token::Int(int));

                    if !current_branch.is_none() {
                        current_branch.add_child(branch);
                    } else {
                        current_branch = branch;
                    }
                },
                Token::Operator(op) => {
                    let mut branch = Branch::new(Token::Operator(op));
                    branch.add_child(current_branch);
                    current_branch = branch;
                },
                Token::None => {
                    return Err(format_err!("Cannot have a 'Start' token inside a code block."));
                }
            }
        }

        self.tree = Some(current_branch);

        Ok(())
    }

    pub fn eval(&mut self) -> Result<i32,Error> {

        if let Some(ref tree) = self.tree {
            Ok(tree.eval()?.clone())
        } else {
            Err(format_err!("Tree has not been build."))
        }
    }
}