use helios_syntax::SyntaxKind;
use rowan::SmolStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum Event {
    StartNode {
        kind: SyntaxKind,
        forward_parent: Option<usize>,
    },
    AddToken {
        kind: SyntaxKind,
        text: SmolStr,
    },
    FinishNode,
    Placeholder,
}
