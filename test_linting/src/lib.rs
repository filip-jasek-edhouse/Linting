#![feature(rustc_private)]
#![warn(unused_extern_crates)]

// A list of available compiler crates can be found here:
// https://doc.rust-lang.org/nightly/nightly-rustc/
extern crate rustc_arena;
extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_lexer;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;
extern crate rustc_parse;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_trait_selection;

use clippy_utils::diagnostics::span_lint;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::LateContext;
use rustc_lint::LateLintPass;

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Warns when a labeled `continue` is used (for example `continue 'epic`).
    ///
    /// ### Why is this bad?
    ///
    /// Labeled control-flow jumps are easy to miss in reviews and can make loops
    /// harder to reason about.
    ///
    /// ### Known problems
    ///
    /// This lint intentionally allows unlabeled `continue`.
    ///
    /// ### Example
    ///
    /// ```rust
    /// 'epic: loop {
    ///     if should_skip() {
    ///         continue 'epic;
    ///     }
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// loop {
    ///     if should_skip() {
    ///         continue;
    ///     }
    /// }
    /// ```
    pub TEST_LINTING,
    Warn,
    "warn on labeled continue usage"
}

impl<'tcx> LateLintPass<'tcx> for TestLinting {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::Continue(destination) = expr.kind
            && destination.label.is_some()
        {
            span_lint(
                cx,
                TEST_LINTING,
                expr.span,
                "signpost used: labeled continue (`continue 'label`)",
            );
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
