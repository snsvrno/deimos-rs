use crate::codewrap::CodeWrappable;
use crate::token::Token;

#[derive(Debug)]
pub enum SyntaxElement {
    Token(Token),

    // statements and their children
    StatementAssign(Box<SyntaxElement>, Box<SyntaxElement>),    // the left, the right
    StatementLocalAssign(Box<SyntaxElement>,Option<Box<SyntaxElement>>), // the varname and option assignment
    StatementDoEnd(Box<SyntaxElement>),
    StatementLast(Option<Box<SyntaxElement>>),

    VarList(Vec<Box<SyntaxElement>>),

    Var(Box<SyntaxElement>),
    VarDot(Box<SyntaxElement>, Box<SyntaxElement>),     // left.right
    VarIndex(Box<SyntaxElement>, Box<SyntaxElement>),    // left[right]

    NameList(Vec<Box<SyntaxElement>>),

    ExpList(Vec<Box<SyntaxElement>>),

    // expressions and their children
    Exp(Box<SyntaxElement>),
    Binop(Box<SyntaxElement>, Box<SyntaxElement>, Box<SyntaxElement>), // exp1, op, exp2
    Unop(Box<SyntaxElement>, Box<SyntaxElement>),   // op, exp

    PrefixExp(Box<SyntaxElement>),

    TableConstructor(Box<SyntaxElement>),
    Field(Box<SyntaxElement>),
    FieldAssignment(Box<SyntaxElement>,Box<SyntaxElement>),
    FieldList(Vec<Box<SyntaxElement>>),

    Chunk(Vec<Box<SyntaxElement>>),
    Block(Box<SyntaxElement>),

    Empty // used for creating dummy objects, so i don't need to actually put anything real in there.
}

impl CodeWrappable for SyntaxElement { }


impl std::fmt::Display for SyntaxElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxElement::Token(token) => write!(f, "{}", token),
            SyntaxElement::Binop(left, op, right) => write!(f, "({} {} {})", op, left, right),
            SyntaxElement::Unop(op, exp) => write!(f, "({} {})", op, exp),
            SyntaxElement::Exp(item) => write!(f, "{}", item),
            SyntaxElement::PrefixExp(item) => write!(f, "{}", item),
            SyntaxElement::ExpList(list) => write!(f, "<Exp {}>", SyntaxElement::list_to_string(list,", ","")),
            SyntaxElement::Var(item) => write!(f, "{}", item),
            SyntaxElement::VarDot(left, right) => write!(f, "{}.{}", left, right),
            SyntaxElement::VarIndex(left, right) => write!(f, "{}[{}]", left, right),
            SyntaxElement::VarList(list) => write!(f, "<Var {}>", SyntaxElement::list_to_string(list,", ","")),
            SyntaxElement::StatementAssign(left,right) => write!(f, "(= {} {})", left, right),
            SyntaxElement::StatementDoEnd(block) => write!(f, "(do\n{}end)", block),
            SyntaxElement::Block(chunk) => write!(f, "{}", chunk),
            SyntaxElement::Chunk(statements) => write!(f, "{}", SyntaxElement::list_to_string(statements,"\n","  ")),
            SyntaxElement::FieldList(list) => write!(f, "<Fields {}>", SyntaxElement::list_to_string(list,", ","")),
            SyntaxElement::FieldAssignment(left,right) => write!(f, "(= {} {})", left, right),
            SyntaxElement::Field(token) => write!(f, "{}", token),
            SyntaxElement::TableConstructor(list) => write!(f, "(Table {})", list),
            SyntaxElement::NameList(list) => write!(f, "<Name {}>", SyntaxElement::list_to_string(list,", ","")),
            SyntaxElement::StatementLocalAssign(left,Some(right)) => write!(f, "(= local {} {})", left, right),
            SyntaxElement::StatementLocalAssign(left,None) => write!(f, "(= local {})", left),

            _ => write!(f, "SyntaxElement not defined!!")
        }
    }
}

impl SyntaxElement {

    pub fn ref_to_inside<'a>(&'a self) -> &SyntaxElement {
        //! used to return the inside item, a reference
        
        match self {
            SyntaxElement::Var(ref item) => item,
            _ => unimplemented!(),
        }
    }

    pub fn ending_token(&self) -> Token {
        //! used to find the ending token of this phrase, to be
        //! used in the block_stack.

        match self {
            SyntaxElement::StatementDoEnd(_) => Token::End,
            SyntaxElement::TableConstructor(_) => Token::RightMoustache,
            _ => { assert!(false); Token::Nil },
        }
    }

    pub fn does_match_token(&self, token : Token) -> bool {
        match self {
            SyntaxElement::Token(this_token) => this_token == &token,
            __ => false,
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

    pub fn is_field(&self) -> bool {
        match self {
            SyntaxElement::Field(_) | 
            SyntaxElement::FieldAssignment(_,_) => true,
            _ => false,
        }
    }

    pub fn is_prefix_exp(&self) -> bool {
        match self {
            SyntaxElement::PrefixExp(_) => true,
            _ => false,
        }
    }

    pub fn is_name(&self) -> bool {
        match self {
            SyntaxElement::Token(Token::Identifier(_)) => true,
            _ => false,
        }
    }

    pub fn is_name_list(&self) -> bool {
        match self {
            SyntaxElement::NameList(_) => true,
            _ => false,
        }
    }

    pub fn is_var(&self) -> bool {
        match self {
            SyntaxElement::Var(_) | 
            SyntaxElement::VarDot(_,_) => true,
            _ => false,
        }
    }

    pub fn insides_single_only<'a>(&'a self) -> Option<&'a SyntaxElement> {
        match self {
            SyntaxElement::Exp(ref insides) => Some(&*insides),
            SyntaxElement::PrefixExp(ref insides) => Some(&*insides),
            SyntaxElement::Var(ref insides) => Some(&*insides),
            _ => None,
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
            SyntaxElement::StatementLocalAssign(_,_) |
            SyntaxElement::StatementDoEnd(_) => true,
            _ => false,
        }
    }

    pub fn is_table(&self) -> bool {
        match self {
            SyntaxElement::TableConstructor(_) => true,
            _ => false,
        }
    }

    pub fn is_last_statement(&self) -> bool {
        match self {
            SyntaxElement::StatementLast(_) => true,
            _ => false,
        }
    }

    pub fn convert_to_name(self) -> Result<SyntaxElement,SyntaxElement> {
        //! attempts to convert to a name, if it succeeds its Ok() with the 
        //! new name, if it fails then its Err() with the old element
        
        match self {
            SyntaxElement::Var(token) => {
                let insides = *token;
                match insides.is_name() {
                    true => Ok(insides),
                    false => Err(SyntaxElement::Var(Box::new(insides))),
                } 
            },
            other_element => Err(other_element),
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