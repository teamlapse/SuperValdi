use valdi_rust_ir::ids::{SourceSpanId, StateId};

use crate::{
    diagnostics::{RuntimeDiagnostic, RuntimeResult},
    patch::IdentityPatchResult,
};

pub const RUNTIME_STATE_DUPLICATE: &str = "RUNTIME_STATE_DUPLICATE";
pub const RUNTIME_STATE_MISSING: &str = "RUNTIME_STATE_MISSING";
pub const RUNTIME_STATE_TYPE_MISMATCH: &str = "RUNTIME_STATE_TYPE_MISMATCH";
pub const RUNTIME_BINDING_FIELD_MISSING: &str = "RUNTIME_BINDING_FIELD_MISSING";
pub const RUNTIME_STATE_IDENTITY_INCOMPATIBLE: &str = "RUNTIME_STATE_IDENTITY_INCOMPATIBLE";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StateValue {
    Null,
    Bool(bool),
    I64(i64),
    Text(&'static str),
    List(Vec<StateValue>),
    Record(Vec<StateField>),
}

impl StateValue {
    pub const fn kind(&self) -> StateValueKind {
        match self {
            Self::Null => StateValueKind::Null,
            Self::Bool(_) => StateValueKind::Bool,
            Self::I64(_) => StateValueKind::I64,
            Self::Text(_) => StateValueKind::Text,
            Self::List(_) => StateValueKind::List,
            Self::Record(_) => StateValueKind::Record,
        }
    }

    pub fn field(&self, name: &str) -> Option<&StateValue> {
        match self {
            Self::Record(fields) => fields
                .iter()
                .find(|field| field.name == name)
                .map(|field| &field.value),
            _ => None,
        }
    }

    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    pub const fn as_i64(&self) -> Option<i64> {
        match self {
            Self::I64(value) => Some(*value),
            _ => None,
        }
    }

    pub const fn as_text(&self) -> Option<&'static str> {
        match self {
            Self::Text(value) => Some(*value),
            _ => None,
        }
    }

    pub fn display_token(&self) -> String {
        match self {
            Self::Null => "null".to_string(),
            Self::Bool(value) => value.to_string(),
            Self::I64(value) => value.to_string(),
            Self::Text(value) => value.to_string(),
            Self::List(values) => {
                let items = values
                    .iter()
                    .map(StateValue::display_token)
                    .collect::<Vec<_>>();
                format!("[{}]", items.join(","))
            }
            Self::Record(fields) => {
                let items = fields
                    .iter()
                    .map(|field| format!("{}={}", field.name, field.value.display_token()))
                    .collect::<Vec<_>>();
                format!("{{{}}}", items.join(","))
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateValueKind {
    Null,
    Bool,
    I64,
    Text,
    List,
    Record,
}

impl StateValueKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Bool => "bool",
            Self::I64 => "i64",
            Self::Text => "text",
            Self::List => "list",
            Self::Record => "record",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateField {
    pub name: &'static str,
    pub value: StateValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateEntry {
    pub state_id: StateId,
    pub value: StateValue,
    pub source_span_id: Option<SourceSpanId>,
    pub invalidated: bool,
}

impl StateEntry {
    pub const fn new(
        state_id: StateId,
        value: StateValue,
        source_span_id: Option<SourceSpanId>,
    ) -> Self {
        Self {
            state_id,
            value,
            source_span_id,
            invalidated: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateInvalidation {
    pub state_id: StateId,
    pub reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateStore {
    entries: Vec<StateEntry>,
    invalidations: Vec<StateInvalidation>,
}

impl StateStore {
    pub fn new(entries: Vec<StateEntry>) -> RuntimeResult<Self> {
        for (index, entry) in entries.iter().enumerate() {
            if entries[index + 1..]
                .iter()
                .any(|other| other.state_id == entry.state_id)
            {
                return Err(RuntimeDiagnostic::error(
                    RUNTIME_STATE_DUPLICATE,
                    "$.state",
                    format!("duplicate state id {}", entry.state_id.as_str()),
                    entry.source_span_id,
                ));
            }
        }
        Ok(Self {
            entries,
            invalidations: Vec::new(),
        })
    }

    pub fn invalidations(&self) -> &[StateInvalidation] {
        &self.invalidations
    }

    pub fn entry(&self, state_id: StateId) -> Option<&StateEntry> {
        self.entries.iter().find(|entry| entry.state_id == state_id)
    }

    pub fn value(&self, state_id: StateId) -> RuntimeResult<&StateValue> {
        self.entry(state_id)
            .map(|entry| &entry.value)
            .ok_or_else(|| missing_state_diagnostic(state_id, None))
    }

    pub fn set(
        &mut self,
        state_id: StateId,
        value: StateValue,
        source_span_id: Option<SourceSpanId>,
    ) {
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|entry| entry.state_id == state_id)
        {
            entry.value = value;
            entry.source_span_id = source_span_id;
            return;
        }
        self.entries
            .push(StateEntry::new(state_id, value, source_span_id));
    }

    pub fn update(
        &mut self,
        state_id: StateId,
        value: StateValue,
        reason: &'static str,
    ) -> RuntimeResult<()> {
        let entry = self
            .entries
            .iter_mut()
            .find(|entry| entry.state_id == state_id)
            .ok_or_else(|| missing_state_diagnostic(state_id, None))?;
        entry.value = value;
        entry.invalidated = true;
        self.invalidations
            .push(StateInvalidation { state_id, reason });
        Ok(())
    }

    pub fn invalidate(
        &mut self,
        state_id: StateId,
        reason: &'static str,
    ) -> RuntimeResult<StateInvalidation> {
        let entry = self
            .entries
            .iter_mut()
            .find(|entry| entry.state_id == state_id)
            .ok_or_else(|| missing_state_diagnostic(state_id, None))?;
        entry.invalidated = true;
        let invalidation = StateInvalidation { state_id, reason };
        self.invalidations.push(invalidation);
        Ok(invalidation)
    }

    pub fn expect_kind(
        &self,
        state_id: StateId,
        expected: StateValueKind,
    ) -> RuntimeResult<&StateValue> {
        let entry = self
            .entry(state_id)
            .ok_or_else(|| missing_state_diagnostic(state_id, None))?;
        if entry.value.kind() == expected {
            return Ok(&entry.value);
        }
        Err(RuntimeDiagnostic::error(
            RUNTIME_STATE_TYPE_MISMATCH,
            "$.state.value",
            format!(
                "state {} has type {}, expected {}",
                state_id.as_str(),
                entry.value.kind().as_str(),
                expected.as_str()
            ),
            entry.source_span_id,
        ))
    }

    pub fn resolve_field_path(
        &self,
        path: &'static str,
        source_span_id: Option<SourceSpanId>,
    ) -> RuntimeResult<StateValue> {
        let mut segments = path.split('.');
        let root = segments.next().unwrap_or("");
        if root.is_empty() {
            return Err(RuntimeDiagnostic::error(
                RUNTIME_BINDING_FIELD_MISSING,
                "$.binding.field_path",
                "binding field path is empty",
                source_span_id,
            ));
        }

        let mut value = self.value(StateId::new(root))?;
        for segment in segments {
            value = value.field(segment).ok_or_else(|| {
                RuntimeDiagnostic::error(
                    RUNTIME_BINDING_FIELD_MISSING,
                    "$.binding.field_path",
                    format!("field path {path} is missing segment {segment}"),
                    source_span_id,
                )
            })?;
        }
        Ok(value.clone())
    }
}

pub fn retain_compatible_state(
    store: &StateStore,
    patch_result: &IdentityPatchResult,
    expected_state_ids: &[StateId],
) -> RuntimeResult<StateStore> {
    let mut retained = Vec::new();
    for expected in expected_state_ids {
        let slot = patch_result
            .preserved_state_slots
            .iter()
            .find(|slot| slot.state_id == *expected)
            .ok_or_else(|| {
                RuntimeDiagnostic::error(
                    RUNTIME_STATE_IDENTITY_INCOMPATIBLE,
                    "$.identity_patch.preserved_state_slots",
                    format!("state identity {} was not preserved", expected.as_str()),
                    None,
                )
            })?;
        let entry = store.entry(slot.state_id).ok_or_else(|| {
            RuntimeDiagnostic::error(
                RUNTIME_STATE_IDENTITY_INCOMPATIBLE,
                "$.state_patch.state",
                format!(
                    "state {} is missing from retained store",
                    slot.state_id.as_str()
                ),
                None,
            )
        })?;
        retained.push(entry.clone());
    }
    StateStore::new(retained)
}

fn missing_state_diagnostic(
    state_id: StateId,
    source_span_id: Option<SourceSpanId>,
) -> RuntimeDiagnostic {
    RuntimeDiagnostic::error(
        RUNTIME_STATE_MISSING,
        "$.state",
        format!("missing state {}", state_id.as_str()),
        source_span_id,
    )
}
