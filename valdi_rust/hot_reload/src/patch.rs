use valdi_rust_ir::{
    accessibility::{AccessibilityLabel, AccessibilityNode},
    assets::AssetRef,
    elements::ElementKind,
    events::EventBinding,
    hot_reload::{HotReloadPatch, PatchIdentity},
    ids::{ActionId, HotReloadIdentityId, KeyId, ModuleId, NativeViewId, NodeId, SourceSpanId},
    layout::LayoutStyle,
    styling::Style,
    text::TextNode,
    tree::ChildOrder,
};
use valdi_rust_runtime::RuntimeBinding;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HotReloadEditFamily {
    UiTree,
    Style,
    Layout,
    Text,
    Asset,
    Binding,
    Event,
    Accessibility,
    ModuleRef,
    NativeViewRef,
    ActionBody,
}

impl HotReloadEditFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UiTree => "ui_tree",
            Self::Style => "style",
            Self::Layout => "layout",
            Self::Text => "text",
            Self::Asset => "asset",
            Self::Binding => "binding",
            Self::Event => "event",
            Self::Accessibility => "accessibility",
            Self::ModuleRef => "module_ref",
            Self::NativeViewRef => "native_view_ref",
            Self::ActionBody => "action_body",
        }
    }

    pub const fn path(self) -> &'static str {
        match self {
            Self::UiTree => "$.patch.ui_tree",
            Self::Style => "$.patch.style",
            Self::Layout => "$.patch.layout",
            Self::Text => "$.patch.text",
            Self::Asset => "$.patch.asset",
            Self::Binding => "$.patch.binding",
            Self::Event => "$.patch.event",
            Self::Accessibility => "$.patch.accessibility",
            Self::ModuleRef => "$.patch.module_ref",
            Self::NativeViewRef => "$.patch.native_view_ref",
            Self::ActionBody => "$.patch.action_body",
        }
    }
}

pub const REQUIRED_PATCH_FAMILIES: &[HotReloadEditFamily] = &[
    HotReloadEditFamily::UiTree,
    HotReloadEditFamily::Style,
    HotReloadEditFamily::Layout,
    HotReloadEditFamily::Text,
    HotReloadEditFamily::Asset,
    HotReloadEditFamily::Binding,
    HotReloadEditFamily::Event,
    HotReloadEditFamily::Accessibility,
    HotReloadEditFamily::ModuleRef,
    HotReloadEditFamily::NativeViewRef,
];

#[derive(Clone, Debug, PartialEq)]
pub struct HotReloadPatchIntent {
    pub patch: HotReloadPatch,
    pub edit: HotReloadEdit,
    pub source_span_id: Option<SourceSpanId>,
}

impl HotReloadPatchIntent {
    pub fn new(
        id: HotReloadIdentityId,
        edit: HotReloadEdit,
        source_span_id: Option<SourceSpanId>,
    ) -> Self {
        Self {
            patch: HotReloadPatch {
                identity: PatchIdentity { id, source_span_id },
                compatibility: edit.compatibility(),
            },
            edit,
            source_span_id,
        }
    }

    pub const fn family(&self) -> HotReloadEditFamily {
        self.edit.family()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HotReloadEdit {
    UiTree(UiTreePatch),
    Style(StylePatch),
    Layout(LayoutPatch),
    Text(TextPatch),
    Asset(AssetPatch),
    Binding(BindingPatch),
    Event(EventPatch),
    Accessibility(AccessibilityPatch),
    ModuleRef(ModuleRefPatchIntent),
    NativeViewRef(NativeViewRefPatchIntent),
    ActionBody(ActionBodyPatch),
}

impl HotReloadEdit {
    pub const fn family(&self) -> HotReloadEditFamily {
        match self {
            Self::UiTree(_) => HotReloadEditFamily::UiTree,
            Self::Style(_) => HotReloadEditFamily::Style,
            Self::Layout(_) => HotReloadEditFamily::Layout,
            Self::Text(_) => HotReloadEditFamily::Text,
            Self::Asset(_) => HotReloadEditFamily::Asset,
            Self::Binding(_) => HotReloadEditFamily::Binding,
            Self::Event(_) => HotReloadEditFamily::Event,
            Self::Accessibility(_) => HotReloadEditFamily::Accessibility,
            Self::ModuleRef(_) => HotReloadEditFamily::ModuleRef,
            Self::NativeViewRef(_) => HotReloadEditFamily::NativeViewRef,
            Self::ActionBody(_) => HotReloadEditFamily::ActionBody,
        }
    }

    pub const fn compatibility(&self) -> valdi_rust_ir::hot_reload::CompatibilityClass {
        match self {
            Self::ActionBody(_) => valdi_rust_ir::hot_reload::CompatibilityClass::RequiresRebuild,
            Self::UiTree(_)
            | Self::Style(_)
            | Self::Layout(_)
            | Self::Text(_)
            | Self::Asset(_)
            | Self::Binding(_)
            | Self::Event(_)
            | Self::Accessibility(_)
            | Self::ModuleRef(_)
            | Self::NativeViewRef(_) => valdi_rust_ir::hot_reload::CompatibilityClass::UiOnly,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiTreePatch {
    pub parent_id: NodeId,
    pub node_id: NodeId,
    pub element_kind: ElementKind,
    pub order: ChildOrder,
    pub key: Option<KeyId>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StylePatch {
    pub node_id: NodeId,
    pub style: Style,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutPatch {
    pub node_id: NodeId,
    pub layout: LayoutStyle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextPatch {
    pub node_id: NodeId,
    pub text: TextNode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssetPatch {
    pub node_id: NodeId,
    pub asset: AssetRef,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BindingPatch {
    pub binding: RuntimeBinding,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EventPatch {
    pub binding: EventBinding,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessibilityPatch {
    pub node: AccessibilityNode,
    pub label: AccessibilityLabel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModuleRefPatchIntent {
    pub module_id: ModuleId,
    pub expected_shape: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeViewRefPatchIntent {
    pub native_view_id: NativeViewId,
    pub expected_shape: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActionBodyPatch {
    pub action_id: ActionId,
    pub edit_token: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PatchFamilyDecision {
    pub family: HotReloadEditFamily,
    pub supported_without_rebuild: bool,
}

pub fn required_patch_family_decisions() -> Vec<PatchFamilyDecision> {
    REQUIRED_PATCH_FAMILIES
        .iter()
        .copied()
        .map(|family| PatchFamilyDecision {
            family,
            supported_without_rebuild: true,
        })
        .collect()
}
