use crate::codewrap::CodeWrappable;
use crate::token::Token;

#[derive(Debug)]
pub enum SyntaxElement {
    Token(Token),

    // statements and their children
    StatementAssign(Box<SyntaxElement>, Box<SyntaxElement>),    // the left, the right
    StatementDoEnd(Box<SyntaxElement>),
    StatementLast(Option<Box<SyntaxElement>>),

    VarList(Vec<Box<SyntaxElement>>),

    Var(Box<SyntaxElement>),

    ExpList(Vec<Box<SyntaxElement>>),

    // expressions and their children
    Exp(Box<SyntaxElement>),
    Binop(Box<SyntaxElement>, Box<SyntaxElement>, Box<SyntaxElement>), // exp1, op, exp2
    Unop(Box<SyntaxElement>, Box<SyntaxElement>),   // op, exp

    Chunk(Vec<Box<SyntaxElement>>),
    Block(Box<SyntaxElement>),
}

impl CodeWrappable for SyntaxElement { }


impl std::fmt::Display for SyntaxElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxElement::Token(token) => write!(f, "{}", token),
            SyntaxElement::Binop(left, op, right) => write!(f, "({} {} {})", op, left, right),
            SyntaxElement::Unop(op, exp) => write!(f, "({} {})", op, exp),
            SyntaxElement::Exp(item) => write!(f, "{}", item),
            SyntaxElement::ExpList(list) => write!(f, "<Exp {}>", SyntaxElement::list_to_string(list,", ","")),
            SyntaxElement::Var(item) => write!(f, "{}", item),
            SyntaxElement::VarList(list) => write!(f, "<Var {}>", SyntaxElement::list_to_string(list,", ","")),
            SyntaxElement::StatementAssign(left,right) => write!(f, "(= {} {})", left, right),
            SyntaxElement::StatementDoEnd(block) => write!(f, "(do\n{}end)", block),
            SyntaxElement::Block(chunk) => write!(f, "{}", chunk),
            SyntaxElement::Chunk(statements) => write!(f, "{}", SyntaxElement::list_to_string(statements,"\n","  ")),

            _ => write!(f, "SyntaxElement not defined!!")
        }
    }
}

impl SyntaxElement {

    pub fn ending_token(&self) -> Token {
        //! used to find the ending token of this phrase, to be
        //! used in the block_stack.

        match self {
            SyntaxElement::StatementDoEnd(_) => Token::End,
            _ => { assert!(false); Token::Nil },
        }
    }

    pub fn is_block(&self) -> bool {
        match self {
            SyntaxElement::Block(_) => true,
            __ => false,
        }
    }

    pub fn is_exp(&self) -> bool {
        match self {
            SyntaxElement::Exp(_) => true,
            _ => false,
        }
    }

    pub fn is_exp_list(&self) -> bool {
        match self {
            SyntaxElement::ExpList(_) => true,
            _ => false,
        }
    }

    pub fn is_var_list(&self) -> bool {
        match self {
            SyntaxElement::VarList(_) => true,
            _ => false,
        }
    }

    pub fn is_statement(&self) -> bool {
        match self {
            SyntaxElement::StatementAssign(_,_) |
            SyntaxElement::StatementDoEnd(_) => true,
            _ => false,
        }
    }

    pub fn is_last_statement(&self) -> bool {
        match self {
            SyntaxElement::StatementLast(_) => true,
            _ => false,
        }
    }
    
    fn list_to_string(list : &Vec<Box<SyntaxElement>>, divider : &str, padding : &str) -> String {
        let mut string : String = String::new();

        for item in list {
            string = format!("{}{}{}{}",string, padding, item, divider);
        }

        return string;
    }

}