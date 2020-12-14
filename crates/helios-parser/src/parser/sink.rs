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
        let mut reordered_events = self.events.clone();

        for (idx, event) in self.events.iter().enumerate() {
            if let Event::StartNodeAt { kind, checkpoint } = event {
                reordered_events.remove(idx);
                reordered_events
                    .insert(*checkpoint, Event::StartNode { kind: *kind });
            }
        }

        for event in reordered_events {
            match event {
                Event::StartNode { kind } => {
                    self.builder.start_node(HeliosLanguage::kind_to_raw(kind))
                }
                Event::AddToken { kind, text } => self.token(kind, text),
                Event::FinishNode => self.builder.finish_node(),
                _ => unreachable!("StartNodeAt"),
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
