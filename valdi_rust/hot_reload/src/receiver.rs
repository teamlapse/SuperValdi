use crate::transport::HotReloadTransportMessage;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppHotReloadReceiver {
    session_id: &'static str,
    received: Vec<HotReloadTransportMessage>,
}

impl AppHotReloadReceiver {
    pub const fn new(session_id: &'static str) -> Self {
        Self {
            session_id,
            received: Vec::new(),
        }
    }

    pub fn receive(&mut self, message: HotReloadTransportMessage) -> bool {
        if message.session_id != self.session_id {
            return false;
        }
        self.received.push(message);
        true
    }

    pub fn received(&self) -> &[HotReloadTransportMessage] {
        &self.received
    }
}
