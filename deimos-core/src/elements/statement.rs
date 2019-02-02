/// all references to a spec are from the [lua 5.1 spec](https://www.lua.org/manual/5.1/manual.html#8)

use failure::{Error,format_err};

use crate::elements::Token;
use crate::elements::TokenType;

use crate::elements::CodeSlice;
use crate::elements::Scope;

use crate::elements::statement_evals;

use std::collections::HashMap;

#[derive(PartialEq,Clone,Debug,Hash,Eq)]
pub enum TableIndex {
    Number(String),
    String(String),
}

impl TableIndex {
    pub fn create(text : &str) -> TableIndex {
        match text.parse::<f32>() {
            Err(_) => TableIndex::String(text.to_string()),
            Ok(_) => TableIndex::Number(text.to_string()),
        }
    }
}

impl std::fmt::Display for TableIndex {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TableIndex::Number(num) => write!(f,"{}",num),
            TableIndex::String(st) => write!(f,"\"{}\"",st),
        }
    }
}

#[derive(PartialEq,Debug,Clone)]
pub enum Statement {
    Empty,
    Token(Token),
    Unary(Token,Box<Statement>),                                    // unop, expr
    Binary(Token,Box<Statement>,Box<Statement>),                    // binop, expr1, expr2
    Group(Box<Statement>),

    FieldNamed(Box<Statement>,Box<Statement>),                      // [expr]=expr 
    FieldBracket(Box<Statement>,Box<Statement>),                    // Name=expr
    FieldList(Vec<Box<Statement>>),                                 // field {fieldsep field} [fieldsep]

    ComplexVar(Vec<Box<Statement>>),                                // a var but with access stuff --->  prefixexp `[´ exp `]´ | prefixexp `.´ Name 

    ExprList(Vec<Box<Statement>>),   
    VarList(Vec<Box<Statement>>),   
    NameList(Vec<String>),   
    
    Table(HashMap<TableIndex,Statement>),

    TableConstructor(Vec<Box<Statement>>),                          // { fieldlist }

    DoEnd(Vec<Box<Statement>>),
    WhileDoEnd(Box<Statement>,Vec<Box<Statement>>),

    Assignment(Box<Statement>,Box<Statement>),                 // varlist `=´ explist
    AssignmentLocal(Box<Statement>,Box<Statement>),            // local namelist [`=´ explist] 

    Function(Vec<String>,Vec<Box<Statement>>),                      // funcbody ::= `(´ [parlist] `)´ block end
    FunctionNamed(String,Vec<String>,Vec<Box<Statement>>),          // funcbody ::= name`(´ [parlist] `)´ block end
    FunctionCall(Box<Statement>,Box<Statement>),                       
    Return(Box<Statement>),                                         // laststat ::= return [explist] | break
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

    pub fn eval_as_function(&self, mut scope : &mut Scope, arg : &Statement) -> Result<Statement,Error> {
        match self {
            Statement::Function(ref args, ref content) => {
                // sets all the arguements
                scope.push_local();
                for i in 0 .. args.len() {
                    let value : Statement = if let Some(value) = arg.as_list_index(i) {
                        value.clone()
                    } else {
                        Statement::Empty
                    };
                    scope.assign_local(&args[i],value);
                }

                let mut returned = Statement::Empty;
                
                // evaluating the function
                for line in content.iter() {
                    let result = line.eval(&mut scope)?;
                    if let Statement::Return(result) = result {
                        returned = *result;                        
                        break;
                    }
                }

                scope.pop_local();
                Ok(returned)
            },
            _ => Err(format_err!("Cannot evaluate {} as a function",self)),
        }
    }

    pub fn eval(&self, mut scope : &mut Scope) -> Result<Statement,Error> {
        match self {
            Statement::ComplexVar(ref address) => {
                let var_name = {
                    let mut string = String::new();

                    for i in 0 .. address.len() {
                        // needs to evaluate each piece, because what if we put some logic in there or something
                        // like `x[y+1]`

                        if i == 0 {
                            let section : Statement = if address[i].is_name() { *address[i].clone() } else { address[i].eval(scope)? };
                            string = format!("{}",section.as_user_output().unwrap());
                        } else {
                            let section = address[i].eval(scope)?;
                            string = format!("{}[{}]",string,section.as_user_output().unwrap());
                        }
                    }

                    string
                };

                match scope.get_value(&var_name) {
                    Some(value) => Ok(value.clone()),
                    None => Ok(Statement::Empty),
                }
            },
            Statement::NameList(ref list) => {
                let mut return_list : Vec<Box<Statement>> = Vec::new();

                for name in list.iter() {
                    let value : Statement = match scope.get_value(&name) {
                        None => Statement::Empty,
                        Some(value) => value.clone(),
                    };
                    return_list.push(Box::new(value));
                }

                Ok(Statement::ExprList(return_list))
            },
            Statement::Group(ref insides) => insides.eval(scope),
            Statement::ExprList(ref list) => {
                let mut new_list : Vec<Box<Statement>> = Vec::new();
                for i in 0 .. list.len() {
                    new_list.push(Box::new(list[i].eval(scope)?));
                }
                if list.len() == 1 {
                    Ok(*new_list.remove(0))
                } else {
                    Ok(Statement::ExprList(new_list))
                }
            },
            Statement::Return(stat) => {
                let result = stat.eval(scope)?;
                Ok(Statement::Return(Box::new(result)))
            },
            Statement::FunctionNamed(ref name,ref args,ref content) => {
                let func =Statement::Function(args.clone(),content.clone());
                scope.register_function(name,func);
                Ok(Statement::Empty)
            },
            Statement::FunctionCall(ref name,ref args) => {
                scope.eval_function(&name.as_name(),&args)
            },
            Statement::Unary(op,s1) => {
                let eval_s1 = s1.eval(&mut scope)?;
                match op.get_type() {
                    TokenType::Minus => statement_evals::unop::minus(&eval_s1),
                    TokenType::Not => statement_evals::unop::not(&eval_s1),
                    TokenType::Pound |
                    _ => Err(format_err!("{} is not a unary operator", op)),
                }
            },
            Statement::Binary(op,s1,s2) => {
                let eval_s1 = s1.eval(&mut scope)?;
                let eval_s2 = s2.eval(&mut scope)?;
                match op.get_type() {
                    TokenType::Plus => statement_evals::binop::plus(&eval_s1,&eval_s2),
                    TokenType::Minus => statement_evals::binop::minus(&eval_s1,&eval_s2),
                    TokenType::Star => statement_evals::binop::star(&eval_s1,&eval_s2),
                    TokenType::Slash => statement_evals::binop::slash(&eval_s1,&eval_s2),
                    TokenType::Carrot => statement_evals::binop::carrot(&eval_s1,&eval_s2),
                    TokenType::Percent => statement_evals::binop::percent(&eval_s1,&eval_s2),
                    TokenType::DoublePeriod => statement_evals::binop::double_period(&eval_s1,&eval_s2),
                    TokenType::LessThan |
                    TokenType::LessEqual |
                    TokenType::GreaterThan |
                    TokenType::GreaterEqual |
                    TokenType::EqualEqual |
                    TokenType::NotEqual |
                    TokenType::And |
                    TokenType::Or |
                    _ => Err(format_err!("{} is not a binary operator",op)),
                }
            },
            Statement::Assignment(ref vars, ref exprs) => {
                // when performing assignments, the exprlist (the right side)
                // must be evaluated before anything else is evaluated. so that
                // you should be able to do the following.
                //
                // x,y = y,x
                //
                // and actually swap the two values.
            
                let mut results : Vec<Statement> = Vec::new();

                /*for ex in exprs.as_list() {
                    results.push(ex.eval(&mut scope)?);
                }*/

                let val_exprs = exprs.eval(&mut scope)?;

                if vars.is_namelist() {
                    let list = vars.as_namelist();
                    for i in (0 .. list.len()).rev() {
                        let value = match val_exprs.as_list_index(i) {
                            None => Statement::Empty,
                            Some(value) => value.clone(),                        
                        };
                        scope.assign(&list[i], value);
                    }
                } else {
                    let list = vars.as_list();
                    for i in (0 .. list.len()).rev() {
                        let value = match val_exprs.as_list_index(i) {
                            None => Statement::Empty,
                            Some(value) => value.clone(),                        
                        };

                        match *list[i] {
                            Statement::Token(ref token) => match token.get_type() {
                                TokenType::Identifier(ref var_name) => {
                                    scope.assign(&var_name,value)?;
                                },
                                _=> { return Err(format_err!("Assignment: don't know what to do with {}",token)); },
                            },
                            _ => (),
                        }
                    }
                }

                Ok(Statement::Empty)
            },
            Statement::Token(ref token) => match token.get_type() {
                TokenType::Identifier(ref var_name) => match scope.get_value(var_name) {
                    Some(value) => Ok(value.clone()),
                    None => Ok(Statement::Empty),
                },
                _ => Ok(self.clone())
            },
            Statement::Table(ref table) => {
                let mut eval_table : HashMap<TableIndex,Statement> = HashMap::new();

                for (k,v) in table {
                    eval_table.insert(k.clone(),v.eval(scope)?);
                }

                Ok(Statement::Table(eval_table))
            },
            _ => Ok(Statement::Empty)
        }
    }

    ///////////////////////////////////////////////////////////////////////
    /// ACCESS METHODS

    pub fn as_user_output(&self) -> Option<String> {
        match self {
            Statement::Token(ref token) => match token.get_type() {
                TokenType::Number(num) => Some(format!("{}",num)),
                TokenType::String(string) => Some(format!("{}",string)),
                TokenType::Identifier(id) => Some(format!("{}",id)),
                TokenType::Nil => Some(format!("nil")),
                _ => None,
            },
            Statement::ExprList(ref list) => {
                let mut string = String::new();
                for i in 0 .. list.len() {
                    if i == 0 {
                        string = format!("{}",list[i].as_user_output().unwrap());
                    } else {
                        string = format!("{}    {}",string,list[i].as_user_output().unwrap());
                    }
                }
                Some(string)
            },
            Statement::Table(ref table) => {
                // TODO, some way to print this?
                Some(format!("table 0x123"))
            },
            Statement::Empty => Some(format!("nil")),
            _ => None,
        }
    }

    pub fn as_token_type<'a>(&'a self) -> &'a TokenType {
        match self {
            Statement::Token(ref token) => &token.get_type(),
            _ => panic!("Cannot unwrap {:?} as a Token",self),
        }
    }
    
    pub fn len(&self) -> usize {
        match self {
            Statement::FieldList(ref list) => list.len(),
            Statement::ExprList(ref list) => list.len(),
            Statement::VarList(ref list) => list.len(),
            Statement::NameList(ref list) => list.len(),

            _ => 0,
        }
    }

    pub fn as_number<'a>(&'a self) -> &'a f32 {
        match self {
            Statement::Token(ref token) => match token.get_type() {
                TokenType::Number(num) => num,
                _ => panic!("Cannot unwrap {:?} as a number",self),
            },
            _ => panic!("Cannot unwrap {:?} as a number",self),
        }
    }
    
    pub fn as_bool(&self) -> bool {
        match self {
            Statement::Token(ref token) => match token.get_type() {
                TokenType::False => false,
                TokenType::True => true,
                _ => panic!("Cannot unwrap {:?} as a bool", self),
            },
            _ => panic!("Cannot unwrap {:?} as a bool", self),
        }
    }

    pub fn as_list<'a>(&'a self) -> Vec<&'a Statement> {
        match self {
            Statement::ExprList(ref list) |
            Statement::VarList(ref list) => {
                let mut new_list : Vec<&Statement> = Vec::new();
                for i in 0 .. list.len() {
                    new_list.push(&list[i]);
                }
                new_list
            },
            _ => vec![&self],
        }
    }

    pub fn as_list_index<'a>(&'a self, i : usize) -> Option<&'a Statement> {
        match self {
            Statement::VarList(ref list) |
            Statement::ExprList(ref list) => {
                if list.len() > i {
                    Some(&list[i])
                } else { None }
            },
            _ => {
                if i == 0 { Some(&self) } else { None }
            },
        }
    }   

    pub fn as_namelist<'a>(&'a self) -> &'a Vec<String> {
        match self {
            Statement::NameList(ref list) => &list,
            _ => panic!("Cannot unwrap {:?} as a name list",self),
        }
    }

    pub fn as_namelist_mut<'a>(&'a mut self) -> &'a mut Vec<String> {
        match self {
            Statement::NameList(ref mut list) => list,
            _ => panic!("Cannot unwrap {:?} as a name list",self),
        }
    }

    pub fn as_list_mut<'a>(&'a mut self) -> &'a mut Vec<Box<Statement>> {
        match self {
            Statement::VarList(ref mut list) => list,
            Statement::ExprList(ref mut list) => list,
            _ => panic!("Cannot unwrap {:?} as a mut list",self),
        }
    }

    pub fn cast_to_bool(&self) -> Option<bool> {
        if self.is_bool() {
            return Some(self.as_bool().clone());
        }

        None
    }

    pub fn cast_to_number(&self) -> Option<f32> {
        if self.is_number() { 
            return Some(self.as_number().clone()); 
        }
        
        if self.is_string() { 
            return match self.as_string().parse::<f32>() {
                Err(_) => None,
                Ok(float) => Some(float), 
            };
        }

        None
    }

    fn into_varlist(self) -> Statement {
        //! turns a namelist into a var list
        //! will panic
         
        if !self.is_namelist() {
            panic!("Can only change a namelist into a varlist, not sure how to do it for {}",self);
        }

        let mut items : Vec<Box<Statement>> = Vec::new();
        let code_slice = self.get_code_slice();
        for i in self.as_namelist() {
            let token = Statement::Token(Token::new(TokenType::Identifier(i.to_string()),code_slice.clone()));
            items.push(Box::new(token));
        }
        Statement::VarList(items)
    }

    pub fn get_code_slice(&self) -> CodeSlice {
        // TODO : implement this for all the other types.
        match self {
            Statement::Token(ref token) => token.get_code_slice().clone(),
            Statement::Binary(_,s1,s2) => CodeSlice::create_from(&s1.get_code_slice(), &s2.get_code_slice()),
            _ => CodeSlice::empty(),
        }
    }

    pub fn as_string<'a>(&'a self) -> &'a str {
        match self {
            Statement::Token(ref token) => match token.get_type() {
                TokenType::String(string) => string,
                _ => panic!("Cannot unwrap {:?} as a string",self),
            },
            _ => panic!("Cannot unwrap {:?} as a string",self),
        }
    }
    pub fn as_name<'a>(&'a self) -> &'a str {
        match self {
            Statement::Token(ref token) => match token.get_type() {
                TokenType::Identifier(string) => string,
                _ => panic!("Cannot unwrap {:?} as a name",self),
            },
            _ => panic!("Cannot unwrap {:?} as a name",self),
        }
    }

    ///////////////////////////////////////////////////////////////////////
    /// IS CHECKS

    pub fn is_empty(&self) -> bool {
        match self {
            Statement::Empty => true,
            _ => false,
        }
    }

    pub fn is_token(&self,token:TokenType) -> bool {
        //! checks if is a token and of that type

        match self {
            Statement::Token(t) => t.get_type() == &token,
            _ => false,
        }
    }

    pub fn is_token_ref(&self,token : &TokenType) -> bool {
        match self {
            Statement::Token(ref t) => t.get_type() == token,
            _ => false
        }
    }
    
    pub fn is_bool(&self) -> bool {
        if self.is_token(TokenType::False) || self.is_token(TokenType::True) {
            true
        } else {
            false
        }
    }

    pub fn is_args(&self) -> bool {
        //!
        //! 
        //! ```text
        //! 
        //!     [x]     `(´ [explist] `)´ | 
        //!     [ ]     tableconstructor | 
        //!     [x]     String 
        //! 
        //! ```

        if self.is_string() { return true; }
        if self.is_exprlist() { return true; }

        false 
    }

    pub fn is_a_list(&self) -> bool {
        match self {
            Statement::VarList(_) |
            Statement::ExprList(_) |
            Statement::NameList(_) => true,
            _ => false,
        }
    }

    pub fn is_fieldlist(&self) -> bool {
        //! checks if technically a field list
        //!
        //! ```text
        //!
        //!     [ ]     field {fieldsep field} [fieldsep]
        //!
        //! ```
        
        // if a single item, could be a list of element 1
        if self.is_field() { return true; }
        // a field can be an expr, so an expression list is a type of 
        // valid fieldlist
        if self.is_exprlist() { return true; }

        match self {
            Statement::FieldList(_) => true,
            _ => false,
        }

    }

    pub fn is_exprlist(&self) -> bool {
        if self.is_varlist() { return true; }
        if self.is_expr() { return true; }

        match self {
            Statement::ExprList(_) => true,
            _ => false,
        }
    }

    pub fn is_namelist(&self) -> bool {
        match self {
            Statement::NameList(_) => true,
            _ => false,
        }
    }

    pub fn is_complex_var(&self) -> bool {
        match self {
            Statement::ComplexVar(_) => true,
            _ => false,
        }
    }

    pub fn is_varlist(&self) -> bool {
        //! a varlist is defined as 
        //! 
        //! ```text
        //! 
        //!     var {`,´ var}
        //! 
        //! ```
        //! 
        //! and a var is defined as
        //! 
        //! ```text
        //! 
        //!     Name | 
        //!     prefixexp `[´ exp `]´ | 
        //!     prefixexp `.´ Name 
        //! 
        //! ```
        //! 
        //! thus a list of names is a variable list as well.

        if self.is_namelist() { return true; }
        if self.is_var() { return true; }

        match self {
            Statement::VarList(_) => true,
            _ => false,
        }
    }

    pub fn is_a_token(&self) -> bool {
        //! checks if is a token and of that type

        match self {
            Statement::Token(t) => true,
            _ => false,
        }
    }

    pub fn is_prefixexp(&self) -> bool {
        //! checks if a prefixexpression
        //! 
        //! ```text
        //! 
        //!     [x]     var |
        //!     [x]     functioncall |
        //!     [x]     `(´ exp `)´
        //! 
        //! ```
        
        if self.is_expr() { return true; }
        if self.is_var() { return true; }
        
        match self {
            Statement::FunctionCall(_,_) => true,
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

    pub fn is_prefix(&self) -> bool {
        //! checking if an pre-expression
        //! 
        //! ```text
        //! 
        //!     [x]   var | 
        //!     [ ]   functioncall | 
        //!     [x]   `(´ exp `)´
        //! 
        //! ```
        
        if self.is_var() { return true; }
        
        match self {
            Statement::Group(_) => true,
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
        //!     [x]   prefixexp | 
        //!     [ ]   tableconstructor | 
        //!     [x]   exp binop exp | 
        //!     [x]   unop exp 
        //! 
        //! ```
    
        if self.is_prefix() { return true; }

        match self {
            Statement::Token(token) => match token.get_type() {
                TokenType::Nil | 
                TokenType::False | 
                TokenType::True |
                TokenType::Number(_) | 
                TokenType::String(_) => true,
                _ => false,
            },
            Statement::Group(_) |
            Statement::Binary(_,_,_) |
            Statement::ComplexVar(_) |
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
		//!     [x]   local namelist [`=´ explist] 
        //! 
        //! ```
        
        match self {
            Statement::DoEnd(_) |
            Statement::WhileDoEnd(_,_) |
            Statement::AssignmentLocal(_,_) |
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

    pub fn is_number(&self) -> bool {
        //! checking if its a number
        
        match self {
            Statement::Token(ref token) => match token.get_type() {
                TokenType::Number(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        //! checking if its a number
        
        match self {
            Statement::Token(ref token) => match token.get_type() {
                TokenType::String(_) => true,
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

    pub fn into_list(self) -> Statement {
        match self {
            Statement::VarList(_) |
            Statement::ExprList(_) |
            Statement::NameList(_) => self,
            statement => Statement::create_list(vec![Box::new(statement)]),
        }
    }

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
                // basically we are checking the order of operation here
                
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

    pub fn create_table(constructor : Statement) -> Statement {
        let parts = constructor.into_list();

        let mut map : HashMap<TableIndex,Statement> = HashMap::new();
        let mut index = 1;

        for p in parts.as_list() {
            match p {
                Statement::Assignment(namelist,exprlist) => {
                    let var = &namelist.as_namelist()[0];
                    let expr = exprlist.as_list_index(0).unwrap();

                    map.insert(
                        TableIndex::String(var.to_string()),
                        expr.clone()
                    );
                },
                part => {
                    map.insert(
                        TableIndex::Number(format!("{}",index)),
                        part.clone()
                    );
                    index += 1;
                }
            }
        }

        let table = Statement::Table(map);

        table
    }
    
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

    pub fn create_field(first : Statement, second :Statement) -> Option<Statement> {
        //! creates a field object based on what the statements are. Assumes
        //! that the syntax has been checked and results in the is call.
        //! 
        //! ```text
        //! 
        //!     field ::= `[´ exp `]´ `=´ exp | Name `=´ exp | exp
        //! 
        //! ```
        
        if first.is_name() && second.is_expr() {
            Some(Statement::FieldNamed(Box::new(first),Box::new(second)))
        } else if first.is_expr() && second.is_expr() {
            Some(Statement::FieldBracket(Box::new(first),Box::new(second)))
        } else {
            None
        }

    }

    

    pub fn create_list(mut items : Vec<Box<Statement>>) -> Statement {
        //! creates a list out of a vec of Statements
        //!
        //! ``` text
        //! 
        //!      [x]     namelist ::= Name {`,´ Name}
        //!      [x]     varlist ::= var {`,´ var}
        //!      [x]     explist ::= {exp `,´} exp
        //!      [ ]     fieldlist ::= field {fieldsep field} [fieldsep]
        //!      [ ]     parlist ::= namelist [`,´ `...´] | `...´
        //! 
        //! ```

        // check if we already have a list
        if items.len() == 1 {
            if items[0].is_a_list() {
                return *(items.remove(0));
            }
        }
        
        let mut names_count = 0;
        for i in items.iter() { if i.is_name(){ names_count +=1; }}
        if names_count == items.len() {
            let mut strings : Vec<String> = Vec::new();
            for i in items { strings.push(i.as_name().to_string()); }
            return Statement::NameList(strings);
        }

        let mut var_count = 0;
        for i in items.iter() { if i.is_var(){ var_count +=1; }}
        if var_count == items.len() {
            return Statement::VarList(items);
        }


        Statement::ExprList(items)
    }

    pub fn create_assignment(in_vars: Statement, in_exprs : Statement, local : bool) -> Statement {
        // gets the two lists the same length
       
        let mut vars = in_vars.into_list();
        let mut exprs = in_exprs.into_list();

        loop {
            if vars.len() == exprs.len() { break; }
            match vars.len() > exprs.len() {
                true => {
                    exprs.as_list_mut().push(Box::new(Statement::Empty));
                },
                false => {
                    //vars = vars.add_to_list(Statement::Empty).unwrap();
                    if vars.is_namelist() {
                        vars.as_namelist_mut().push("_".to_string());
                    } else {
                        vars.as_list_mut().push(Box::new(Statement::Empty));
                    }
                },
            }
        }
        
        if vars.len() != exprs.len() {
            panic!("Error creating assignment, varlist and expr list must be the same!");
        }
        
        match local {
            true => Statement::AssignmentLocal(Box::new(vars),Box::new(exprs)),
            false => Statement::Assignment(Box::new(vars),Box::new(exprs))
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

    pub fn explode_list(self) -> Vec<Box<Statement>> {
        match self {
            Statement::VarList(list) => list,
            Statement::ExprList(list) => list,
            Statement::ComplexVar(list) => list,
            Statement::NameList(_) => {
                let list = self.into_varlist();
                list.explode_list()
            },
            _=> panic!("can't explode {} as a list",self),
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

    fn render_strings(list : &Vec<String>) -> String {
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

    fn render_hashmap(hash : &HashMap<TableIndex,Statement>) -> String {
        let mut items = String::new();

        for (k,v) in hash {
            items = format!("{}{} : {}, ",items,k,v);
        }

        items
    }

    fn render_complex_var(list : &Vec<Box<Statement>>) -> String {
        let mut string = String::new();

        for i in 0 .. list.len() {
            if i == 0 {
                string = format!("{}",list[i]);
            } else {
                string = format!("{}[{}]",string,list[i]);
            }
        }

        string
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
            Statement::ExprList(list) => write!(f,"{}",Statement::render_list(&list)),
            Statement::VarList(list) => write!(f,"{}",Statement::render_list(&list)),
            Statement::NameList(list) => write!(f,"{}",Statement::render_strings(&list)),
            Statement::TableConstructor(list) => write!(f,"[ {} ]",Statement::render_list(&list)),
            Statement::Table(hash) => write!(f,"{{ {} }}",Statement::render_hashmap(&hash)),
            Statement::DoEnd(stats) => write!(f,"(do {} end)",Statement::render_statements(&stats)),
            Statement::WhileDoEnd(expr,stats) => write!(f,"(while {} do {} end)",expr,Statement::render_statements(&stats)),

            Statement::Assignment(varlist,exprlist) => write!(f,"(= {} {})",&varlist,&exprlist),
            Statement::AssignmentLocal(varlist,exprlist) => write!(f,"(= local {} {})",&varlist,&exprlist),
            
            Statement::FunctionCall(name,args) => write!(f,"(fncall<{}> {})",name,&args),
            Statement::Function(args,body) => write!(f,"(fn<{}> {} end)",Statement::render_strings(&args),Statement::render_list(&body)),
            Statement::FunctionNamed(name,args,body) => write!(f,"(fn {}<{}> {} end)",name,Statement::render_strings(&args),Statement::render_list(&body)),
            Statement::Return(list) => write!(f,"(return {})",&list),
            Statement::ComplexVar(access) => write!(f, "{}",Statement::render_complex_var(&access)),
            
            Statement::Group(insides) => write!(f,"{}",insides),

            Statement::Empty => write!(f,"nil"),
        }
    }
}

#[cfg(test)]
mod spec {

    use crate::test_crate::*;

    #[test]
    fn unop() {
        //! ```text
        //! 
        //!     unop ::= `-´ | not | `#´
        //! 
        //! ```
        
        for t in vec!["-", "not", "#"] {
            assert!(statement!(t).is_unop());
        }

        for t in vec!["+", "*"] {
            assert!(!statement!(t).is_unop());
        }

    }
    
    #[test]
    fn binop() {
        //! ```text
        //! 
        //!     binop ::= `+´ | `-´ | `*´ | `/´ | `^´ | `%´ | `..´ | 
		//!               `<´ | `<=´ | `>´ | `>=´ | `==´ | `~=´ | 
		//!               and | or
        //! 
        //! ```

        for t in vec![
            "+", "-", "*", "/", "^", "%",
            "..", "<", "<=", ">", ">=", "or", 
            "==", "~=", "and" 
        ] {
            assert!(statement!(t).is_binop());
        }

        for t in vec!["not", "#"] {
            assert!(!statement!(t).is_binop());
        }

    }
    
    #[test]
    fn fieldsep() {
        //! ```text
        //! 
        //!     fieldsep ::= `,´ | `;´
        //! 
        //! ```

        for t in vec![ ",", ";" ] {
            assert!(statement!(t).is_fieldsep());
        }

        for t in vec!["not", "#"] {
            assert!(!statement!(t).is_fieldsep());
        }
    }

    #[test]
    fn field() {
        //! ```text
        //! 
        //!     field ::= `[´ exp `]´ `=´ exp | 
        //!               Name `=´ exp | 
        //!               exp
        //! 
        //! ```

        use crate::elements::Statement;

        let one = Statement::create_field(statement!("2"),statement!("2"));
        assert!(one.is_some());
        assert!(one.unwrap().is_field());

        
        let two = Statement::create_field(statement!("linda"),statement!("2"));
        assert!(two.is_some());
        assert!(two.unwrap().is_field());

        
        let three = binary!("+","2","4");
        assert!(three.is_field());

    }
}
