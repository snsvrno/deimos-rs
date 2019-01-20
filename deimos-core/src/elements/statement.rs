/// all references to a spec are from the [lua 5.1 spec](https://www.lua.org/manual/5.1/manual.html#8)

use crate::elements::Token;
use crate::elements::TokenType;

use crate::elements::CodeSlice;

#[derive(PartialEq,Debug)]
pub enum Statement {
    Empty,
    Token(Token),
    Unary(Token,Box<Statement>),                                // unop, expr
    Binary(Token,Box<Statement>,Box<Statement>),                // binop, expr1, expr2

    FieldNamed(Box<Statement>,Box<Statement>),                  // [expr]=expr 
    FieldBracket(Box<Statement>,Box<Statement>),                // Name=expr
    FieldList(Vec<Box<Statement>>),                             // field {fieldsep field} [fieldsep]

    TableConstructor(Vec<Box<Statement>>),                      // { fieldlist }

    DoEnd(Vec<Box<Statement>>),
    WhileDoEnd(Box<Statement>,Vec<Box<Statement>>),

    Assignment(Vec<Box<Statement>>,Vec<Box<Statement>>),        // varlist `=´ explist
    AssignmentLocal(Vec<Box<Statement>>,Vec<Box<Statement>>),   // local namelist [`=´ explist] 
}

impl Statement {
        pub fn tokens_to_statements(tokens : Vec<Token>) -> Vec<Statement> {
        //! convinence function that converts a vec<token> to a vec<statement>
        //! by wrapping each token with a statement.
        
        let mut statements : Vec<Statement> = Vec::new();

        for t in tokens {
            statements.push(Statement::Token(t));
        }

        statements
    }

    ///////////////////////////////////////////////////////////////////////
    /// ACCESS METHODS

    pub fn as_token_type<'a>(&'a self) -> &'a TokenType {
        match self {
            Statement::Token(ref token) => {
                &token.get_type()
            },
            _ => {
                panic!("Cannot unwrap {:?} as a Token",self)
            }
        }
    }

    pub fn get_code_slice(&self) -> CodeSlice {
        match self {
            Statement::Token(ref token) => token.get_code_slice().clone(),
            _ => CodeSlice::empty(),
        }
    }
    ///////////////////////////////////////////////////////////////////////
    /// IS CHECKS

    pub fn is_token(&self,token:TokenType) -> bool {
        //! checks if is a token and of that type

        match self {
            Statement::Token(t) => t.get_type() == &token,
            _ => false,
        }
    }

    pub fn is_unop(&self) -> bool {
        //! checking if a unary operator
        //! 
        //! ```text
        //! 
        //!     [x]    '-' | 
        //!     [x]    not |        
        //!     [x]    '#'
        //! 
        //! ``` 

        
        match self {
            Statement::Token(token) => match token.get_type() {
                TokenType::Minus | 
                TokenType::Not |
                TokenType::Pound => true,
                _ => false,
            }
            _ => false,
        }
    }

    pub fn is_binop(&self) -> bool {
        //! checking if a binary operator
        //! 
        //! ```text
        //! 
        //!     [x]    '+'  | 
        //!     [x]    '-'  | 
        //!     [x]    '*'  | 
        //!     [x]    '/'  | 
        //!     [x]    '^'  | 
        //!     [x]    '%'  | 
        //!     [x]    '..' | 
        //!     [x]    '<'  | 
        //!     [x]    '<=' | 
        //!     [x]    '>'  | 
        //!     [x]    '>=' | 
        //!     [x]    '==' | 
        //!     [x]    '~=' | 
        //!     [x]    and  | 
        //!     [x]    or
        //! 
        //! ```
        
        match self {
            Statement::Token(token) => match token.get_type() {
                TokenType::Plus |
                TokenType::Minus |
                TokenType::Star |
                TokenType::Slash |
                TokenType::Carrot |
                TokenType::Percent |
                TokenType::DoublePeriod |
                TokenType::GreaterThan |
                TokenType::LessThan |
                TokenType::GreaterEqual |
                TokenType::LessEqual |
                TokenType::EqualEqual |
                TokenType::NotEqual |
                TokenType::And |
                TokenType::Or => true,
                _ => false,
           },
           _ => false,
        }
    }

    pub fn is_fieldsep(&self) -> bool {
        //! checking if a field separator
        //! 
        //! ```text
        //! 
        //!     [x]   ',' | 
        //!     [x]   ';'
        //! 
        //! ```
        
        match self {
            Statement::Token(token) => match token.get_type() {
                TokenType::Comma |
                TokenType::SemiColon => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_expr(&self) -> bool {
        //! checking if an expression
        //! 
        //! ```text
        //! 
        //!     [x]   nil | 
        //!     [x]   false | 
        //!     [x]   true | 
        //!     [x]   Number | 
        //!     [x]   String | 
        //!     [ ]   '...' | 
        //!     [ ]   function | 
        //!     [ ]   prefixexp | 
        //!     [ ]   tableconstructor | 
        //!     [x]   exp binop exp | 
        //!     [x]   unop exp 
        //! 
        //! ```
    
        match self {
            Statement::Token(token) => match token.get_type() {
                TokenType::Nil | 
                TokenType::False | 
                TokenType::True |
                TokenType::Number(_) | 
                TokenType::String(_) => true,
                _ => false,
            },
            Statement::Binary(_,_,_) |
            Statement::Unary(_,_) => true,
            _ => false,
        }
    }

    pub fn is_var(&self) -> bool {
        //! checks if a statement is a variable
        //! 
        //! ```test
        //! 
        //!     [x]    Name
        //!     [ ]    prefixexp `[´ exp `]´
        //!     [ ]    prefixexp `.´ Name 
        //! 
        //! ```
        
        match self {
            Statement::Token(token) => {
                match token.get_type() {
                    TokenType::Identifier(_) => true,
                    _ => false,
                }
            },
            _ => false,
        }
    }

    pub fn is_stat(&self) -> bool {
        //! checking if a statement
        //! 
        //! ```text
        //! 
        //!     [x]   varlist `=´ explist | 
		//!     [ ]   functioncall | 
		//!     [x]   do block end | 
		//!     [x]   while exp do block end | 
		//!     [ ]   repeat block until exp | 
		//!     [ ]   if exp then block {elseif exp then block} [else block] end | 
		//!     [ ]   for Name `=´ exp `,´ exp [`,´ exp] do block end | 
		//!     [ ]   for namelist in explist do block end | 
		//!     [ ]   function funcname funcbody | 
		//!     [ ]   local function Name funcbody | 
		//!     [ ]   local namelist [`=´ explist] 
        //! 
        //! ```
        
        match self {
            Statement::DoEnd(_) |
            Statement::WhileDoEnd(_,_) |
            Statement::Assignment(_,_) => true,

            _ => false,
        }

    }

    pub fn is_name(&self) -> bool {
        //! name is the same thing is TokenType::Idenfitier
        
        match self {
            Statement::Token(ref token) => match token.get_type() {
                TokenType::Identifier(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_field(&self) -> bool {
        //! checking if something is a field
        //! 
        //! ```text
        //! 
        //!     [x]   '[' exp ']' '=' exp |
        //!     [x]   Name '=' exp | 
        //!     [x]   exp
        //! 
        //! ```
        
        if self.is_expr() { return true; }

        match self {
            Statement::FieldBracket(_,_) |
            Statement::FieldNamed(_,_) => true,
            _ => false,
        }
    }

    ///////////////////////////////////////////////////////////////
    /// COUNTING / COUNTER

    pub fn counting_loops(statement : &Statement, depth : &mut usize) {
        //! counting what loop we are in, so we can find nested loops and such,
        //! a loop is of types:
        //!
        //! ```test
        //!     do .. end
        //!     while .. do .. end
        //!     repeat .. until ..
        //!     if .. then .. end
        //!     for .. do .. end
        //! ```

        if let Statement::Token(token) = statement {
            match token.get_type() {
                // ending tokens
                TokenType::End => {
                    if *depth > 0 { *depth -= 1; }
                },

                // starting tokens
                //
                // doesn't do while or if because they aren't the true loop starter,
                // if you take some code like 
                //
                //      b = b + 1
                //      if b > 4 then
                //          b = b * 10
                //      end
                //
                // the `if b > 4` is the same depth as `b = b + 1`, but everything after the
                // `then` is deeper. `while .. do .. end` is the same for the `while .. do`
                // section
                TokenType::Do |
                TokenType::Then => *depth +=1,

                _ => (),
            }
        }
    }

    ///////////////////////////////////////////////////////////////
    /// INTO CONVERSIONS

    pub fn into_unary(self,expr : Statement) -> Statement {
        if !self.is_unop() { panic!("Cannot make {:?} into unary, not an operator.",self); }
        if !expr.is_expr() { panic!("Cannot make unary, {:?} isn't an expression.",expr); }

        match self {
            Statement::Token(token) => Statement::Unary(token,Box::new(expr)),
            _ => panic!("Cannot make {:?} into unary, not an operator.",self),
        }
        
    }

    pub fn into_binary(self,expr1 : Statement, expr2 : Statement) -> Statement {
        if !self.is_binop()  { panic!("Cannot make {:?} into binary, not an operator.",self); }
        if !expr1.is_expr() { panic!("Cannot make binary, expr1 {:?} isn't an expression.", expr1); }
        if !expr2.is_expr() { panic!("Cannot make binary, expr2 {:?} isn't an expression.", expr2); }

        match self {
            Statement::Token(token) => { 
                // need to check if we have another binary that resolved too soon,
                // basically we are checking the order of operation here.
                
                if let Statement::Binary(ref op,_,_) = expr1 {
                    if TokenType::oop_binary(&token.get_type(),&op.get_type()) {
                        let (op,n_expr1,n_expr2) = expr1.explode_binary();
                        let inner = Statement::Binary(token,Box::new(n_expr2),Box::new(expr2));
                        let outer = Statement::Binary(op,Box::new(n_expr1),Box::new(inner));
                        return outer;
                    }
                }
                
                Statement::Binary(token, Box::new(expr1), Box::new(expr2))
            },
            _ => panic!("Cannot make {:?} into binary, not an operator.", self),
        }

    }

    ///////////////////////////////////////////////////////////////////
    /// CREATIONS

    pub fn create_do_end(mut statements : Vec<Statement>) -> Statement {
        let mut list : Vec<Box<Statement>> = Vec::new();
        for s in (0 .. statements.len()).rev() {
            list.push(Box::new(statements.remove(s)));
        }
        Statement::DoEnd(list)
    }

    fn convert_to_box_list(list : Vec<Statement>) -> Vec<Box<Statement>> {
        let mut new_list : Vec<Box<Statement>> = Vec::new();

        for l in list {
            new_list.push(Box::new(l))
        }

        new_list
    }

    pub fn create_assignment(mut vars: Vec<Statement>, mut exprs : Vec<Statement>, local : bool) -> Statement {
        // gets the two lists the same length
        loop {
            if vars.len() == exprs.len() { break; }
            match vars.len() > exprs.len() {
                true => exprs.push(Statement::Empty),
                false => vars.push(Statement::Empty),
            }
        }
        
        if vars.len() != exprs.len() {
            panic!("Error creating assignment, varlist and expr list must be the same!");
        }


        match local {
            true => Statement::AssignmentLocal(Statement::convert_to_box_list(vars),Statement::convert_to_box_list(exprs)),
            false => Statement::Assignment(Statement::convert_to_box_list(vars),Statement::convert_to_box_list(exprs))
        }
    }


    ///////////////////////////////////////////////////////////////////
    /// EXPLOSIONS

    pub fn explode_binary(self) -> (Token,Statement,Statement) {
        match self {
            Statement::Binary(op,ex1,ex2) => {
                return (op,*ex1,*ex2);
            },
            _ => panic!("Exploding {:?} as a binary isn't allowed, not a binary.",self),

        }
    }

    ///////////////////////////////////////////////////////////////////
    /// DISPLAY HELPERS
    ///
    
    fn render_list(list : &Vec<Box<Statement>>) -> String {
        let mut items = String::new();

        for i in 0 .. list.len() {
            if i > 0 {
                items = format!("{}, {}",items,list[i]);
            } else {
                items = format!("{}",list[i]);
            }
        }

        items
    } 

    fn render_statements(list : &Vec<Box<Statement>>) -> String {
        if list.len() == 0 {
            return "".to_string();
        }

        let mut string = format!("{}",list[0]);

        if list.len() == 1 { return string; }
        else {
            for i in 1 .. list.len() {
                string = format!("{}\n{}",string,list[i]);
            }
            return string;
        }
    }
}   

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Statement::Token(token) => write!(f,"{}",token),
            Statement::Unary(op,expr) => write!(f,"({} {})",op,expr),
            Statement::Binary(op,e1,e2) => write!(f,"({} {} {})",op,e1,e2),
            
            Statement::FieldNamed(name,expr) => write!(f,"{} = {}",name,expr),     
            Statement::FieldBracket(expr1,expr2) => write!(f,"[{}] = {}",expr1,expr2),
            Statement::FieldList(list) => write!(f,"{}",Statement::render_list(&list)),
            Statement::TableConstructor(list) => write!(f,"[ {} ]",Statement::render_list(&list)),

            Statement::DoEnd(stats) => write!(f,"(do {} end)",Statement::render_statements(&stats)),
            Statement::WhileDoEnd(expr,stats) => write!(f,"(while {} do {} end)",expr,Statement::render_statements(&stats)),

            Statement::Assignment(varlist,exprlist) => write!(f,"(= {} {})",Statement::render_list(&varlist),Statement::render_list(&exprlist)),
            Statement::AssignmentLocal(varlist,exprlist) => write!(f,"(= local {} {})",Statement::render_list(&varlist),Statement::render_list(&exprlist)),

            Statement::Empty => write!(f,"nil"),
        }
    }
}

mod tests {

    #[test]
    fn unop() {
        use crate::elements::{ Token, TokenType, Statement };

        for t in vec![TokenType::Minus, TokenType::Not, TokenType::Pound] {
            let statement = Statement::Token(Token::simple(t));
            assert!(statement.is_unop());
        }

        for t in vec![TokenType::Plus, TokenType::Star] {
            let statement = Statement::Token(Token::simple(t));
            assert!(!statement.is_unop());
        }

    }
    
    #[test]
    fn binop() {
        use crate::elements::{ Token, TokenType, Statement };

        for t in vec![
            TokenType::Plus, TokenType::Minus, TokenType::Star,
            TokenType::Slash, TokenType::Carrot, TokenType::Percent,
            TokenType::DoublePeriod, TokenType::GreaterThan, TokenType::GreaterEqual,
            TokenType::LessThan, TokenType::LessEqual, TokenType::Or, 
            TokenType::EqualEqual, TokenType::NotEqual, TokenType::And 
        ] {
            let statement = Statement::Token(Token::simple(t));
            println!("{}",statement);
            assert!(statement.is_binop());
        }

        for t in vec![TokenType::Not, TokenType::Pound] {
            let statement = Statement::Token(Token::simple(t));
            assert!(!statement.is_binop());
        }

    }
    
    #[test]
    fn fieldsep() {
        use crate::elements::{ Token, TokenType, Statement };

        for t in vec![ TokenType::Comma, TokenType::SemiColon ] {
            let statement = Statement::Token(Token::simple(t));
            println!("{}",statement);
            assert!(statement.is_fieldsep());
        }

        for t in vec![TokenType::Not, TokenType::Pound] {
            let statement = Statement::Token(Token::simple(t));
            assert!(!statement.is_fieldsep());
        }
    }

}
