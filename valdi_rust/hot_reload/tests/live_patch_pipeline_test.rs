use valdi_rust_hot_reload::{
    HotReloadSampleHost, SimulatedFileWatcher, WatchedEdit, WatchedEditKind, SUPPORTED_PATCH_INPUTS,
};

#[test]
fn supported_patches_apply_to_persistent_host_without_rebuild() {
    let mut host = HotReloadSampleHost::new().unwrap();
    let before = host.state_token().unwrap();
    let patches = host.apply_all_supported(SUPPORTED_PATCH_INPUTS).unwrap();
    let after = host.state_token().unwrap();

    assert_eq!(patches.len(), 10);
    assert_eq!(host.received_count(), 10);
    assert_eq!(host.full_rebuilds(), 0);
    assert_eq!(before, after);
    assert!(host.backend_operation_count() >= 30);
    assert!(host
        .trace_snapshot()
        .contains("session=hot_reload.session.v1"));
}

#[test]
fn simulated_watcher_drains_declarative_edits_in_order() {
    let mut watcher = SimulatedFileWatcher::new(vec![
        WatchedEdit {
            path: "app.hot.valdi",
            kind: WatchedEditKind::DeclarativeSource,
            sequence: 1,
        },
        WatchedEdit {
            path: "logo@2x.png",
            kind: WatchedEditKind::Asset,
            sequence: 2,
        },
    ]);

    let edits = watcher.drain();
    assert_eq!(edits.len(), 2);
    assert_eq!(edits[0].kind.as_str(), "declarative_source");
    assert_eq!(edits[1].kind.as_str(), "asset");
    assert!(watcher.drain().is_empty());
}
