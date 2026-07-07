use valdi_rust_backend::{
    sample_backend_operations, BackendCapability, BackendCapabilityDecision, BackendCapabilitySet,
    BackendTarget, MockBackend, RenderBackend,
};

fn apply_all<B: RenderBackend>(backend: &mut B) {
    assert_eq!(
        backend.negotiate(BackendCapability::ViewTree),
        BackendCapabilityDecision::Supported(BackendCapability::ViewTree)
    );
    for operation in sample_backend_operations() {
        backend.apply(operation).expect("operation applies");
    }
}

#[test]
fn render_backend_trait_compiles_for_configured_targets() {
    for target in [
        BackendTarget::RustHost,
        BackendTarget::Ios,
        BackendTarget::Android,
        BackendTarget::Png,
        BackendTarget::WebDom,
    ] {
        let capabilities = BackendCapabilitySet::all_for(target);
        let mut backend = MockBackend::new(capabilities);
        assert_eq!(backend.target(), target);
        apply_all(&mut backend);
        assert_eq!(
            backend.operations().len(),
            sample_backend_operations().len()
        );
    }
}
