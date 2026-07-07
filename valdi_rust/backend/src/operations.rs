use valdi_rust_ir::{
    animations::{Animation, AnimationLifecycle, TransactionGroup},
    assets::AssetLoadingState,
    elements::ElementIdentity,
    events::{DrawEventPayload, FrameObserverPayload, LayoutEventPayload, VisibilityEventPayload},
    ids::{AssetId, BindingId, KeyId, NativeViewId, NodeId},
    native_views::LifecycleHook,
    styling::Style,
    text::TextMeasurementHook,
    tree::{ChildOrder, DestructionPolicy},
    web_dom::DomMapping,
};

use crate::capabilities::BackendCapability;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendOperationFamily {
    Create,
    Destroy,
    Root,
    Move,
    AttributeStyle,
    AnimationStart,
    AnimationEnd,
    AnimationCancel,
    LayoutCallback,
    DrawCallback,
    VisibilityObserver,
    FrameObserver,
    AssetLoad,
    TextMeasure,
    NativeViewMount,
    WebViewMount,
    TransactionGroup,
}

impl BackendOperationFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Destroy => "destroy",
            Self::Root => "root",
            Self::Move => "move",
            Self::AttributeStyle => "attribute_style",
            Self::AnimationStart => "animation_start",
            Self::AnimationEnd => "animation_end",
            Self::AnimationCancel => "animation_cancel",
            Self::LayoutCallback => "layout_callback",
            Self::DrawCallback => "draw_callback",
            Self::VisibilityObserver => "visibility_observer",
            Self::FrameObserver => "frame_observer",
            Self::AssetLoad => "asset_load",
            Self::TextMeasure => "text_measure",
            Self::NativeViewMount => "native_view_mount",
            Self::WebViewMount => "webview_mount",
            Self::TransactionGroup => "transaction_group",
        }
    }

    pub const fn diagnostic_path(self) -> &'static str {
        match self {
            Self::Create => "$.operations[].create",
            Self::Destroy => "$.operations[].destroy",
            Self::Root => "$.operations[].root",
            Self::Move => "$.operations[].move",
            Self::AttributeStyle => "$.operations[].attribute_style",
            Self::AnimationStart => "$.operations[].animation_start",
            Self::AnimationEnd => "$.operations[].animation_end",
            Self::AnimationCancel => "$.operations[].animation_cancel",
            Self::LayoutCallback => "$.operations[].layout_callback",
            Self::DrawCallback => "$.operations[].draw_callback",
            Self::VisibilityObserver => "$.operations[].visibility_observer",
            Self::FrameObserver => "$.operations[].frame_observer",
            Self::AssetLoad => "$.operations[].asset_load",
            Self::TextMeasure => "$.operations[].text_measure",
            Self::NativeViewMount => "$.operations[].native_view_mount",
            Self::WebViewMount => "$.operations[].webview_mount",
            Self::TransactionGroup => "$.operations[].transaction_group",
        }
    }

    pub const fn required_capability(self) -> BackendCapability {
        match self {
            Self::Create | Self::Destroy | Self::Root | Self::Move => BackendCapability::ViewTree,
            Self::AttributeStyle => BackendCapability::AttributesAndStyle,
            Self::AnimationStart | Self::AnimationEnd | Self::AnimationCancel => {
                BackendCapability::Animations
            }
            Self::LayoutCallback => BackendCapability::LayoutCallbacks,
            Self::DrawCallback => BackendCapability::DrawCallbacks,
            Self::VisibilityObserver => BackendCapability::VisibilityObservers,
            Self::FrameObserver => BackendCapability::FrameObservers,
            Self::AssetLoad => BackendCapability::AssetLoading,
            Self::TextMeasure => BackendCapability::TextMeasurement,
            Self::NativeViewMount => BackendCapability::NativeViewMount,
            Self::WebViewMount => BackendCapability::WebViewMount,
            Self::TransactionGroup => BackendCapability::TransactionGroups,
        }
    }
}

pub const REQUIRED_OPERATION_FAMILIES: &[BackendOperationFamily] = &[
    BackendOperationFamily::Create,
    BackendOperationFamily::Destroy,
    BackendOperationFamily::Root,
    BackendOperationFamily::Move,
    BackendOperationFamily::AttributeStyle,
    BackendOperationFamily::AnimationStart,
    BackendOperationFamily::AnimationEnd,
    BackendOperationFamily::AnimationCancel,
    BackendOperationFamily::LayoutCallback,
    BackendOperationFamily::DrawCallback,
    BackendOperationFamily::VisibilityObserver,
    BackendOperationFamily::FrameObserver,
    BackendOperationFamily::AssetLoad,
    BackendOperationFamily::TextMeasure,
    BackendOperationFamily::NativeViewMount,
    BackendOperationFamily::WebViewMount,
    BackendOperationFamily::TransactionGroup,
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BackendOperation {
    Create(CreateOperation),
    Destroy(DestroyOperation),
    Root(RootOperation),
    Move(MoveOperation),
    AttributeStyle(AttributeStyleOperation),
    Animation(AnimationOperation),
    LayoutCallback(LayoutCallbackOperation),
    DrawCallback(DrawCallbackOperation),
    VisibilityObserver(VisibilityObserverOperation),
    FrameObserver(FrameObserverOperation),
    AssetLoad(AssetLoadOperation),
    TextMeasure(TextMeasureOperation),
    NativeViewMount(NativeViewMountOperation),
    WebViewMount(WebViewMountOperation),
    TransactionGroup(TransactionGroupOperation),
}

impl BackendOperation {
    pub const fn family(&self) -> BackendOperationFamily {
        match self {
            Self::Create(_) => BackendOperationFamily::Create,
            Self::Destroy(_) => BackendOperationFamily::Destroy,
            Self::Root(_) => BackendOperationFamily::Root,
            Self::Move(_) => BackendOperationFamily::Move,
            Self::AttributeStyle(_) => BackendOperationFamily::AttributeStyle,
            Self::Animation(operation) => match operation.lifecycle {
                AnimationLifecycle::Start => BackendOperationFamily::AnimationStart,
                AnimationLifecycle::End => BackendOperationFamily::AnimationEnd,
                AnimationLifecycle::Cancel => BackendOperationFamily::AnimationCancel,
            },
            Self::LayoutCallback(_) => BackendOperationFamily::LayoutCallback,
            Self::DrawCallback(_) => BackendOperationFamily::DrawCallback,
            Self::VisibilityObserver(_) => BackendOperationFamily::VisibilityObserver,
            Self::FrameObserver(_) => BackendOperationFamily::FrameObserver,
            Self::AssetLoad(_) => BackendOperationFamily::AssetLoad,
            Self::TextMeasure(_) => BackendOperationFamily::TextMeasure,
            Self::NativeViewMount(_) => BackendOperationFamily::NativeViewMount,
            Self::WebViewMount(_) => BackendOperationFamily::WebViewMount,
            Self::TransactionGroup(_) => BackendOperationFamily::TransactionGroup,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CreateOperation {
    pub identity: ElementIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DestroyOperation {
    pub node_id: NodeId,
    pub policy: DestructionPolicy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RootOperation {
    pub node_id: NodeId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MoveOperation {
    pub parent_id: NodeId,
    pub child_id: NodeId,
    pub order: ChildOrder,
    pub key: Option<KeyId>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AttributeStyleOperation {
    pub node_id: NodeId,
    pub change: AttributeStyleChange,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttributeStyleChange {
    Attribute(AttributeUpdate),
    Style(Style),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AttributeUpdate {
    pub name: AttributeName,
    pub value: AttributeValue,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttributeName {
    AccessibilityLabel,
    TestIdentifier,
    InputValue,
    NativeViewAttribute(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttributeValue {
    StaticText(&'static str),
    Bool(bool),
    Number(f32),
    Binding(BindingId),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AnimationOperation {
    pub animation: Animation,
    pub lifecycle: AnimationLifecycle,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutCallbackOperation {
    pub node_id: NodeId,
    pub payload: LayoutEventPayload,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DrawCallbackOperation {
    pub node_id: NodeId,
    pub payload: DrawEventPayload,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VisibilityObserverOperation {
    pub node_id: NodeId,
    pub payload: VisibilityEventPayload,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrameObserverOperation {
    pub node_id: NodeId,
    pub payload: FrameObserverPayload,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssetLoadOperation {
    pub node_id: NodeId,
    pub asset_id: AssetId,
    pub state: AssetLoadingState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextMeasureOperation {
    pub node_id: NodeId,
    pub hook: TextMeasurementHook,
    pub constraint_width_points: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeViewMountOperation {
    pub node_id: NodeId,
    pub native_view_id: NativeViewId,
    pub lifecycle: LifecycleHook,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebViewMountOperation {
    pub node_id: NodeId,
    pub dom_mapping: DomMapping,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransactionGroupOperation {
    pub group: TransactionGroup,
    pub phase: TransactionPhase,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransactionPhase {
    Begin,
    Commit,
    Cancel,
}
