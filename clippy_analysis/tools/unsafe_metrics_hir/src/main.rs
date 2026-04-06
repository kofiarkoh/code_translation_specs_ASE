#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_span;

use clap::Parser;
use rustc_driver::{Callbacks, Compilation};
use rustc_hir as hir;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::intravisit::{self, Visitor};
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;
use rustc_span::Span;
use serde::Serialize;
use std::collections::BTreeSet;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    crate_root: PathBuf,

    #[arg(long)]
    output_json: PathBuf,
}

#[derive(Default, Serialize)]
struct Metrics {
    unsafe_blocks: usize,
    unsafe_lines: usize,
    type_casts: usize,
    unsafe_calls: usize,
}

struct MetricVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    unsafe_line_set: BTreeSet<usize>,
    metrics: Metrics,
}

impl<'tcx> MetricVisitor<'tcx> {
    fn add_span_lines(&mut self, span: Span) {
        let sm = self.tcx.sess.source_map();

        if let Ok(file_lines) = sm.span_to_lines(span) {
            for line_info in file_lines.lines {
                self.unsafe_line_set.insert(line_info.line_index + 1);
            }
        }
    }

    fn callee_is_unsafe_fn(&self, expr: &'tcx hir::Expr<'tcx>) -> bool {
        let typeck = self.tcx.typeck(expr.hir_id.owner.def_id);

        match &expr.kind {
            hir::ExprKind::MethodCall(..) => {
                if let Some(def_id) = typeck.type_dependent_def_id(expr.hir_id) {
                    let sig = self.tcx.fn_sig(def_id).skip_binder();
                    return sig.safety().is_unsafe();
                }
                false
            }

            hir::ExprKind::Path(qpath) => {
                let res = typeck.qpath_res(qpath, expr.hir_id);

                match res {
                    Res::Def(
                        DefKind::Fn
                        | DefKind::AssocFn
                        | DefKind::Ctor(..),
                        def_id,
                    ) => {
                        let sig = self.tcx.fn_sig(def_id).skip_binder();
                        sig.safety().is_unsafe()
                    }
                    _ => false,
                }
            }

            _ => false,
        }
    }
}

impl<'tcx> Visitor<'tcx> for MetricVisitor<'tcx> {
    fn visit_expr(&mut self, expr: &'tcx hir::Expr<'tcx>) {
        match &expr.kind {
            hir::ExprKind::Cast(..) => {
                self.metrics.type_casts += 1;
            }

            hir::ExprKind::Call(callee, _args) => {
                if self.callee_is_unsafe_fn(callee) {
                    self.metrics.unsafe_calls += 1;
                }
            }

            hir::ExprKind::MethodCall(..) => {
                if self.callee_is_unsafe_fn(expr) {
                    self.metrics.unsafe_calls += 1;
                }
            }

            _ => {}
        }

        intravisit::walk_expr(self, expr);
    }

    fn visit_block(&mut self, block: &'tcx hir::Block<'tcx>) {
        let is_unsafe = matches!(block.rules, hir::BlockCheckMode::UnsafeBlock(_));

        if is_unsafe {
            self.metrics.unsafe_blocks += 1;
            self.add_span_lines(block.span);
        }

        intravisit::walk_block(self, block);
    }
}

struct AnalyzerCallbacks {
    output_json: PathBuf,
}

impl Callbacks for AnalyzerCallbacks {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> Compilation {
        let mut visitor = MetricVisitor {
            tcx,
            unsafe_line_set: BTreeSet::new(),
            metrics: Metrics::default(),
        };

        for owner_id in tcx.hir_body_owners() {
            let body = tcx.hir_body_owned_by(owner_id);
            visitor.visit_body(body);
        }

        visitor.metrics.unsafe_lines = visitor.unsafe_line_set.len();

        std::fs::write(
            &self.output_json,
            serde_json::to_string_pretty(&visitor.metrics).unwrap(),
        )
        .unwrap();

        Compilation::Stop
    }
}

fn main() {
    let args = Args::parse();

    let crate_root = args.crate_root.canonicalize().unwrap();
    let output_json = args.output_json;

    let callbacks = &mut AnalyzerCallbacks { output_json };

    let rustc_args = vec![
        "rustc".to_string(),
        crate_root.to_string_lossy().to_string(),
        "--crate-name".to_string(),
        "unsafe_metrics_target".to_string(),
        "--edition".to_string(),
        "2021".to_string(),
    ];

    rustc_driver::run_compiler(&rustc_args, callbacks);
}