use valdi_rust_ir::ids::SourceSpanId;

use crate::diagnostics::{HotPatchDiagnostic, HotPatchResult, HOT_PATCH_RELEASE_EXCLUDED};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HotPatchBuildProfile {
    Debug,
    Release,
}

impl HotPatchBuildProfile {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Debug => "debug",
            Self::Release => "release",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReleaseExclusionReport {
    pub profile: HotPatchBuildProfile,
    pub hot_patch_machinery_present: bool,
    pub dev_loader_symbols: &'static [&'static str],
}

const DEBUG_DEV_LOADER_SYMBOLS: &[&str] = &[
    "valdi_hot_patch_action_body_loader",
    "valdi_hot_patch_action_body_registry",
];
const NO_RELEASE_SYMBOLS: &[&str] = &[];

pub const fn debug_build_report() -> ReleaseExclusionReport {
    ReleaseExclusionReport {
        profile: HotPatchBuildProfile::Debug,
        hot_patch_machinery_present: true,
        dev_loader_symbols: DEBUG_DEV_LOADER_SYMBOLS,
    }
}

pub const fn release_build_report() -> ReleaseExclusionReport {
    ReleaseExclusionReport {
        profile: HotPatchBuildProfile::Release,
        hot_patch_machinery_present: false,
        dev_loader_symbols: NO_RELEASE_SYMBOLS,
    }
}

pub fn validate_release_exclusion(report: ReleaseExclusionReport) -> HotPatchResult<()> {
    if report.profile == HotPatchBuildProfile::Release
        && !report.hot_patch_machinery_present
        && report.dev_loader_symbols.is_empty()
    {
        return Ok(());
    }

    Err(release_excluded_diagnostic(None))
}

pub fn release_excluded_diagnostic(source_span_id: Option<SourceSpanId>) -> HotPatchDiagnostic {
    HotPatchDiagnostic::rebuild_required(
        HOT_PATCH_RELEASE_EXCLUDED,
        "$.rust_hot_patch.release",
        "release_excludes_hot_patch_machinery",
        "release build excludes Rust logic hot patch machinery",
        source_span_id,
    )
}
