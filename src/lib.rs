//! Pure language-neutral dependency evaluation over admitted generated model values.
pub use codegate_behavior::dependency as model;
pub use codegate_semantic_behavior::semantic as semantic_model;
mod admit;
mod analysis;
mod check;
pub mod semantic_wire;
pub mod source_identity;
pub mod wire;
use model::*;

/// The direct-response implementation. It reads no files, tools, clock or environment.
pub fn evaluate(input: Evaluate) -> EvaluateResponse {
    let mut policy = input.policy.clone();
    policy.forbidden.sort_by_key(|r| {
        (
            r.source.0.clone(),
            r.target.0.clone(),
            format!("{:?}", r.kind),
        )
    });
    let mut report = Evaluation {
        format: "codegate-dependency-report/0.1".into(),
        source_id: input.snapshot.source_id.clone(),
        configuration_id: input.snapshot.configuration_id.clone(),
        policy,
        metric_id: "dependency.unique-fan-out/1".into(),
        checker_id: "dependency.forbidden-edge/1".into(),
        coverage: input.snapshot.coverage.status,
        verdict: Verdict::Error,
        fan_out: vec![],
        findings: vec![],
        diagnostics: vec![],
    };
    match admit::admit(&input) {
        Err(diagnostics) => report.diagnostics = diagnostics,
        Ok(graph) => {
            report.fan_out = analysis::fan_out(&graph);
            report.findings = check::forbidden(&graph);
            let (verdict, diagnostic) = match report.coverage {
                CoverageStatus::Complete => (
                    if report.findings.is_empty() {
                        Verdict::Pass
                    } else {
                        Verdict::Fail
                    },
                    None,
                ),
                CoverageStatus::Partial => (
                    if report.findings.is_empty() {
                        Verdict::Incomplete
                    } else {
                        Verdict::Fail
                    },
                    Some(DiagnosticCode::PartialCoverage),
                ),
                CoverageStatus::Unsupported => (
                    Verdict::Unsupported,
                    Some(DiagnosticCode::UnsupportedCoverage),
                ),
                CoverageStatus::Failed => (Verdict::Error, Some(DiagnosticCode::FailedCoverage)),
            };
            report.verdict = verdict;
            if let Some(code) = diagnostic {
                report.diagnostics.push(Diagnostic {
                    code,
                    subject: "coverage".into(),
                });
            }
        }
    }
    EvaluateResponse { report }
}

/// The generated seam returns an outcome only; this adapter retains that call's typed response.
#[derive(Default)]
pub struct Evaluator {
    response: Option<EvaluateResponse>,
}
impl Evaluator {
    pub fn take_response(&mut self) -> Option<EvaluateResponse> {
        self.response.take()
    }
}
impl model::obligations::EvaluateBehavior for Evaluator {
    fn evaluate(
        &mut self,
        input: Evaluate,
    ) -> Result<EvaluateOutcome, codegate_behavior::obligation::UnmetObligation> {
        self.response = Some(evaluate(input));
        Ok(EvaluateOutcome::Evaluated)
    }
}
pub fn exit_code(report: &Evaluation) -> u8 {
    match (report.coverage, report.verdict) {
        (CoverageStatus::Complete, Verdict::Pass) => 0,
        (CoverageStatus::Complete, Verdict::Fail) => 1,
        _ => 2,
    }
}
