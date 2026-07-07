use valdi_rust_ir::{
    animations::{Animation, AnimationKind, AnimationLifecycle, Timing, TransactionGroup},
    assets::AssetLoadingState,
    elements::{ElementIdentity, ElementKind},
    events::{DrawEventPayload, FrameObserverPayload, LayoutEventPayload, VisibilityEventPayload},
    ids::{AssetId, ComponentId, NativeViewId, NodeId},
    native_views::LifecycleHook,
    styling::{Color, Style},
    text::TextMeasurementHook,
    tree::{ChildOrder, DestructionPolicy},
    web_dom::{ClassEmission, DomMapping},
};

use crate::{
    capabilities::{BackendCapabilitySet, BackendTarget, FIXTURE_TAG_BACKEND_DECISIONS},
    diagnostics::BackendResult,
    operations::{
        AnimationOperation, AssetLoadOperation, AttributeStyleChange, AttributeStyleOperation,
        BackendOperation, CreateOperation, DestroyOperation, DrawCallbackOperation,
        FrameObserverOperation, LayoutCallbackOperation, MoveOperation, NativeViewMountOperation,
        RootOperation, TextMeasureOperation, TransactionGroupOperation, TransactionPhase,
        VisibilityObserverOperation, WebViewMountOperation,
    },
    validator::validate_operation_capabilities,
    RenderBackend,
};

#[derive(Clone, Debug)]
pub struct MockBackend {
    capabilities: BackendCapabilitySet,
    operations: Vec<BackendOperation>,
}

impl MockBackend {
    pub fn new(capabilities: BackendCapabilitySet) -> Self {
        Self {
            capabilities,
            operations: Vec::new(),
        }
    }

    pub fn operations(&self) -> &[BackendOperation] {
        &self.operations
    }
}

impl RenderBackend for MockBackend {
    fn target(&self) -> BackendTarget {
        self.capabilities.target
    }

    fn capabilities(&self) -> BackendCapabilitySet {
        self.capabilities
    }

    fn apply(&mut self, operation: BackendOperation) -> BackendResult<()> {
        validate_operation_capabilities(&operation, self.capabilities)?;
        self.operations.push(operation);
        Ok(())
    }
}

pub fn sample_backend_operations() -> Vec<BackendOperation> {
    let root = NodeId::new("node.root");
    let child = NodeId::new("node.child");
    let component = ComponentId::new("component.main");

    vec![
        BackendOperation::Create(CreateOperation {
            identity: ElementIdentity {
                component_id: component,
                node_id: root,
                kind: ElementKind::View,
            },
        }),
        BackendOperation::Root(RootOperation { node_id: root }),
        BackendOperation::Move(MoveOperation {
            parent_id: root,
            child_id: child,
            order: ChildOrder(1),
            key: None,
        }),
        BackendOperation::AttributeStyle(AttributeStyleOperation {
            node_id: child,
            change: AttributeStyleChange::Style(Style::minimal(Color::rgba(12, 34, 56, 255))),
        }),
        BackendOperation::Animation(AnimationOperation {
            animation: Animation {
                node_id: child,
                kind: AnimationKind::Property,
                timing: Timing::milliseconds(180),
            },
            lifecycle: AnimationLifecycle::Start,
        }),
        BackendOperation::Animation(AnimationOperation {
            animation: Animation {
                node_id: child,
                kind: AnimationKind::Property,
                timing: Timing::milliseconds(180),
            },
            lifecycle: AnimationLifecycle::End,
        }),
        BackendOperation::Animation(AnimationOperation {
            animation: Animation {
                node_id: child,
                kind: AnimationKind::Property,
                timing: Timing::milliseconds(180),
            },
            lifecycle: AnimationLifecycle::Cancel,
        }),
        BackendOperation::LayoutCallback(LayoutCallbackOperation {
            node_id: child,
            payload: LayoutEventPayload {
                width: 320.0,
                height: 48.0,
            },
        }),
        BackendOperation::DrawCallback(DrawCallbackOperation {
            node_id: child,
            payload: DrawEventPayload { frame_number: 7 },
        }),
        BackendOperation::VisibilityObserver(VisibilityObserverOperation {
            node_id: child,
            payload: VisibilityEventPayload { visible: true },
        }),
        BackendOperation::FrameObserver(FrameObserverOperation {
            node_id: child,
            payload: FrameObserverPayload { frame_number: 8 },
        }),
        BackendOperation::AssetLoad(AssetLoadOperation {
            node_id: child,
            asset_id: AssetId::new("asset.logo"),
            state: AssetLoadingState::Loaded,
        }),
        BackendOperation::TextMeasure(TextMeasureOperation {
            node_id: child,
            hook: TextMeasurementHook {
                hook_name: "measure.primary_label",
            },
            constraint_width_points: 320,
        }),
        BackendOperation::NativeViewMount(NativeViewMountOperation {
            node_id: child,
            native_view_id: NativeViewId::new("native.camera"),
            lifecycle: LifecycleHook::Create,
        }),
        BackendOperation::WebViewMount(WebViewMountOperation {
            node_id: child,
            dom_mapping: DomMapping {
                tag_name: "div",
                class_emission: ClassEmission::Static("valdi-root"),
            },
        }),
        BackendOperation::TransactionGroup(TransactionGroupOperation {
            group: TransactionGroup {
                name: "initial_render",
            },
            phase: TransactionPhase::Commit,
        }),
        BackendOperation::Destroy(DestroyOperation {
            node_id: child,
            policy: DestructionPolicy::DestroySubtree,
        }),
    ]
}

pub fn mock_backend_snapshot() -> String {
    let operations = sample_backend_operations();
    let mut lines = vec![
        "backend_operation_snapshot_v1".to_string(),
        "operations:".to_string(),
    ];
    for operation in operations {
        lines.push(format!(
            "- family={} capability={}",
            operation.family().as_str(),
            operation.family().required_capability().as_str()
        ));
    }
    lines.push("fixture_tag_decisions:".to_string());
    for decision in FIXTURE_TAG_BACKEND_DECISIONS {
        let families = decision
            .operation_families
            .iter()
            .map(|family| family.as_str())
            .collect::<Vec<_>>()
            .join(",");
        lines.push(format!(
            "- tag={} target={} capability={} families={}",
            decision.fixture_tag,
            decision.target.as_str(),
            decision.capability.as_str(),
            families
        ));
    }
    format!("{}\n", lines.join("\n"))
}
