use super::event::Event;
use super::Parser;
use drop_bomb::DropBomb;
use helios_syntax::SyntaxKind;

pub(super) struct Marker {
    pos: usize,
    bomb: DropBomb,
}

impl Marker {
    pub(super) fn new(pos: usize) -> Self {
        Self {
            pos,
            bomb: DropBomb::new("Marker is not completed before being dropped"),
        }
    }

    pub(super) fn complete(
        mut self,
        parser: &mut Parser,
        kind: SyntaxKind,
    ) -> CompletedMarker {
        self.bomb.defuse();

        let event_at_pos = &mut parser.events[self.pos];
        assert_eq!(*event_at_pos, Event::Placeholder);

        *event_at_pos = Event::StartNode {
            kind,
            forward_parent: None,
        };

        parser.events.push(Event::FinishNode);

        CompletedMarker { pos: self.pos }
    }
}

pub(super) struct CompletedMarker {
    pos: usize,
}

impl CompletedMarker {
    pub(super) fn precede(self, parser: &mut Parser) -> Marker {
        let new_m = parser.start();

        if let Event::StartNode {
            ref mut forward_parent,
            ..
        } = parser.events[self.pos]
        {
            *forward_parent = Some(new_m.pos - self.pos);
        } else {
            unreachable!();
        }

        new_m
    }
}
