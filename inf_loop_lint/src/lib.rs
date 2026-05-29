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
use rustc_hir::{Expr, ExprKind, LoopSource};
use rustc_lint::LateContext;
use rustc_lint::LateLintPass;

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Emits an error for `loop { ... }` expressions.
    ///
    /// ### Why is this bad?
    ///
    /// Unbounded loops are often accidental and can hang a program.
    ///
    /// ### Known problems
    ///
    /// This lint only targets the `loop` keyword and does not flag `while` or
    /// `for` loops.
    ///
    /// ### Example
    ///
    /// ```rust
    /// loop {
    ///     do_work();
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// while should_continue() {
    ///     do_work();
    /// }
    /// ```
    pub INF_LOOP_LINT,
    Deny,
    "use of `loop` expressions"
}

impl<'tcx> LateLintPass<'tcx> for InfLoopLint {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::Loop(_, _, source, _) = expr.kind
            && matches!(source, LoopSource::Loop)
        {
            span_lint(
                cx,
                INF_LOOP_LINT,
                expr.span,
                "unbounded `loop` expression detected",
            );
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
