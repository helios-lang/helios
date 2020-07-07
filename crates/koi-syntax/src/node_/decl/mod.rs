use crate::source::Span;
use std::fmt::Debug;

pub mod block_decl;
pub mod function_decl;
pub mod module_decl;
pub mod type_decl;

pub trait DeclarationNode: Debug {
    fn span(&self) -> Span;
}

#[cfg(test)]
mod tests {
    use crate::source::Position;
    use crate::token::*;
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_module_decl_node() {
        // module M =
        //     type T =
        //         def foo() = 0
        let module = module_decl::ModuleDeclarationNode {
            module_keyword: Token::with(
                TokenKind::Keyword(Keyword::Module),
                Span::new(
                    Position::new(0, 0, 0),
                    Position::new(0, 6, 6),
                )
            ),
            identifier: Token::with(
                TokenKind::Identifier,
                Span::new(
                    Position::new(0, 7, 7),
                    Position::new(0, 8, 8),
                )
            ),
            equal_symbol: Token::with(
                TokenKind::Keyword(Keyword::Module),
                Span::new(
                    Position::new(0, 9, 9),
                    Position::new(0, 10, 10),
                )
            ),
            decl_block: Arc::new(block_decl::BlockDeclarationNode {
                begin_token: Token::with(
                    TokenKind::Begin,
                    Span::new(
                        Position::new(0, 11, 11),
                        Position::new(1, 4, 4),
                    )
                ),
                declaration_list: vec![
                    Arc::new(
                        type_decl::TypeDeclarationNode {
                            type_keyword: Token::with(
                                TokenKind::Keyword(Keyword::Type),
                                Span::new(
                                    Position::new(1, 4, 4),
                                    Position::new(1, 8, 8),
                                )
                            ),
                            identifier: Token::with(
                                TokenKind::Identifier,
                                Span::new(
                                    Position::new(1, 9, 9),
                                    Position::new(1, 10, 10),
                                )
                            ),
                            equal_symbol: Token::with(
                                TokenKind::Symbol(Symbol::Eq),
                                Span::new(
                                    Position::new(1, 11, 11),
                                    Position::new(1, 12, 12),
                                )
                            ),
                            decl_block: Arc::new(function_decl::FunctionDeclarationNode),
                        }
                    ),
                ],
                end_token: Token::with(
                    TokenKind::Keyword(Keyword::Module),
                    Span::new(
                        Position::new(2, 21, 21),
                        Position::new(2, 21, 21),
                    )
                ),
            }),
        };

        println!("{}", module.span());
        println!("{:#?}", module);
    }
}
