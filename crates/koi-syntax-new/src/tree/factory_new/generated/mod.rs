pub mod nodes;
pub mod tokens;

use super::*;
pub use nodes::*;
pub use tokens::*;
use paste::paste;

#[macro_export]
macro_rules! make_node_constructor {
    (
        $name:ident {
            $( $member:ident : $member_type:ty ),* $(,)?
        }
    ) => {
        paste! {
            #[derive(Clone, Debug, Default, Eq, PartialEq)]
            pub struct [<$name>] {
                pub(crate) parent: Option<NodeId>,
                $(
                    pub(crate) $member: Option<$member_type>,
                )*
            }

            impl [<$name>] {
                pub fn new(parent: Option<NodeId>) -> Self {
                    Self { parent, ..Self::default() }
                }

                $(
                    pub fn [<$member>]<F>(mut self, constructor: F) -> Self
                    where
                        F: FnOnce(Option<NodeId>) -> $member_type,
                    {
                        self.$member = Some(constructor(self.parent));
                        self
                    }
                )*
            }

            impl SyntaxFactory {
                pub fn [<make_ $name:snake>](
                    parent: Option<NodeId>
                ) -> [<$name:camel>] {
                    [<$name:camel>]::new(parent)
                }
            }
        }
    }
}

#[macro_export]
macro_rules! make_token_constructor {
    (
        $name:ident {
            $( $member:ident : $member_type:ty ),* $(,)?
        }
    ) => {
        paste! {
            #[derive(Clone, Debug, Default, Eq, PartialEq)]
            pub struct [<$name>] {
                pub(crate) parent: Option<NodeId>,
                pub(crate) start: usize,
                pub(crate) leading_trivia: SyntaxTriviaList,
                pub(crate) trailing_trivia: SyntaxTriviaList,
                $(
                    pub(crate) $member: $member_type,
                )*
            }

            impl [<$name>] {
                pub fn new(parent: Option<NodeId>) -> Self {
                    Self { parent, ..Self::default() }
                }

                $(
                    pub fn [<$member>](
                        mut self,
                        $member: $member_type
                    ) -> Self {
                        self.$member = $member;
                        self
                    }
                )*

                pub fn start(mut self, start: usize) -> Self {
                    self.start = start;
                    self
                }

                pub fn leading_trivia<S>(mut self, leading_trivia: S) -> Self
                where
                    S: Into<SyntaxTriviaList>,
                {
                    self.leading_trivia = leading_trivia.into();
                    self
                }

                pub fn trailing_trivia<S>(mut self, trailing_trivia: S) -> Self
                where
                    S: Into<SyntaxTriviaList>,
                {
                    self.trailing_trivia = trailing_trivia.into();
                    self
                }
            }

            impl SyntaxFactory {
                pub fn [<make_ $name:snake>](
                    parent: Option<NodeId>
                ) -> [<$name:camel>] {
                    [<$name:camel>]::new(parent)
                }
            }
        }
    }
}
