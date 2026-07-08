use valdi_rust_ir::{
    ids::NativeViewId,
    native_views::{LifecycleHook, NativeViewAttribute, NativeViewContract, NativeViewEvent},
};

pub const fn native_view_contract(
    id: NativeViewId,
    lifecycle: LifecycleHook,
) -> NativeViewContract {
    NativeViewContract { id, lifecycle }
}

pub const fn native_view_attribute(
    name: &'static str,
    value_type: &'static str,
) -> NativeViewAttribute {
    NativeViewAttribute { name, value_type }
}

pub const fn native_view_event(name: &'static str, payload_type: &'static str) -> NativeViewEvent {
    NativeViewEvent { name, payload_type }
}
