pub mod lexer;
pub mod token;

use crate::source::Source;
use lexer::Lexer;
use token::Token;

pub type Ast = Vec<Token>;

pub fn parse<'a>(source: Source<'a>) -> Ast {
    let mut queue = Ast::new();
    let mut lexer = Lexer::with(source);

    loop {
        match lexer.next_token() {
            Some(token) => queue.push(token),
            None => {
                // eprintln!("Done!");
                break
            }
        }
    }

    queue
}
