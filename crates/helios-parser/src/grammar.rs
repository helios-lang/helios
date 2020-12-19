//! Module responsible for describing how to parse nodes.

use crate::parser::marker::CompletedMarker;
use crate::parser::Parser;
use helios_syntax::SyntaxKind;

mod expr;

pub(crate) fn root(parser: &mut Parser) -> CompletedMarker {
    let m = parser.start();
    expr::parse_expr(parser, 0);
    m.complete(parser, SyntaxKind::Root)
}
