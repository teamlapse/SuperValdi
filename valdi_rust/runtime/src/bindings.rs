use valdi_rust_ir::ids::{BindingId, SourceSpanId};

use crate::{
    diagnostics::{RuntimeDiagnostic, RuntimeResult},
    expressions::{
        evaluate_expression, ExpressionContext, RuntimeExpression, RuntimePlatformConstants,
    },
    state::{StateStore, StateValue},
};

pub const RUNTIME_BINDING_MISSING: &str = "RUNTIME_BINDING_MISSING";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeBinding {
    pub binding_id: BindingId,
    pub expression: RuntimeExpression,
    pub source_span_id: Option<SourceSpanId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BindingResolution {
    pub binding_id: BindingId,
    pub value: StateValue,
    pub source_span_id: Option<SourceSpanId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BindingResolver {
    pub bindings: Vec<RuntimeBinding>,
    pub platform_constants: RuntimePlatformConstants,
}

impl BindingResolver {
    pub fn resolve(
        &self,
        binding_id: BindingId,
        state: &StateStore,
    ) -> RuntimeResult<BindingResolution> {
        let binding = self
            .bindings
            .iter()
            .find(|binding| binding.binding_id == binding_id)
            .ok_or_else(|| {
                RuntimeDiagnostic::error(
                    RUNTIME_BINDING_MISSING,
                    "$.bindings.binding_id",
                    format!("binding {} is missing", binding_id.as_str()),
                    None,
                )
            })?;
        let context = ExpressionContext {
            state,
            platform_constants: &self.platform_constants,
        };
        Ok(BindingResolution {
            binding_id,
            value: evaluate_expression(&context, &binding.expression)?,
            source_span_id: binding.source_span_id,
        })
    }

    pub fn resolve_all(&self, state: &StateStore) -> RuntimeResult<Vec<BindingResolution>> {
        self.bindings
            .iter()
            .map(|binding| self.resolve(binding.binding_id, state))
            .collect()
    }
}
