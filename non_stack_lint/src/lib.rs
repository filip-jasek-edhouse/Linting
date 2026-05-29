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
    /// Emits an error on common heap allocation entry points.
    ///
    /// ### Why is this bad?
    ///
    /// Heap allocations can increase latency, memory pressure, and runtime
    /// nondeterminism compared to stack-only code.
    ///
    /// ### Known problems
    ///
    /// This lint is conservative and currently targets common constructors
    /// (`Box::new`, `Vec::new/with_capacity`, `String::new/with_capacity`,
    /// `Rc::new`, `Arc::new`) plus `std::alloc` allocation functions.
    /// It does not detect every possible heap allocation route.
    ///
    /// ### Example
    ///
    /// ```rust
    /// let _x = Box::new(1u32);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// let x = 1u32;
    /// ```
    pub NON_STACK_LINT,
    Deny,
    "use of common non-stack memory allocation"
}

impl<'tcx> LateLintPass<'tcx> for NonStackLint {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
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

        if let Some(def_id) = called {
            let path = cx.tcx.def_path_str(def_id);

            let is_box_new = (path.contains("alloc::boxed::Box") || path.contains("std::boxed::Box"))
                && path.ends_with("::new");
            let is_vec_ctor = (path.contains("alloc::vec::Vec") || path.contains("std::vec::Vec"))
                && (path.ends_with("::new") || path.ends_with("::with_capacity"));
            let is_string_ctor =
                (path.contains("alloc::string::String") || path.contains("std::string::String"))
                && (path.ends_with("::new") || path.ends_with("::with_capacity"));
            let is_rc_new = (path.contains("alloc::rc::Rc") || path.contains("std::rc::Rc"))
                && path.ends_with("::new");
            let is_arc_new = (path.contains("alloc::sync::Arc") || path.contains("std::sync::Arc"))
                && path.ends_with("::new");
            let is_std_alloc_fn = path.ends_with("::alloc::alloc") || path.ends_with("::alloc::alloc_zeroed");

            let is_heap_entrypoint =
                is_box_new || is_vec_ctor || is_string_ctor || is_rc_new || is_arc_new || is_std_alloc_fn;

            if is_heap_entrypoint {
                span_lint(
                    cx,
                    NON_STACK_LINT,
                    expr.span,
                    "non-stack memory allocation detected",
                );
            }
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
