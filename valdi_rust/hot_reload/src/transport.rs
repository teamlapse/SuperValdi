use crate::patch_generator::GeneratedHotReloadPatch;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HotReloadTransportMessage {
    pub sequence: u64,
    pub session_id: &'static str,
    pub patch_id: &'static str,
    pub family: &'static str,
    pub operation_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InMemoryHotReloadTransport {
    next_sequence: u64,
    messages: Vec<HotReloadTransportMessage>,
}

impl InMemoryHotReloadTransport {
    pub const fn new() -> Self {
        Self {
            next_sequence: 1,
            messages: Vec::new(),
        }
    }

    pub fn publish(
        &mut self,
        session_id: &'static str,
        patch: &GeneratedHotReloadPatch,
    ) -> HotReloadTransportMessage {
        let message = HotReloadTransportMessage {
            sequence: self.next_sequence,
            session_id,
            patch_id: patch.intent.patch.identity.id.as_str(),
            family: patch.intent.family().as_str(),
            operation_count: patch.operations.len(),
        };
        self.next_sequence += 1;
        self.messages.push(message.clone());
        message
    }

    pub fn messages(&self) -> &[HotReloadTransportMessage] {
        &self.messages
    }
}
