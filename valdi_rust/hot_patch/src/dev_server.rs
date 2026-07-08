use valdi_rust_ir::ids::ActionId;

use crate::{
    diagnostics::{HotPatchDiagnostic, HotPatchDiagnosticSeverity},
    loader::HotPatchLoadRecord,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HotPatchMessageKind {
    Applied,
    RebuildRequired,
}

impl HotPatchMessageKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Applied => "applied",
            Self::RebuildRequired => "rebuild_required",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HotPatchDevServerMessage {
    pub session_id: &'static str,
    pub kind: HotPatchMessageKind,
    pub action_id: Option<ActionId>,
    pub code: &'static str,
    pub path: &'static str,
    pub severity: HotPatchDiagnosticSeverity,
    pub reason: &'static str,
    pub message: String,
}

impl HotPatchDevServerMessage {
    pub fn applied(session_id: &'static str, record: HotPatchLoadRecord) -> Self {
        Self {
            session_id,
            kind: HotPatchMessageKind::Applied,
            action_id: Some(record.action_id),
            code: crate::diagnostics::HOT_PATCH_ACTION_BODY_SUPPORTED,
            path: "$.rust_hot_patch.action_body",
            severity: HotPatchDiagnosticSeverity::Info,
            reason: "action_body_changed",
            message: format!(
                "action {} patched generation {} {}->{}",
                record.action_id.as_str(),
                record.generation,
                record.previous_body_token,
                record.active_body_token
            ),
        }
    }

    pub fn rebuild_required(session_id: &'static str, diagnostic: HotPatchDiagnostic) -> Self {
        Self {
            session_id,
            kind: HotPatchMessageKind::RebuildRequired,
            action_id: None,
            code: diagnostic.code,
            path: diagnostic.path,
            severity: diagnostic.severity,
            reason: diagnostic.reason,
            message: diagnostic.message,
        }
    }

    pub fn stable_line(&self) -> String {
        let action = self
            .action_id
            .map(|action_id| action_id.as_str())
            .unwrap_or("none");
        format!(
            "session={} kind={} action={} code={} path={} severity={} reason={} message={}",
            self.session_id,
            self.kind.as_str(),
            action,
            self.code,
            self.path,
            self.severity.as_str(),
            self.reason,
            self.message
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HotPatchSession {
    pub session_id: &'static str,
    messages: Vec<HotPatchDevServerMessage>,
}

impl HotPatchSession {
    pub fn new(session_id: &'static str) -> Self {
        Self {
            session_id,
            messages: Vec::new(),
        }
    }

    pub fn publish_applied(&mut self, record: HotPatchLoadRecord) -> HotPatchDevServerMessage {
        let message = HotPatchDevServerMessage::applied(self.session_id, record);
        self.messages.push(message.clone());
        message
    }

    pub fn publish_rebuild_required(
        &mut self,
        diagnostic: HotPatchDiagnostic,
    ) -> HotPatchDevServerMessage {
        let message = HotPatchDevServerMessage::rebuild_required(self.session_id, diagnostic);
        self.messages.push(message.clone());
        message
    }

    pub fn messages(&self) -> &[HotPatchDevServerMessage] {
        &self.messages
    }
}
