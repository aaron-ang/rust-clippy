use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::source::SpanRangeExt;
use clippy_utils::{expr_or_init, is_expr_const_aligned, std_or_core};
use rustc_errors::Applicability;
use rustc_hir::{Expr, Mutability, Ty, TyKind};
use rustc_lint::LateContext;

use super::DANGLING_PTR;

pub(super) fn check(cx: &LateContext<'_>, expr: &Expr<'_>, from: &Expr<'_>, to: &Ty<'_>) {
    if let TyKind::Ptr(ref ptr_ty) = to.kind {
        let init_expr = expr_or_init(cx, from);
        if is_expr_const_aligned(cx, init_expr, ptr_ty.ty)
            && let Some(std_or_core) = std_or_core(cx)
        {
            let sugg_fn = match ptr_ty.mutbl {
                Mutability::Not => "ptr::dangling",
                Mutability::Mut => "ptr::dangling_mut",
            };

            let sugg = if let TyKind::Infer(()) = ptr_ty.ty.kind {
                format!("{std_or_core}::{sugg_fn}()")
            } else if let Some(mut_ty_snip) = ptr_ty.ty.span.get_source_text(cx) {
                format!("{std_or_core}::{sugg_fn}::<{mut_ty_snip}>()")
            } else {
                return;
            };

            span_lint_and_sugg(
                cx,
                DANGLING_PTR,
                expr.span,
                "manual creation of a dangling pointer",
                "try",
                sugg,
                Applicability::MachineApplicable,
            );
        }
    }
}
