use futures_util::{SinkExt, StreamExt};
use serde_json::{Result, Value};
use styledlog::*;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

mod messages;
use crate::messages::{ClientPackets, Packets};

#[tokio::main]
async fn main() {
    // configure logging stuff
    add_level("info");
    add_level("warn");
    add_level("error");
    let info_level_style = Style::new().color(Color::BrightGreen).bold();
    let warn_level_style = Style::new().color(Color::BrightYellow).bold();
    let error_level_style = Style::new().color(Color::BrightRed).bold();

    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr)
        .await
        .expect("error binding localhost address");
    log_level(
        "\n",
        "info",
        info_level_style,
        "Running on ws://127.0.0.1:8080!",
        "show",
    );

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws = accept_async(stream).await.expect("ws handshake failed");
    let (mut write, mut read) = ws.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) => {
                let v: Value = match serde_json::from_str(&msg.into_text().expect("error")) {
                    Ok(v) => v,
                    Err(_) => Value::Null,
                };

                println!("{}", &v);

                let packet: Result<Packets> = serde_json::from_value(v.clone());

                match packet {
                    Ok(Packets::Chat { user, content }) => {}

                    Ok(Packets::Ping) => {
                        let msg = ClientPackets::Ping;

                        println!("ping");

                        if let Err(err) = write
                            .send(async_tungstenite::tungstenite::Message::Text(
                                serde_json::to_string(&msg).unwrap().into(),
                            ))
                            .await
                        {
                            println!("error sending pong");
                        }
                    }

                    Ok(Packets::Move { direction }) => {}

                    Ok(Packets::Spawn { username }) => {}

                    Err(_) => {
                        println!("invalid packet received {:?}", packet);
                    }
                }
            }

            Err(e) => {
                println!("aaa err {e}");
                break;
            }
        }
    }
}
