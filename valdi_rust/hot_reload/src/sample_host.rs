use valdi_rust_backend::{BackendCapabilitySet, BackendTarget, MockBackend, RenderBackend};
use valdi_rust_ir::ids::StateId;
use valdi_rust_runtime::{
    sample_action_scheduler, sample_binding_resolver, sample_state_store, ActionScheduler,
    BindingResolver, RuntimeDocument, StateStore,
};

use crate::{
    declarative_parser::parse_declarative_patch,
    diagnostics::HotReloadResult,
    fixtures::{
        sample_runtime_document, HOT_RELOAD_SAMPLE_SESSION_ID, HOT_RELOAD_TRACE_FIXTURE_ID,
    },
    patch::HotReloadEdit,
    patch_generator::{
        generate_patch, validate_required_patch_family_coverage, GeneratedHotReloadPatch,
    },
    receiver::AppHotReloadReceiver,
    registries::{
        sample_module_registry, sample_native_view_registry, MockModuleRegistry,
        MockNativeViewRegistry,
    },
    transport::InMemoryHotReloadTransport,
    validator::{validate_patch_intent, HotReloadValidationContext},
    HotReloadDiagnostic,
};

#[derive(Debug)]
pub struct HotReloadSampleHost {
    session_id: &'static str,
    document: RuntimeDocument,
    state: StateStore,
    bindings: BindingResolver,
    scheduler: ActionScheduler,
    modules: MockModuleRegistry,
    native_views: MockNativeViewRegistry,
    backend: MockBackend,
    transport: InMemoryHotReloadTransport,
    receiver: AppHotReloadReceiver,
    trace: Vec<String>,
    full_rebuilds: u32,
}

impl HotReloadSampleHost {
    pub fn new() -> HotReloadResult<Self> {
        Ok(Self {
            session_id: HOT_RELOAD_SAMPLE_SESSION_ID,
            document: sample_runtime_document()?,
            state: sample_state_store().map_err(runtime_error)?,
            bindings: sample_binding_resolver(),
            scheduler: sample_action_scheduler().map_err(runtime_error)?,
            modules: sample_module_registry(),
            native_views: sample_native_view_registry(),
            backend: MockBackend::new(BackendCapabilitySet::all_for(BackendTarget::RustHost)),
            transport: InMemoryHotReloadTransport::new(),
            receiver: AppHotReloadReceiver::new(HOT_RELOAD_SAMPLE_SESSION_ID),
            trace: vec![
                "hot_reload_trace_v1".to_string(),
                format!("fixture={HOT_RELOAD_TRACE_FIXTURE_ID}"),
                format!("session={HOT_RELOAD_SAMPLE_SESSION_ID}"),
            ],
            full_rebuilds: 0,
        })
    }

    pub fn apply_declarative(
        &mut self,
        input: &'static str,
    ) -> HotReloadResult<GeneratedHotReloadPatch> {
        let intent = parse_declarative_patch(input)?;
        let before_state = self.state_token()?;
        validate_patch_intent(
            &intent,
            &HotReloadValidationContext {
                document: &self.document,
                state: &self.state,
                scheduler: &self.scheduler,
                modules: &self.modules,
                native_views: &self.native_views,
            },
        )?;
        let generated = generate_patch(&intent);
        for operation in generated.operations.iter().copied() {
            self.backend.apply(operation).map_err(|diagnostic| {
                HotReloadDiagnostic::rebuild_required(
                    "HOT_RELOAD_BACKEND_APPLY_FAILED",
                    diagnostic.path,
                    "mock backend rejected generated patch operation",
                    diagnostic.message,
                    intent.source_span_id,
                )
            })?;
        }
        let message = self.transport.publish(self.session_id, &generated);
        let received = self.receiver.receive(message);
        self.apply_host_side_metadata(&generated);
        let after_state = self.state_token()?;
        self.trace.push(format!(
            "patch={} family={} compatibility={:?} received={} state_preserved={} rebuilds={} operations={}",
            generated.intent.patch.identity.id.as_str(),
            generated.intent.family().as_str(),
            generated.intent.patch.compatibility,
            received,
            before_state == after_state,
            self.full_rebuilds,
            generated
                .operation_families
                .iter()
                .map(|family| family.as_str())
                .collect::<Vec<_>>()
                .join(",")
        ));
        Ok(generated)
    }

    pub fn apply_all_supported(
        &mut self,
        inputs: &[&'static str],
    ) -> HotReloadResult<Vec<GeneratedHotReloadPatch>> {
        let mut patches = Vec::new();
        for input in inputs {
            patches.push(self.apply_declarative(input)?);
        }
        validate_required_patch_family_coverage(&patches).map_err(|family| {
            HotReloadDiagnostic::error(
                "HOT_RELOAD_PATCH_FAMILY_COVERAGE_MISSING",
                "$.hot_reload.patch_families",
                "required patch family missing",
                format!("missing patch family {}", family.as_str()),
                None,
            )
        })?;
        Ok(patches)
    }

    pub fn trace_snapshot(&self) -> String {
        format!("{}\n", self.trace.join("\n"))
    }

    pub fn received_count(&self) -> usize {
        self.receiver.received().len()
    }

    pub fn full_rebuilds(&self) -> u32 {
        self.full_rebuilds
    }

    pub fn backend_operation_count(&self) -> usize {
        self.backend.operations().len()
    }

    pub fn state_token(&self) -> HotReloadResult<String> {
        self.state
            .value(StateId::new("app"))
            .map(|value| value.display_token())
            .map_err(runtime_error)
    }

    fn apply_host_side_metadata(&mut self, generated: &GeneratedHotReloadPatch) {
        match &generated.intent.edit {
            HotReloadEdit::Binding(binding) => {
                self.bindings.bindings.push(binding.binding.clone());
            }
            HotReloadEdit::ActionBody(_) => {
                self.full_rebuilds += 1;
            }
            _ => {}
        }
    }
}

fn runtime_error(diagnostic: valdi_rust_runtime::RuntimeDiagnostic) -> HotReloadDiagnostic {
    HotReloadDiagnostic::rebuild_required(
        diagnostic.code,
        diagnostic.path,
        "runtime hot reload compatibility check failed",
        diagnostic.message,
        diagnostic.source_span_id,
    )
}
