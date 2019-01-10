use crate::scanner::Scanner;
use failure::Error;

use crate::token::Token;
use crate::tokentype::TokenType;
use crate::chunk::Chunk;

use crate::grammar::gram::Gram;
use crate::grammar::binary::Binary;
use crate::grammar::unary::Unary;
use crate::grammar::expression::Expression;
use crate::grammar::grouping::Grouping;

use crate::grammar::blockdo::BlockDo;
use crate::grammar::blockwhile::BlockWhile;
use crate::grammar::blockrepeat::BlockRepeat;

pub struct Tree<'a> {
    raw_code : &'a str,
    tokens : Vec<Chunk>,
}

impl<'a> Tree<'a> {
    pub fn from_scanner(scanner : Scanner<'a>) -> Result<Tree,Error> {
        let (raw_code,mut raw_tokens) = scanner.explode();

        let mut tokens : Vec<Chunk> = Vec::new();
        let mut sub_tokens : Chunk = Chunk::new();

        // loops through the linear list of tokens and groups them into statements.
        loop {
            let token : Option<Token> = if raw_tokens.len() > 0 { Some(raw_tokens.remove(0)) } else { None }; 

            match token {
                None => break,
                Some(token) => {
                    match token.get_type() {
                        TokenType::SemiColon | 
                        TokenType::EOL => {
                            if sub_tokens.len() > 0 { 
                                tokens.push(sub_tokens);
                                sub_tokens = Chunk::new();
                            }
                        },
                        // special tokens that should be on their own Chunk
                        TokenType::Repeat |
                        TokenType::Until |
                        TokenType::While |
                        TokenType::End | 
                        TokenType::Do => {
                            if sub_tokens.len() > 0 {
                                tokens.push(sub_tokens);
                                sub_tokens = Chunk::new();
                            }

                            let gram = Gram::create(token);
                            match Expression::create_into_gram(&gram) {
                                None => tokens.push(Chunk::new_from(vec![gram])),
                                Some(expr) => tokens.push(Chunk::new_from(vec![expr])),
                            }                           
                        },
                        _ => {
                            let gram = Gram::create(token);
                            match Expression::create_into_gram(&gram) {
                                None => sub_tokens.add(gram),
                                Some(expr) => sub_tokens.add(expr),
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

        let tree = Tree {
            raw_code : raw_code,
            tokens : tokens,
        };

        Ok(tree)
    }

    pub fn create_tree(mut self) -> Result<Self,Error> {

        // works on all the grams, except for blocks
        for mut line in self.tokens.iter_mut() {
            let mut tier = 0;
            loop {
                // we'll check for grouping every iteration
                // new groupings might be generated every time we do something.
                if Grouping::process_set(&mut line)? {
                    // we restart the tiers if we find a grouping match because
                    // grouping helps us go out of order, so there will probably 
                    // be a new match to a higher priority operation that is now
                    // available.
                    //
                    //    (2 + 3) * 4
                    //
                    // in the above example, (2+3) will be grouped after the '+' is
                    // resolved (so in binary tier 4), '*' is binary tier 3, so we need
                    // to re run that tier in order to get the '*' to match.
                    tier = 0;
                }

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

        // now that everything is 'compressed', lets try and find some blocks
        BlockRepeat::process(&mut self.tokens)?;
        BlockWhile::process(&mut self.tokens)?;
        BlockDo::process(&mut self.tokens)?;


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

        let test_against = expression!(&binary!(
            &token!(TokenType::Equal),
            &expression!(&literal!(TokenType::Identifier("bob".to_string()))),
            &expression!(&binary!(
                &token!(TokenType::Plus),
                &expression!(&literal!(TokenType::Number(5.0))),
                &expression!(&literal!(TokenType::Number(2.0)))
            ))
        ));

        assert_eq!(tree.tokens.len(),1);
        assert_eq!(tree.tokens[0].len(),1);
        
        if tree.tokens[0].at(0) != &test_against {
            panic!("Equality check failed.\n\n  Left: {}\n  Right: {}\n\n",
                tree.tokens[0].at(0),
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

        let test_against = expression!(&binary!(
            &token!(TokenType::Equal),
            &expression!(&literal!(TokenType::Identifier("bob".to_string()))),
            &expression!(&binary!(
                &token!(TokenType::Plus),
                &expression!(&literal!(TokenType::Number(5.0))),
                &expression!(&unary!(
                    &token!(TokenType::Minus),
                    &expression!(&literal!(TokenType::Number(2.0)))
                ))
            ))
        ));

        assert_eq!(tree.tokens.len(),1);
        assert_eq!(tree.tokens[0].len(),1);
        
        if tree.tokens[0].at(0) != &test_against {
            panic!("Equality check failed.\n\n  Left: {}\n  Right: {}\n\n",
                tree.tokens[0].at(0),
                test_against
            );
        }

    }

    #[test]
    fn simple_grouping() {
        use crate::scanner::Scanner;
        use crate::tree::Tree;
        use crate::tokentype::TokenType;
        
        let scanner = Scanner::new("bob = (2+3)*4").scan().unwrap();
        let tree = Tree::from_scanner(scanner).unwrap().create_tree().unwrap();

        let test_against = expression!(&binary!(
            &token!(TokenType::Equal),
            &expression!(&literal!(TokenType::Identifier("bob".to_string()))),
            &expression!(&binary!(
                &token!(TokenType::Star),
                &expression!(&grouping!(&expression!(&binary!(
                    &token!(TokenType::Plus),
                    &expression!(&literal!(TokenType::Number(2.0))),
                    &expression!(&literal!(TokenType::Number(3.0)))
                )))),
                &expression!(&literal!(TokenType::Number(4.0)))
            ))
        ));

        assert_eq!(tree.tokens.len(),1);
        assert_eq!(tree.tokens[0].len(),1);

        if tree.tokens[0].at(0) != &test_against {
            panic!("Equality check failed.\n\n  Left: {}\n  Right: {}\n\n",
                tree.tokens[0].at(0),
                test_against
            );
        }

    }
}