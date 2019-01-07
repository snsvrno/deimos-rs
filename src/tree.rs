use crate::scanner::Scanner;
use failure::Error;

use crate::token::Token;
use crate::grammar::gram::Gram;
use crate::tokentype::TokenType;

use crate::grammar::binary::Binary;
use crate::grammar::unary::Unary;
use crate::grammar::expression::Expression;
use crate::grammar::grouping::Grouping;

pub struct Tree<'a> {
    raw_code : &'a str,
    tokens : Vec<Vec<Gram>>,
}

impl<'a> Tree<'a> {
    pub fn from_scanner(scanner : Scanner<'a>) -> Result<Tree,Error> {
        let (raw_code,mut raw_tokens) = scanner.explode();

        let mut tokens : Vec<Vec<Gram>> = Vec::new();
        let mut sub_tokens : Vec<Gram> = Vec::new();

        // loops through the linear list of tokens and groups them into statements.
        loop {
            let token : Option<Token> = if raw_tokens.len() > 0 { Some(raw_tokens.remove(0)) } else { None }; 

            match token {
                None => break,
                Some(token) => {
                    match token.get_type() {
                        TokenType::EOL => {
                            if sub_tokens.len() > 0 { 
                                tokens.push(sub_tokens);
                                sub_tokens = Vec::new();
                            }
                        },
                        _ => {
                            let gram = Gram::create(token);
                            match Expression::create_into_gram(&gram) {
                                None => sub_tokens.push(gram),
                                Some(expr) => sub_tokens.push(expr),
                            }
                            
                        }
                    }
                }
            }

        }

        // catches the last tokens if there are any.
        if sub_tokens.len() > 0 {
            tokens.push(sub_tokens);
        }

        for t in tokens.iter() {
            for t2 in t.iter() {
                println!("");
                println!("T : {:?}",t2);
            }
        }

        let tree = Tree {
            raw_code : raw_code,
            tokens : tokens,
        };

        Ok(tree)
    }

    pub fn create_tree(mut self) -> Result<Self,Error> {

        for mut line in self.tokens.iter_mut() {
            let mut tier = 0;
            loop {
                // we'll check for grouping every iteration
                // new groupings might be generated every time we do something.
                
                if tier == 3 {
                    // '-' and 'Not' should be done after checking for '-' and '+' binaries, 
                    // and then '-', '+', '*', and '/' should be done again. 
                    Unary::process_set(&mut line)?;
                    Binary::process_set_increment(&mut line,1)?;
                    Binary::process_set_increment(&mut line,2)?;
                }
                let new_tier = Binary::process_set_increment(&mut line,tier)?;
                match new_tier {
                    None => break,
                    Some(t) => tier = t,
                }
            }
        }

        for line in self.tokens.iter() {
            println!("====");
            for token in line.iter() {
                println!("{}",token);
            }
        }

        Ok(self)
    }
}

mod tests {
    #[test]
    fn simple_binary_code() {
        use crate::scanner::Scanner;
        use crate::tree::Tree;
        use crate::tokentype::TokenType;

        
        let scanner = Scanner::new("bob = 5 + 2").scan().unwrap();
        let tree = Tree::from_scanner(scanner).unwrap().create_tree().unwrap();

        let test_against = create_expression!(create_binary!(
            create_token!(TokenType::Equal),
            create_expression!(create_literal!(TokenType::Identifier("bob".to_string()))),
            create_expression!(create_binary!(
                create_token!(TokenType::Plus),
                create_expression!(create_literal!(TokenType::Number(5.0))),
                create_expression!(create_literal!(TokenType::Number(2.0)))
            ))
        ));

        assert_eq!(tree.tokens.len(),1);
        assert_eq!(tree.tokens[0].len(),1);
        
        if tree.tokens[0][0] != test_against {
            panic!("Equality check failed.\n\n  Left: {}\n  Right: {}\n\n",
                tree.tokens[0][0],
                test_against
            );
        }
    }

    #[test]
    fn simple_binary_unary_code() {
        use crate::scanner::Scanner;
        use crate::tree::Tree;
        use crate::tokentype::TokenType;
        
        let scanner = Scanner::new("bob = 5 + -2").scan().unwrap();
        let tree = Tree::from_scanner(scanner).unwrap().create_tree().unwrap();

        let test_against = create_expression!(create_binary!(
            create_token!(TokenType::Equal),
            create_expression!(create_literal!(TokenType::Identifier("bob".to_string()))),
            create_expression!(create_binary!(
                create_token!(TokenType::Plus),
                create_expression!(create_literal!(TokenType::Number(5.0))),
                create_expression!(create_unary!(
                    create_token!(TokenType::Minus),
                    create_expression!(create_literal!(TokenType::Number(2.0)))
                ))
            ))
        ));

        assert_eq!(tree.tokens.len(),1);
        assert_eq!(tree.tokens[0].len(),1);
        if tree.tokens[0][0] != test_against {
            panic!("Equality check failed.\n\n  Left: {}\n  Right: {}\n\n",
                tree.tokens[0][0],
                test_against
            );
        }

    }
}