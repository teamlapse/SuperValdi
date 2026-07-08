//! Rust IR hot reload pipeline.

pub mod compatibility;
pub mod declarative_parser;
pub mod diagnostics;
pub mod fixtures;
pub mod latency;
pub mod patch;
pub mod patch_generator;
pub mod receiver;
pub mod registries;
pub mod sample_host;
pub mod transport;
pub mod validator;
pub mod watcher;

pub use compatibility::{compatibility_class, validate_binding_and_action_compatibility};
pub use declarative_parser::parse_declarative_patch;
pub use diagnostics::{
    HotReloadDiagnostic, HotReloadDiagnosticSeverity, HotReloadResult, HOT_RELOAD_ACTION_MISSING,
    HOT_RELOAD_BINDING_MISSING, HOT_RELOAD_LATENCY_DRIFT, HOT_RELOAD_MODULE_CONTRACT_SHAPE_CHANGED,
    HOT_RELOAD_MODULE_REF_MISSING, HOT_RELOAD_NATIVE_VIEW_CONTRACT_SHAPE_CHANGED,
    HOT_RELOAD_NATIVE_VIEW_REF_MISSING, HOT_RELOAD_NODE_MISSING, HOT_RELOAD_PARSE_ERROR,
    HOT_RELOAD_REBUILD_REQUIRED, HOT_RELOAD_TRACE_DRIFT,
};
pub use fixtures::{
    hot_reload_latency_snapshot, hot_reload_trace_snapshot, sample_runtime_document,
    validate_hot_reload_trace_snapshot, ACTION_BODY_PATCH_INPUT, HOT_RELOAD_SAMPLE_SESSION_ID,
    HOT_RELOAD_TRACE_FIXTURE_ID, MODULE_SHAPE_DRIFT_INPUT, NATIVE_VIEW_SHAPE_DRIFT_INPUT,
    SUPPORTED_PATCH_INPUTS,
};
pub use latency::{
    deterministic_latency_measurements, hot_reload_latency_report,
    validate_hot_reload_latency_report, HotReloadLatencyMeasurement, HotReloadLatencySegment,
    PATCH_LATENCY_BUDGET_US,
};
pub use patch::{
    required_patch_family_decisions, AccessibilityPatch, ActionBodyPatch, AssetPatch, BindingPatch,
    EventPatch, HotReloadEdit, HotReloadEditFamily, HotReloadPatchIntent, LayoutPatch,
    ModuleRefPatchIntent, NativeViewRefPatchIntent, PatchFamilyDecision, StylePatch, TextPatch,
    UiTreePatch, REQUIRED_PATCH_FAMILIES,
};
pub use patch_generator::{
    generate_patch, validate_required_patch_family_coverage, GeneratedHotReloadPatch,
};
pub use receiver::AppHotReloadReceiver;
pub use registries::{
    sample_module_registry, sample_native_view_registry, MockModuleRegistry,
    MockNativeViewRegistry, ModuleRefContract, NativeViewRefContract,
};
pub use sample_host::HotReloadSampleHost;
pub use transport::{HotReloadTransportMessage, InMemoryHotReloadTransport};
pub use validator::{validate_patch_intent, HotReloadValidationContext};
pub use watcher::{SimulatedFileWatcher, WatchedEdit, WatchedEditKind};

pub const CRATE_ID: &str = "valdi_rust_hot_reload";
pub const OWNER_PR: &str = "PR11";
pub const PUBLIC_API_BOUNDARY: &str = "rust_ir_hot_reload_pipeline";

pub fn foundation_dependency_ids() -> [&'static str; 4] {
    [
        valdi_rust_backend::CRATE_ID,
        valdi_rust_dsl::CRATE_ID,
        valdi_rust_ir::CRATE_ID,
        valdi_rust_runtime::CRATE_ID,
    ]
}
