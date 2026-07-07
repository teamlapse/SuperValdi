//! Asset schema.
//!
//! ```
//! use valdi_rust_ir::{assets::{AssetRef, AssetResizeMode}, ids::AssetId};
//!
//! let asset = AssetRef { id: AssetId::new("logo"), source: "logo.png", resize_mode: AssetResizeMode::Contain };
//! assert_eq!(asset.id.as_str(), "logo");
//! ```

use crate::ids::AssetId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssetRef {
    pub id: AssetId,
    pub source: &'static str,
    pub resize_mode: AssetResizeMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssetVariant {
    pub platform: &'static str,
    pub scale: u16,
    pub path: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RemoteAssetRef {
    pub url: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DataAssetRef {
    pub media_type: &'static str,
    pub bytes_label: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AssetResizeMode {
    Cover,
    Contain,
    Stretch,
    Center,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AssetLoadingState {
    Pending,
    Loaded,
    Failed(AssetError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssetError {
    pub code: &'static str,
    pub recoverable: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AnimatedAssetMetadata {
    pub frame_count: u32,
    pub duration_ms: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssetCacheKey {
    pub id: AssetId,
    pub variant: &'static str,
}
