use std::collections::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Piece {
    white: bool,
    piecetype: PieceType,
    position: (u8, u8),
    alive: bool,
}

impl Default for Piece {
    fn default() -> Self {
        Self {
            white: false,
            piecetype: PieceType::Pawn,
            position: (0, 0),
            alive: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Board {
    pieces: Vec<Piece>,
    map: [[u8; 8]; 8],
}

impl Default for Board {
    fn default() -> Self {
        let pieces = vec![
            Piece {
                white: true,
                piecetype: PieceType::Pawn,
                position: (1, 2),
                alive: true,
            },
            Piece {
                white: true,
                piecetype: PieceType::Pawn,
                position: (2, 2),
                alive: true,
            },
            Piece {
                white: true,
                piecetype: PieceType::Pawn,
                position: (3, 2),
                alive: true,
            },
            Piece {
                white: true,
                piecetype: PieceType::Pawn,
                position: (4, 2),
                alive: true,
            },
            Piece {
                white: true,
                piecetype: PieceType::Pawn,
                position: (5, 2),
                alive: true,
            },
            Piece {
                white: true,
                piecetype: PieceType::Pawn,
                position: (6, 2),
                alive: true,
            },
            Piece {
                white: true,
                piecetype: PieceType::Pawn,
                position: (7, 2),
                alive: true,
            },
            Piece {
                white: true,
                piecetype: PieceType::Pawn,
                position: (8, 2),
                alive: true,
            },
            Piece {
                white: false,
                piecetype: PieceType::Pawn,
                position: (1, 7),
                alive: true,
            },
            Piece {
                white: false,
                piecetype: PieceType::Pawn,
                position: (2, 7),
                alive: true,
            },
            Piece {
                white: false,
                piecetype: PieceType::Pawn,
                position: (3, 7),
                alive: true,
            },
            Piece {
                white: false,
                piecetype: PieceType::Pawn,
                position: (4, 7),
                alive: true,
            },
            Piece {
                white: false,
                piecetype: PieceType::Pawn,
                position: (5, 7),
                alive: true,
            },
            Piece {
                white: false,
                piecetype: PieceType::Pawn,
                position: (6, 7),
                alive: true,
            },
            Piece {
                white: false,
                piecetype: PieceType::Pawn,
                position: (7, 7),
                alive: true,
            },
            Piece {
                white: false,
                piecetype: PieceType::Pawn,
                position: (8, 7),
                alive: true,
            },
            Piece {
                white: true,
                piecetype: PieceType::Rook,
                position: (1, 1),
                alive: true,
            },
            Piece {
                white: true,
                piecetype: PieceType::Rook,
                position: (8, 1),
                alive: true,
            },
            Piece {
                white: true,
                piecetype: PieceType::Knight,
                position: (2, 1),
                alive: true,
            },
            Piece {
                white: true,
                piecetype: PieceType::Knight,
                position: (7, 1),
                alive: true,
            },
            Piece {
                white: true,
                piecetype: PieceType::Bishop,
                position: (3, 1),
                alive: true,
            },
            Piece {
                white: true,
                piecetype: PieceType::Bishop,
                position: (6, 1),
                alive: true,
            },
            Piece {
                white: true,
                piecetype: PieceType::Queen,
                position: (4, 1),
                alive: true,
            },
            Piece {
                white: true,
                piecetype: PieceType::King,
                position: (5, 1),
                alive: true,
            },
            Piece {
                white: false,
                piecetype: PieceType::Rook,
                position: (1, 8),
                alive: true,
            },
            Piece {
                white: false,
                piecetype: PieceType::Rook,
                position: (8, 8),
                alive: true,
            },
            Piece {
                white: false,
                piecetype: PieceType::Knight,
                position: (2, 8),
                alive: true,
            },
            Piece {
                white: false,
                piecetype: PieceType::Knight,
                position: (7, 8),
                alive: true,
            },
            Piece {
                white: false,
                piecetype: PieceType::Bishop,
                position: (3, 8),
                alive: true,
            },
            Piece {
                white: false,
                piecetype: PieceType::Bishop,
                position: (6, 8),
                alive: true,
            },
            Piece {
                white: false,
                piecetype: PieceType::Queen,
                position: (4, 8),
                alive: true,
            },
            Piece {
                white: false,
                piecetype: PieceType::King,
                position: (5, 8),
                alive: true,
            },
        ];

        let mut map = [[0; 8]; 8];
        for (i, piece) in pieces.iter().enumerate() {
            map[piece.position.0 as usize - 1][piece.position.1 as usize - 1] = i as u8;
        }

        Self { pieces, map }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ServerMessage {
    BoardState(Board),
}

use std::{env, io::Error};

use futures::{StreamExt, SinkExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

use tracing::info;
use tracing_subscriber;

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

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (mut write, read) = ws_stream.split();
    let msg = ServerMessage::BoardState(Board::default());
    let msg_json = serde_json::to_string(&msg).expect("Could not serialize board");
    write.send(Message::Text(msg_json)).await.expect("failed to send message");
}
