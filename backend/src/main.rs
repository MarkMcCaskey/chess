mod chess;

//use futures::{SinkExt, StreamExt};
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::{env, io::Error};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, info, trace, warn};
use tracing_subscriber;

use crate::chess::{Board, PieceType};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ServerMessage {
    BoardState(Board),
    UnrecognizedMessage(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ClientMessage {
    Connect,
    MovePiece { piece_index: u8, location: (u8, u8) },
    Resign,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    White,
    Black,
}

impl Default for Player {
    fn default() -> Self {
        Player::White
    }
}

#[derive(Debug, Clone, Default)]
pub struct GameState {
    board: Board,
    turn: Player,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .pretty()
        .with_thread_names(true)
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    let game_state = Arc::new(Mutex::new(GameState::default()));

    while let Ok((stream, _)) = listener.accept().await {
        let game_state = game_state.clone();
        tokio::spawn(async move { accept_connection(stream, game_state) }.await);
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream, game_state: Arc<Mutex<GameState>>) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (tx, rx) = unbounded();
    let (write, read) = ws_stream.split();
    let msg_handler = read.try_for_each(move |client_msg| {
        debug!("Found client message: {:?}", &client_msg);

        let client_msg: ClientMessage = match client_msg {
            Message::Text(text) => match serde_json::from_str(&text) {
                Ok(v) => v,
                Err(_) => {
                    todo!("malformed client message!");
                }
            },
            Message::Binary(_) => {
                warn!("Binary message ignored!");
                todo!();
            }
            Message::Ping(_) => {
                warn!("Ping message ignored!");
                todo!();
            }
            Message::Pong(_) => {
                warn!("Pong message ignored!");
                todo!();
            }
            Message::Close(_) => {
                warn!("Close message ignored!");
                todo!();
            }
        };
        debug!("Found client message: {:?}", &client_msg);

        let response = match client_msg {
            ClientMessage::Connect => {
                let gs = game_state.lock().unwrap();
                ServerMessage::BoardState(gs.board.clone())
            }
            ClientMessage::MovePiece { .. } => todo!("move piece"),
            ClientMessage::Resign => todo!("resign"),
        };
        debug!("Responding with: {:?}", &response);
        tx.unbounded_send(response).unwrap();

        future::ok(())
    });

    //let msg_json = serde_json::to_string(&response).expect("Could not serialize message");
    let receive_from_others = rx
        .map(|msg| {
            Ok(Message::Text(
                serde_json::to_string(&msg).expect("Could not serialize message"),
            ))
        })
        .forward(write);

    pin_mut!(msg_handler, receive_from_others);
    future::select(msg_handler, receive_from_others).await;
    
    info!("{} disconnected", addr);
}
