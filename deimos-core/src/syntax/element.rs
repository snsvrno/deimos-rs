use crate::codewrap::CodeWrappable;
use crate::token::Token;

#[derive(Debug)]
pub enum SyntaxElement {
    Token(Token),

    // statements and their children
    StatementAssign(Box<SyntaxElement>, Box<SyntaxElement>),    // the left, the right
    
    VarList(Vec<Box<SyntaxElement>>),

    Var(Box<SyntaxElement>),

    ExpList(Vec<Box<SyntaxElement>>),

    // expressions and their children
    Exp(Box<SyntaxElement>),
    Binop(Box<SyntaxElement>, Box<SyntaxElement>, Box<SyntaxElement>), // exp1, op, exp2
    Unop(Box<SyntaxElement>, Box<SyntaxElement>),   // op, exp
}

impl CodeWrappable for SyntaxElement { }


impl std::fmt::Display for SyntaxElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxElement::Token(token) => write!(f, "{}", token),
            SyntaxElement::Binop(left, op, right) => write!(f, "({} {} {})", op, left, right),
            SyntaxElement::Unop(op, exp) => write!(f, "({} {})", op, exp),
            SyntaxElement::Exp(item) => write!(f, "{}", item),
            SyntaxElement::ExpList(list) => write!(f, "<Exp {}>", SyntaxElement::list_to_string(list,", ")),
            SyntaxElement::Var(item) => write!(f, "{}", item),
            SyntaxElement::VarList(list) => write!(f, "<Var {}>", SyntaxElement::list_to_string(list,", ")),
            SyntaxElement::StatementAssign(left,right) => write!(f, "(= {} {})", left, right),

            _ => write!(f, "SyntaxElement not defined!!")
        }
    }
}

impl SyntaxElement {

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
            SyntaxElement::StatementAssign(_,_) => true,
            _ => false,
        }
    }
    
    fn list_to_string(list : &Vec<Box<SyntaxElement>>, divider : &str) -> String {
        let mut string : String = String::new();

        for item in list {
            string = format!("{}{}{}",string, item, divider);
        }

        return string;
    }

}