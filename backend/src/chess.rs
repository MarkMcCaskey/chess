use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Piece {
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
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pieces: Vec<Piece>,
    map: [[u8; 8]; 8],
}

impl Default for Board {
    fn default() -> Self {
        #[rustfmt::skip]
        let pieces = vec![
            Piece { white: true,  piecetype: PieceType::Pawn,   position: (1, 2), alive: true },
            Piece { white: true,  piecetype: PieceType::Pawn,   position: (2, 2), alive: true },
            Piece { white: true,  piecetype: PieceType::Pawn,   position: (3, 2), alive: true },
            Piece { white: true,  piecetype: PieceType::Pawn,   position: (4, 2), alive: true },
            Piece { white: true,  piecetype: PieceType::Pawn,   position: (5, 2), alive: true },
            Piece { white: true,  piecetype: PieceType::Pawn,   position: (6, 2), alive: true },
            Piece { white: true,  piecetype: PieceType::Pawn,   position: (7, 2), alive: true },
            Piece { white: true,  piecetype: PieceType::Pawn,   position: (8, 2), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: (1, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: (2, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: (3, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: (4, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: (5, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: (6, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: (7, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: (8, 7), alive: true },
            Piece { white: true,  piecetype: PieceType::Rook,   position: (1, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::Rook,   position: (8, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::Knight, position: (2, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::Knight, position: (7, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::Bishop, position: (3, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::Bishop, position: (6, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::Queen,  position: (4, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::King,   position: (5, 1), alive: true },
            Piece { white: false, piecetype: PieceType::Rook,   position: (1, 8), alive: true },
            Piece { white: false, piecetype: PieceType::Rook,   position: (8, 8), alive: true },
            Piece { white: false, piecetype: PieceType::Knight, position: (2, 8), alive: true },
            Piece { white: false, piecetype: PieceType::Knight, position: (7, 8), alive: true },
            Piece { white: false, piecetype: PieceType::Bishop, position: (3, 8), alive: true },
            Piece { white: false, piecetype: PieceType::Bishop, position: (6, 8), alive: true },
            Piece { white: false, piecetype: PieceType::Queen,  position: (4, 8), alive: true },
            Piece { white: false, piecetype: PieceType::King,   position: (5, 8), alive: true },
        ];

        let mut map = [[0; 8]; 8];
        for (i, piece) in pieces.iter().enumerate() {
            map[piece.position.0 as usize - 1][piece.position.1 as usize - 1] = i as u8;
        }

        Self { pieces, map }
    }
}
