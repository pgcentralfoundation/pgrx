//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use crate::{
    ActiveMacro, ActiveProvenance, AnalysisStatus, BuildInputs, ExpansionBatch, ExpansionLimits,
    ExpansionResult, ExpansionSkipCode, FrontendError, FrontendOutput, IntegerConstant,
    MacroAnalysis, MacroDependency, MacroScanner, SkipReason, SkipReasonCode,
};
use std::collections::BTreeMap;

/// An inspected C environment and compiler-expanded symbolic invocations resolved together.
///
/// Preparation uses bounded compiler passes for the whole batch. The input files and
/// environment must remain stable through preparation; changes require a new inspection.
/// Emission accepts this session rather than independently assembled syntax or type facts.
pub struct AnalysisSession<'a> {
    frontend: &'a FrontendOutput,
    expansions: ExpansionBatch,
    integer_constants: BTreeMap<String, IntegerConstant>,
}

impl<'a> AnalysisSession<'a> {
    pub fn prepare(
        scanner: &MacroScanner,
        frontend: &'a FrontendOutput,
        names: &[impl AsRef<str>],
    ) -> Result<Self, FrontendError> {
        Self::prepare_with_limits(scanner, frontend, names, ExpansionLimits::default())
    }

    pub fn prepare_with_limits(
        scanner: &MacroScanner,
        frontend: &'a FrontendOutput,
        names: &[impl AsRef<str>],
        limits: ExpansionLimits,
    ) -> Result<Self, FrontendError> {
        let mut expansions =
            crate::expansion::prepare_expansions_with_limits(scanner, frontend, names, limits)?;
        let integer_constants =
            crate::expansion::retain_integer_constants(scanner, frontend, &mut expansions, limits)?;
        Ok(Self { frontend, expansions, integer_constants })
    }

    pub fn frontend(&self) -> &FrontendOutput {
        self.frontend
    }

    pub fn expansions(&self) -> &ExpansionBatch {
        &self.expansions
    }

    /// Referenced object macros whose integer types and values were resolved by Clang.
    /// These facts are independent of the Rust binding representation and include
    /// constants that had to remain expanded because their C grouping was not atomic.
    pub fn integer_constants(&self) -> &BTreeMap<String, IntegerConstant> {
        &self.integer_constants
    }

    pub fn inputs(&self) -> &BuildInputs {
        &self.expansions.inputs
    }

    /// Check that a later generation step consumed the same inspected environment.
    /// Changes require a fresh inspection and preparation, including C value probes.
    pub fn verify_inputs(&self) -> Result<(), FrontendError> {
        crate::expansion::verify_environment(self.frontend)?;
        crate::frontend::verify_input_files(self.inputs())
    }

    pub fn analyze(&self, name: &str) -> MacroAnalysis {
        match self.expansions.results.get(name) {
            Some(ExpansionResult::Expanded { expansion }) => {
                // Marker names stay distinct from captured identifiers introduced by
                // nested expansion, even when those identifiers match a formal's name.
                let active = ActiveMacro {
                    definition: expansion.definition.clone(),
                    provenance: ActiveProvenance::Resolved,
                };
                let mut analysis = crate::analysis::analyze_active_with_constants(
                    self.frontend,
                    name,
                    Some(&active),
                    &self.integer_constants,
                );
                for (parameter, original_name) in
                    analysis.parameters.iter_mut().zip(&expansion.parameters)
                {
                    parameter.name.clone_from(original_name);
                }
                analysis.dependencies = expansion
                    .dependencies
                    .iter()
                    .map(|dependency| MacroDependency {
                        name: dependency.name.clone(),
                        kind: dependency.kind,
                        provenance: dependency.provenance.clone(),
                        // Dependency spans identify possible source origins. The compiler
                        // token stream does not provide an exact per-token origin map.
                        uses: Vec::new(),
                    })
                    .collect();
                analysis
            }
            Some(ExpansionResult::Skipped { reason }) => {
                let mut analysis = crate::analyze(self.frontend, name);
                analysis.expression = None;
                analysis.status = AnalysisStatus::Skipped {
                    reason: SkipReason {
                        code: match reason.code {
                            ExpansionSkipCode::NotActive => SkipReasonCode::NotActive,
                            ExpansionSkipCode::NotFunctionLike => SkipReasonCode::NotFunctionLike,
                            ExpansionSkipCode::MalformedParameters => {
                                SkipReasonCode::MalformedParameters
                            }
                            ExpansionSkipCode::Variadic => SkipReasonCode::Variadic,
                            ExpansionSkipCode::TokenPaste => SkipReasonCode::TokenPaste,
                            ExpansionSkipCode::Stringification => SkipReasonCode::Stringification,
                            ExpansionSkipCode::DynamicBuiltin => SkipReasonCode::DynamicBuiltin,
                            ExpansionSkipCode::ProvenanceAmbiguous => {
                                SkipReasonCode::ProvenanceAmbiguous
                            }
                            ExpansionSkipCode::ProvenanceUnresolved => {
                                SkipReasonCode::ProvenanceUnresolved
                            }
                            ExpansionSkipCode::BudgetExceeded => SkipReasonCode::BudgetExceeded,
                            ExpansionSkipCode::CompilerRejected => SkipReasonCode::CompilerRejected,
                            ExpansionSkipCode::UnrecognizedOutput => {
                                SkipReasonCode::UnrecognizedExpansion
                            }
                        },
                        message: reason.message.clone(),
                        tokens: None,
                        spans: reason.spans.clone(),
                    },
                };
                analysis
            }
            None => {
                let mut analysis = crate::analyze(self.frontend, name);
                analysis.expression = None;
                analysis.status = AnalysisStatus::Skipped {
                    reason: SkipReason {
                        code: SkipReasonCode::ExpansionRequired,
                        message: "macro was not prepared in this analysis session".into(),
                        tokens: None,
                        spans: analysis.provenance.iter().cloned().collect(),
                    },
                };
                analysis
            }
        }
    }
}
