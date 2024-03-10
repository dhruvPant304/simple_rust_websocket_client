use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Event{
    pub(crate) sender_id: Option<String>,
    pub(crate) event_name: String,
    pub(crate) event_data: String
}

impl Event {
    pub fn new_text_event(message: String) -> Event{
        Event{
            sender_id: None,
            event_name: "text".to_string(),
            event_data: message
        }
    }
}
