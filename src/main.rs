use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use std::io::{self};

#[tokio::main]
async fn main() {
    let url = Url::parse("ws://127.0.0.1:8080").expect("Failed to parse URL");

    // Connect to the server
    let (mut ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect");

    println!("WebSocket handshake has been successfully completed");

    // let send_future = ws_stream.send(Message::Text("Hello Websocket".to_string()));
    // send_future.await.expect("failed to send message to web socket");

    while let Some(message) = ws_stream.next().await {
        match  message {
            Ok(msg) => {
                println!("Received message: \n{}", msg)
            },
            Err(e) =>{
                println!("Failed to reade message {}", e)
            }
        }

        let mut user_input = String::new();
        println!("Send Message to server:");
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                let input_string = user_input.trim().to_string();
                let send_future = ws_stream.send(Message::Text(input_string));
                send_future.await.expect("failed to send message to web socket");
            },
            Err(_) => {
                panic!("failed to read user input")
            },
        }
    }
}
