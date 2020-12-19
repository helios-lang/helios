use super::event::Event;
use crate::lexer::Token;
use helios_syntax::Language as HeliosLanguage;
use rowan::{GreenNode, GreenNodeBuilder, Language};

pub struct Sink<'tokens, 'source> {
    tokens: &'tokens [Token<'source>],
    events: Vec<Event>,
    builder: GreenNodeBuilder<'static>,
    cursor: usize,
}

impl<'tokens, 'source> Sink<'tokens, 'source> {
    pub fn new(tokens: &'tokens [Token<'source>], events: Vec<Event>) -> Self {
        Self {
            tokens,
            events,
            builder: GreenNodeBuilder::new(),
            cursor: 0,
        }
    }

    pub fn finish(mut self) -> GreenNode {
        use std::mem;

        for i in 0..self.events.len() {
            match mem::replace(&mut self.events[i], Event::Placeholder) {
                Event::StartNode {
                    kind,
                    forward_parent,
                } => {
                    let mut kinds = vec![kind];
                    let mut idx = i;
                    let mut forward_parent = forward_parent;

                    // Walk through the forward parent of the forward parent,
                    // and its forward parent, and so on until we reach a
                    // `StartNode` event without a forward parent.
                    while let Some(fp) = forward_parent {
                        idx += fp;
                        forward_parent = if let Event::StartNode {
                            kind,
                            forward_parent,
                        } = mem::replace(
                            &mut self.events[idx],
                            Event::Placeholder,
                        ) {
                            kinds.push(kind);
                            forward_parent
                        } else {
                            unreachable!()
                        };
                    }

                    for kind in kinds.into_iter().rev() {
                        self.builder
                            .start_node(HeliosLanguage::kind_to_raw(kind));
                    }
                }
                Event::AddToken => self.token(),
                Event::FinishNode => self.builder.finish_node(),
                Event::Placeholder => {}
            }

            self.eat_trivia();
        }

        self.builder.finish()
    }

    fn eat_trivia(&mut self) {
        while let Some(token) = self.tokens.get(self.cursor) {
            if !token.kind.is_trivia() {
                break;
            }

            self.token();
        }
    }

    fn token(&mut self) {
        let Token { kind, text } = self.tokens[self.cursor];
        self.builder
            .token(HeliosLanguage::kind_to_raw(kind), text.into());
        self.cursor += 1;
    }
}
