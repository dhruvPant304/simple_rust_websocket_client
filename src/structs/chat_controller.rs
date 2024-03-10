use super::messages::MessageContainer;

pub struct ChatController{
    pub(crate) messages_container: MessageContainer,
    pub(crate) connected: bool 
}

impl ChatController {
    pub fn new()-> Self{
        ChatController{
            messages_container: MessageContainer::new(),
            connected : false
        }
    }

    pub fn add_message(&mut self, message: String){
        self.messages_container.messages.push(message);
    }

    pub fn reset_message(&mut self){
        self.messages_container = MessageContainer::new();
    }

    pub fn render_chat(&self){
        _ = clearscreen::clear();
        let messages = &self.messages_container.messages;
        if  !self.connected { 
            println!("waiting for another client to connect!");
            return;
        }
        for message in messages.iter(){
            println!("{}", message);
        }
        println!("Enter your message");
    }
}
