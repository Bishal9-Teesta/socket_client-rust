mod print_type_of;
mod stream_data_type;

use serde_json::Value;
use stream_data_type::StreamDataType;
use websockets::WebSocket;

#[tokio::main]

// ! Without Error Handling
// async fn main() {
//     let mut ws = WebSocket::connect("wss://fstream.binance.com/ws/btcusdt@depth20@100ms")
//         .await
//         .unwrap();
//
//     loop {
//         let socket_value = ws
//             .receive()
//             .await
//             .expect("Socket received bad data! ")
//             .into_text()
//             .expect("Socket data cannot convert to text!")
//             .0
//             .parse()
//             .expect("Socket data was not parsable from String data type");
//         let data: Value = serde_json::from_value(socket_value).unwrap();
//         // println!("Data: {:#?}", data["pu"]);
//         // print_type_of::print_type_of(&data);
//         let socket_data: StreamDataType = serde_json::from_value(data).unwrap();
//         println!("Socket JSON Data: {:#?}", socket_data);
//     }
// }

// ! With Error Handling
async fn main() {
    let mut websocket_status =
        WebSocket::connect("wss://fstream.binance.com/ws/btcusdt@depth20@100ms").await;

    match websocket_status {
        Err(_error) => {
            println!("Web Socket connection creation error: {}", _error);
        }
        Ok(mut websocket) => {
            loop {
                let raw_socket_data = websocket.receive().await;

                match raw_socket_data {
                    Err(_error) => {
                        println!("Web Socket data receiving error: {}", _error);
                    }
                    Ok(socket_data) => {
                        let socket_text_data = socket_data.into_text();

                        match socket_text_data {
                            None => {
                                println!("Got NONE Value from Socket");
                            }
                            Some(text_data) => {
                                let stream_data = serde_json::from_value::<StreamDataType>(text_data.0.parse().unwrap());

                                match stream_data {
                                    Err(_error) => {
                                        println!("Stream data deserialization error: {}", _error);
                                    }
                                    Ok(data) => {
                                        println!("Stream data: {:#?}", data);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
