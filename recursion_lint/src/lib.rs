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
use rustc_hir::def::Res;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::LateContext;
use rustc_lint::LateLintPass;

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Emits an error when a function or method directly calls itself.
    ///
    /// ### Why is this bad?
    ///
    /// Direct recursion can cause stack overflows and is often accidental.
    ///
    /// ### Known problems
    ///
    /// This lint currently focuses on direct recursion (a function calling
    /// itself). It does not detect mutual recursion across multiple functions.
    ///
    /// ### Example
    ///
    /// ```rust
    /// fn factorial(n: u32) -> u32 {
    ///     if n <= 1 { 1 } else { n * factorial(n - 1) }
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// fn factorial(mut n: u32) -> u32 {
    ///     let mut acc = 1;
    ///     while n > 1 {
    ///         acc *= n;
    ///         n -= 1;
    ///     }
    ///     acc
    /// }
    /// ```
    pub RECURSION_LINT,
    Deny,
    "use of direct recursion"
}

impl<'tcx> LateLintPass<'tcx> for RecursionLint {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        let current_owner = cx.tcx.hir_get_parent_item(expr.hir_id).def_id.to_def_id();

        let called = match expr.kind {
            ExprKind::Call(callee, _) => {
                if let ExprKind::Path(qpath) = callee.kind {
                    match cx.qpath_res(&qpath, callee.hir_id) {
                        Res::Def(_, def_id) => Some(def_id),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            ExprKind::MethodCall(..) => cx.typeck_results().type_dependent_def_id(expr.hir_id),
            _ => None,
        };

        if called == Some(current_owner) {
            span_lint(cx, RECURSION_LINT, expr.span, "recursive call detected");
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
