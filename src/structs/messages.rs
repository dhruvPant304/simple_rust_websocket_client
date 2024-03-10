pub struct MessageContainer{
    pub(crate) messages: Vec<String>
}

impl MessageContainer {
    pub fn new() -> Self{
        MessageContainer{
            messages: vec![]
        }
    }
}
