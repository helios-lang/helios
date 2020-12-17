use super::event::Event;
use crate::lexer::Lexeme;
use helios_syntax::{Language as HeliosLanguage, SyntaxKind};
use rowan::{GreenNode, GreenNodeBuilder, Language, SmolStr};

pub(super) struct Sink<'lexemes, 'source> {
    builder: GreenNodeBuilder<'static>,
    lexemes: &'lexemes [Lexeme<'source>],
    cursor: usize,
    events: Vec<Event>,
}

impl<'lexemes, 'source> Sink<'lexemes, 'source> {
    pub(super) fn new(
        lexemes: &'lexemes [Lexeme<'source>],
        events: Vec<Event>,
    ) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            lexemes,
            cursor: 0,
            events,
        }
    }

    pub(super) fn finish(mut self) -> GreenNode {
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
                Event::AddToken { kind, text } => self.token(kind, text),
                Event::FinishNode => self.builder.finish_node(),
                Event::Placeholder => {}
            }

            self.eat_trivia();
        }

        self.builder.finish()
    }

    fn token(&mut self, kind: SyntaxKind, text: SmolStr) {
        self.builder.token(HeliosLanguage::kind_to_raw(kind), text);
        self.cursor += 1;
    }

    fn eat_trivia(&mut self) {
        while let Some(lexeme) = self.lexemes.get(self.cursor) {
            if !lexeme.kind.is_trivia() {
                break;
            }

            self.token(lexeme.kind, lexeme.text.into());
        }
    }
}
