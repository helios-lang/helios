use helios_syntax::SyntaxKind;
use rowan::SmolStr;

#[derive(Debug, Clone)]
pub(super) enum Event {
    StartNode { kind: SyntaxKind },
    StartNodeAt { kind: SyntaxKind, checkpoint: usize },
    AddToken { kind: SyntaxKind, text: SmolStr },
    FinishNode,
}
