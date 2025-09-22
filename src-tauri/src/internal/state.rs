use crate::internal::recorder::RecorderHandle;
use std::sync::Mutex;

#[derive(Default)]
pub struct AppState {
    pub virtual_mic: Mutex<String>,
    pub recorder: Mutex<Option<RecorderHandle>>,
}
