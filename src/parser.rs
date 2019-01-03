use enums::token::Token;
use enums::tokentype::TokenType;
use enums::operator::Operator;
use enums::value::Value;
use enums::eresult::EResult;

use structs::branch::Branch;
use structs::tree::Tree;
use structs::env::Env;

use failure::Error;

#[derive(Eq,PartialEq,Debug)]
enum Mode {
    First,
    String,
    Local,
    Comment,
    Parameters,
    None,
}

pub struct Parser<'a> {
    // stuff used for operation
    raw_code : &'a str,
    trees : Vec<Tree>,
    env : Env,

    // stuff use for building
    cursor_pos : usize,
    code_segment_start : usize,
    mode : Mode,
}

impl<'a> Parser<'a> {
    pub fn new(code : &'a str) -> Parser {
        Parser {
            raw_code : code,
            trees : Vec::new(),
            env : { 
                let mut env = Env::new();
                env.load_lua_standard_functions();
                env 
            },

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
                // checks if we are looking at a function
                if self.mode != Mode::Parameters && sending_token.token_type() == TokenType::Word {
                    if let Token::Operator(Operator::OpenParenth) = next_token  {
                        self.cursor_pos += 1;
                        self.mode = Mode::Parameters;
                    } else {
                        break;
                    }
                } else if self.mode == Mode::Parameters { 
                    if let Token::Operator(Operator::CloseParenth) = next_token {
                        self.mode = Mode::None;
                        sending_token = sending_token.to_function();
                        break;
                    }
                } else {
                    break;
                }
            }
            
        }

        self.mode = Mode::None;
        sending_token
    }

    fn as_token(&mut self, char : &str) -> Token {

        // checks if we are starting a comment


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
            "(" => Token::Operator(Operator::OpenParenth),
            ")" => Token::Operator(Operator::CloseParenth),
            "=" => match self.mode {
                Mode::Local => {
                    self.mode = Mode::None;
                    Token::Operator(Operator::Equals(true))
                },
                _ => Token::Operator(Operator::Equals(false)),
            },
            _ => Token::None,
        }
        
    }

    fn build_tree(&mut self) -> Result<(),Error> {
        let mut current_branch : Option<Branch> = None;
        let mut assignment_branch : Option<Branch> = None;
        let mut current_tree : Tree = Tree::new();

        loop {
            let token = self.next_token();
            
            current_branch = match token {
                Token::None => return Err(format_err!("Cannot have a 'Start' token inside a code block.")),
                Token::Tree(_) =>return Err(format_err!("Tree cannot be built!@")),
                Token::WhiteSpace(_) => current_branch,

                Token::Function(_) |
                Token::Int(_) |
                Token::String(_) => Some(Branch::new(token)),
                
                Token::Word(ref word) => {
                    // special words, stuff for the lexicon
                    match word.as_str() {
                        "do" => None,
                        "end" => None,
                        _ => Some(Branch::new(Token::Word(word.to_string()))),
                    }
                },

                Token::EOL | Token::EOF => {
                    if let Some(mut c_branch) = current_branch {
                        if let Some(mut branch) =  assignment_branch {
                            branch.add_child(c_branch);
                            c_branch = branch;
                            assignment_branch = None;
                        }
                        current_tree.add_branch(c_branch);
                    }

                    if Token::EOF == token {
                        current_tree.set_range(self.code_segment_start,self.cursor_pos);
                        self.trees.push(current_tree);
                        break;
                    }

                    None
                }
                Token::Operator(Operator::Equals(_)) => {
                    match current_branch {
                        Some(current_branch) => {
                            let mut branch = Branch::new(token);
                            branch.add_child(current_branch);
                            assignment_branch = Some(branch);
                            Some(Branch::new(Token::None))
                        },
                        None => return Err(format_err!("Cannot assign operator unless there is something to assign.")),
                    }
                },
                Token::Operator(_) => {
                    match current_branch {
                        Some(current_branch) => {
                            let mut branch = Branch::new(token);
                            branch.add_child(current_branch);
                            Some(branch)
                        },
                        None => return Err(format_err!("Cannot assign operator unless there is something to assign.")),
                    }
                }
            };
        }

        Ok(())
    }

    pub fn eval(&mut self) -> Result<Value,Error> {
        self.build_tree()?;

        // TODO : should I do the command queue idea?
        for tree in self.trees.iter_mut() {
            tree.pretty(Some(self.raw_code));

            let (_code_result, action_queue) = tree.eval(&self.env)?;

            // process the action queue, doesn't worry about Values, only Assignments
            for action in action_queue {
                match action {
                    // always local at this level, because this is the top most level.
                    EResult::Assignment(variable_name,value,_) => { self.env.set_var_local(variable_name,value); }, 
                    _ => (),
                }
            }
        }

        println!("ev : {:?}",self.env);

        Ok(Value::Bool(true))
    }

    pub fn value_of(&'a self, variable_name : &str) -> Option<&'a Value> {
        self.env.get_value_of(variable_name)
    }
}