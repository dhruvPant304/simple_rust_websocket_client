pub mod structs{
    pub mod messages;
    pub mod chat_controller;
    pub mod event;
}

use futures_util::{lock::Mutex,stream::{SplitSink, SplitStream}, SinkExt, StreamExt};
use structs::chat_controller::ChatController;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream};
use url::Url;
use std::{io, sync::Arc};
use tokio::net::TcpStream;

use crate::structs::event::Event;

#[tokio::main]
async fn main() {
    let url = Url::parse("ws://127.0.0.1:8080").expect("Failed to parse URL");

    // Connect to the server
    let (ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect");

    let (write,read) = ws_stream.split();
    let chat_controller = ChatController::new();
    let chat_controller_mutex_ptr = Arc::new(Mutex::new(chat_controller));
    let read_mutex = Mutex::new(read);
    let write_mutex = Mutex::new(write); 
    let read_thread = 
        tokio::spawn(read_message(read_mutex, chat_controller_mutex_ptr.clone()));
    tokio::spawn(send_message(write_mutex, chat_controller_mutex_ptr.clone()));
    let _ =read_thread.await;
}

async fn read_message(
    read_stream_mutex:Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>, 
    chat_controller: Arc<Mutex<ChatController>>){
    let mut read_stream_lock = read_stream_mutex.lock().await;
    while let Some(message) = read_stream_lock.next().await{
        match message {
            Ok(msg) => {
                //parsing received event
                let event: Event = serde_json::from_str(&msg.to_string()).expect("Error whlie parsing evnet from json"); 
                //converting to final message string
                if event.event_name == "text" { 
                    let message_string = "Recieved: ".to_string() + &event.event_data.to_string();
                    chat_controller.lock().await.add_message(message_string);
                }

                if event.event_name == "connection"{
                    chat_controller.lock().await.connected = true;
                }

                if event.event_name == "disconnection"{
                    chat_controller.lock().await.connected = false;
                    chat_controller.lock().await.reset_message();
                } 
                chat_controller.lock().await.render_chat();
            },
            Err(e) => {
                eprintln!("error while reading message: {:?}", e)
            } 
        }
    }
}

async fn send_message(
    write_stream_mutex: Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>, 
    chat_controller: Arc<Mutex<ChatController>>){
    loop {
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                if !chat_controller.lock().await.connected {
                    continue;
                }
                chat_controller.lock().await.render_chat();
                let input_string = user_input.trim().to_string();
                let event = Event::new_text_event(input_string.to_string());
                let event_string = serde_json::to_string(&event).expect("failed to convert event to json string");
                let mut write_stream_lock = write_stream_mutex.lock().await;
                let send_future = write_stream_lock.send(Message::Text(event_string));
                send_future.await.expect("failed to send message to web socket");
                let message_string = "Sent: ".to_string() + &input_string.to_string();
                chat_controller.lock().await.add_message(message_string);
                chat_controller.lock().await.render_chat();
            },
            Err(_) => {
                eprintln!("failed to read user input")
            },
        }
    }
}


