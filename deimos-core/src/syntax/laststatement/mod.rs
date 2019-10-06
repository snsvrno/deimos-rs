use crate::syntax::SyntaxElement;
use crate::codewrap::CodeWrap;
use crate::token::Token;

pub fn process(phrase : &mut Vec<CodeWrap<SyntaxElement>>) -> bool {
    //! works through the elements and checks if any of the following 
    //! are matched, returns true if it reduces something
    //!
    //! [ ] return [explist]
    //! [ ] break

    let mut count = 0;
    loop {
        // the intent of this loop is to work our way down the list,
        // and when we actually process one then we go back to the top
        // of the list and start over, if we go done the list and dont
        // process anyhting then we just leave
        
        count += 1;

        if statement_break(phrase) { continue; }
        if statement_return(phrase) { continue; }

        break;
    }

    if count > 1 { true } 
    else { false }
}

fn statement_break(phrase : &mut Vec<CodeWrap<SyntaxElement>>) -> bool {
    //! finds a break token and makes it a syntaxelement
     
    match phrase.len() {
        1 => match phrase[0].item() {
            SyntaxElement::Token(Token::Break) => { 
                
                let CodeWrap::CodeWrap(_, s, e) = phrase.remove(0);   // the token 'break'
                let new_element = CodeWrap::CodeWrap(SyntaxElement::StatementLastBreak,s,e);
                phrase.insert(0,new_element);
                true
            },
            _ => false,
        },
        _ => false,
    }
}

fn statement_return(phrase : &mut Vec<CodeWrap<SyntaxElement>>) -> bool {
    //! finds a return token and makes it a syntaxelement
     
    match phrase.len() {
        1 => match phrase[0].item() {
            SyntaxElement::Token(Token::Return) => { 
                let CodeWrap::CodeWrap(_, s, e) = phrase.remove(0);   // the token 'return'

                let new_element = CodeWrap::CodeWrap(SyntaxElement::StatementLastReturn(None),s,e);
                phrase.insert(0,new_element);
                true
            },
            _ => false,
        },
        2 => match phrase[0].item() {
            SyntaxElement::Token(Token::Return) => { 
                let CodeWrap::CodeWrap(_, s, _) = phrase.remove(0); // the token 'return'
                let CodeWrap::CodeWrap(explist, _, e) = phrase.remove(0); // the explist

                let new_element = CodeWrap::CodeWrap(
                    SyntaxElement::StatementLastReturn(Some(Box::new(explist))),s,e);
                phrase.insert(0,new_element);
                true
            },
            _ => false,
        },
        _ => false,
    }
}