use valdi_rust_backend::{
    operations::{
        AssetLoadOperation, AttributeName, AttributeStyleChange, AttributeStyleOperation,
        AttributeUpdate, AttributeValue, BackendOperation, CreateOperation,
        LayoutCallbackOperation, MoveOperation, NativeViewMountOperation, TextMeasureOperation,
        TransactionGroupOperation, TransactionPhase,
    },
    BackendOperationFamily,
};
use valdi_rust_ir::{
    animations::TransactionGroup, assets::AssetLoadingState, elements::ElementIdentity,
    events::LayoutEventPayload, native_views::LifecycleHook, text::TextMeasurementHook,
};

use crate::patch::{
    HotReloadEdit, HotReloadEditFamily, HotReloadPatchIntent, REQUIRED_PATCH_FAMILIES,
};

#[derive(Clone, Debug, PartialEq)]
pub struct GeneratedHotReloadPatch {
    pub intent: HotReloadPatchIntent,
    pub operations: Vec<BackendOperation>,
    pub operation_families: Vec<BackendOperationFamily>,
    pub rebuild_required: bool,
}

pub fn generate_patch(intent: &HotReloadPatchIntent) -> GeneratedHotReloadPatch {
    let mut operations = vec![transaction("hot_reload_patch", TransactionPhase::Begin)];
    match &intent.edit {
        HotReloadEdit::UiTree(tree) => {
            operations.push(BackendOperation::Create(CreateOperation {
                identity: ElementIdentity {
                    component_id: valdi_rust_ir::ids::ComponentId::new("component.hot_reload"),
                    node_id: tree.node_id,
                    kind: tree.element_kind,
                },
            }));
            operations.push(BackendOperation::Move(MoveOperation {
                parent_id: tree.parent_id,
                child_id: tree.node_id,
                order: tree.order,
                key: tree.key,
            }));
        }
        HotReloadEdit::Style(style) => {
            operations.push(BackendOperation::AttributeStyle(AttributeStyleOperation {
                node_id: style.node_id,
                change: AttributeStyleChange::Style(style.style),
            }));
        }
        HotReloadEdit::Layout(layout) => {
            operations.push(BackendOperation::LayoutCallback(LayoutCallbackOperation {
                node_id: layout.node_id,
                payload: LayoutEventPayload {
                    width: 320.0,
                    height: match layout.layout.flex.direction {
                        valdi_rust_ir::layout::FlexDirection::Row => 64.0,
                        valdi_rust_ir::layout::FlexDirection::Column => 96.0,
                    },
                },
            }));
        }
        HotReloadEdit::Text(text) => {
            operations.push(BackendOperation::AttributeStyle(AttributeStyleOperation {
                node_id: text.node_id,
                change: AttributeStyleChange::Attribute(AttributeUpdate {
                    name: AttributeName::InputValue,
                    value: AttributeValue::StaticText(text.text.value),
                }),
            }));
            operations.push(BackendOperation::TextMeasure(TextMeasureOperation {
                node_id: text.node_id,
                hook: TextMeasurementHook {
                    hook_name: "hot_reload.text.measure",
                },
                constraint_width_points: 320,
            }));
        }
        HotReloadEdit::Asset(asset) => {
            operations.push(BackendOperation::AssetLoad(AssetLoadOperation {
                node_id: asset.node_id,
                asset_id: asset.asset.id,
                state: AssetLoadingState::Loaded,
            }));
        }
        HotReloadEdit::Binding(binding) => {
            operations.push(BackendOperation::AttributeStyle(AttributeStyleOperation {
                node_id: valdi_rust_ir::ids::NodeId::new("node.hot_reload.text"),
                change: AttributeStyleChange::Attribute(AttributeUpdate {
                    name: AttributeName::InputValue,
                    value: AttributeValue::Binding(binding.binding.binding_id),
                }),
            }));
        }
        HotReloadEdit::Event(event) => {
            operations.push(BackendOperation::AttributeStyle(AttributeStyleOperation {
                node_id: event.binding.node_id,
                change: AttributeStyleChange::Attribute(AttributeUpdate {
                    name: AttributeName::TestIdentifier,
                    value: AttributeValue::StaticText(event.binding.kind_name()),
                }),
            }));
        }
        HotReloadEdit::Accessibility(accessibility) => {
            operations.push(BackendOperation::AttributeStyle(AttributeStyleOperation {
                node_id: accessibility.node.node_id,
                change: AttributeStyleChange::Attribute(AttributeUpdate {
                    name: AttributeName::AccessibilityLabel,
                    value: AttributeValue::StaticText(accessibility.label.0),
                }),
            }));
        }
        HotReloadEdit::ModuleRef(_) => {
            operations.push(transaction("module_ref_patch", TransactionPhase::Commit));
        }
        HotReloadEdit::NativeViewRef(native_view) => {
            operations.push(BackendOperation::NativeViewMount(
                NativeViewMountOperation {
                    node_id: valdi_rust_ir::ids::NodeId::new("node.hot_reload.native_view"),
                    native_view_id: native_view.native_view_id,
                    lifecycle: LifecycleHook::Update,
                },
            ));
        }
        HotReloadEdit::ActionBody(_) => {}
    }
    operations.push(transaction("hot_reload_patch", TransactionPhase::Commit));
    let operation_families = operations
        .iter()
        .map(BackendOperation::family)
        .collect::<Vec<_>>();
    GeneratedHotReloadPatch {
        intent: intent.clone(),
        operations,
        operation_families,
        rebuild_required: matches!(intent.edit, HotReloadEdit::ActionBody(_)),
    }
}

pub fn validate_required_patch_family_coverage(
    patches: &[GeneratedHotReloadPatch],
) -> Result<(), HotReloadEditFamily> {
    for family in REQUIRED_PATCH_FAMILIES {
        if patches.iter().any(|patch| patch.intent.family() == *family) {
            continue;
        }
        return Err(*family);
    }
    Ok(())
}

fn transaction(name: &'static str, phase: TransactionPhase) -> BackendOperation {
    BackendOperation::TransactionGroup(TransactionGroupOperation {
        group: TransactionGroup { name },
        phase,
    })
}

trait EventKindName {
    fn kind_name(&self) -> &'static str;
}

impl EventKindName for valdi_rust_ir::events::EventBinding {
    fn kind_name(&self) -> &'static str {
        match self.kind {
            valdi_rust_ir::events::EventKind::Tap => "tap",
            valdi_rust_ir::events::EventKind::Press => "press",
            valdi_rust_ir::events::EventKind::LongPress => "long_press",
            valdi_rust_ir::events::EventKind::Pan => "pan",
            valdi_rust_ir::events::EventKind::Scroll => "scroll",
            valdi_rust_ir::events::EventKind::Focus => "focus",
            valdi_rust_ir::events::EventKind::Blur => "blur",
            valdi_rust_ir::events::EventKind::Input => "input",
            valdi_rust_ir::events::EventKind::KeyboardSubmit => "keyboard_submit",
            valdi_rust_ir::events::EventKind::Layout => "layout",
            valdi_rust_ir::events::EventKind::Draw => "draw",
            valdi_rust_ir::events::EventKind::Visibility => "visibility",
            valdi_rust_ir::events::EventKind::FrameObserver => "frame_observer",
            valdi_rust_ir::events::EventKind::CustomNative(_) => "custom_native",
        }
    }
}
