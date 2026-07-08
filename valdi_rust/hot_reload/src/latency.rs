use crate::diagnostics::{HotReloadResult, HOT_RELOAD_LATENCY_DRIFT};
use crate::HotReloadDiagnostic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HotReloadLatencySegment {
    FileWatch,
    IrGeneration,
    Transport,
    Validation,
    RuntimeDiff,
    MockBackendApply,
    HostReceive,
}

impl HotReloadLatencySegment {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileWatch => "file_watch",
            Self::IrGeneration => "ir_generation",
            Self::Transport => "transport",
            Self::Validation => "validation",
            Self::RuntimeDiff => "runtime_diff",
            Self::MockBackendApply => "mock_backend_apply",
            Self::HostReceive => "host_receive",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HotReloadLatencyMeasurement {
    pub segment: HotReloadLatencySegment,
    pub elapsed_us: u32,
}

pub const PATCH_LATENCY_BUDGET_US: u32 = 16_000;

pub fn deterministic_latency_measurements() -> Vec<HotReloadLatencyMeasurement> {
    vec![
        HotReloadLatencyMeasurement {
            segment: HotReloadLatencySegment::FileWatch,
            elapsed_us: 700,
        },
        HotReloadLatencyMeasurement {
            segment: HotReloadLatencySegment::IrGeneration,
            elapsed_us: 1_900,
        },
        HotReloadLatencyMeasurement {
            segment: HotReloadLatencySegment::Transport,
            elapsed_us: 600,
        },
        HotReloadLatencyMeasurement {
            segment: HotReloadLatencySegment::Validation,
            elapsed_us: 1_100,
        },
        HotReloadLatencyMeasurement {
            segment: HotReloadLatencySegment::RuntimeDiff,
            elapsed_us: 2_400,
        },
        HotReloadLatencyMeasurement {
            segment: HotReloadLatencySegment::MockBackendApply,
            elapsed_us: 1_700,
        },
        HotReloadLatencyMeasurement {
            segment: HotReloadLatencySegment::HostReceive,
            elapsed_us: 500,
        },
    ]
}

pub fn hot_reload_latency_report() -> String {
    let measurements = deterministic_latency_measurements();
    let total = measurements
        .iter()
        .map(|measurement| measurement.elapsed_us)
        .sum::<u32>();
    let mut lines = vec![
        "hot_reload_latency_report_v1".to_string(),
        format!("budget_us={PATCH_LATENCY_BUDGET_US}"),
    ];
    for measurement in measurements {
        lines.push(format!(
            "- segment={} elapsed_us={}",
            measurement.segment.as_str(),
            measurement.elapsed_us
        ));
    }
    lines.push(format!("total_us={total}"));
    format!("{}\n", lines.join("\n"))
}

pub fn validate_hot_reload_latency_report(expected: &str) -> HotReloadResult<()> {
    let actual = hot_reload_latency_report();
    if actual == expected {
        return Ok(());
    }
    Err(HotReloadDiagnostic::error(
        HOT_RELOAD_LATENCY_DRIFT,
        "$.hot_reload.latency",
        "latency report drift",
        "hot reload latency report drift",
        None,
    ))
}
