use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Piece {
    white: bool,
    piecetype: PieceType,
    position: Option<BoardLocation>,
    alive: bool,
}

impl Default for Piece {
    fn default() -> Self {
        Self {
            white: false,
            piecetype: PieceType::Pawn,
            position: None,
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
    map: [[Option<NonZeroU8>; 8]; 8],
}

#[derive(Debug, Clone)]
pub enum MovePieceError {
    // TODO: break down why the move is invalid
    IllegalMove,
}

pub type BoardLocation = (NonZeroU8, NonZeroU8);

impl Board {
    pub fn move_piece(
        &mut self,
        from: BoardLocation,
        to: BoardLocation,
    ) -> Result<(), MovePieceError> {
        let f_idx1 = from.0.get() as usize - 1;
        let f_idx2 = from.1.get() as usize - 1;
        let t_idx1 = to.0.get() as usize - 1;
        let t_idx2 = to.1.get() as usize - 1;
        let src_idx: NonZeroU8;
        if let Some(s_idx) = self.map[f_idx1][f_idx2] {
            src_idx = s_idx;
        } else {
            return Err(MovePieceError::IllegalMove);
        }

        if let Some(_target_idx) = self.map[t_idx1][t_idx2] {
            return Err(MovePieceError::IllegalMove);
        }

        self.map[t_idx1][t_idx2] = Some(src_idx);
        self.map[f_idx1][f_idx2] = None;
        let piece_idx = src_idx.get() - 1;
        self.pieces[piece_idx as usize].position = Some(to);

        Ok(())
    }
}

// TODO: make this a `const fn` when `?` in `const fn` becomes stable
fn new_loc(x: u8, y: u8) -> Option<(NonZeroU8, NonZeroU8)> {
    Some((NonZeroU8::new(x)?, NonZeroU8::new(y)?))
}

impl Default for Board {
    fn default() -> Self {
        #[rustfmt::skip]
        let pieces = vec![
            Piece { white: true,  piecetype: PieceType::Pawn,   position: new_loc(1, 2), alive: true },
            Piece { white: true,  piecetype: PieceType::Pawn,   position: new_loc(2, 2), alive: true },
            Piece { white: true,  piecetype: PieceType::Pawn,   position: new_loc(3, 2), alive: true },
            Piece { white: true,  piecetype: PieceType::Pawn,   position: new_loc(4, 2), alive: true },
            Piece { white: true,  piecetype: PieceType::Pawn,   position: new_loc(5, 2), alive: true },
            Piece { white: true,  piecetype: PieceType::Pawn,   position: new_loc(6, 2), alive: true },
            Piece { white: true,  piecetype: PieceType::Pawn,   position: new_loc(7, 2), alive: true },
            Piece { white: true,  piecetype: PieceType::Pawn,   position: new_loc(8, 2), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: new_loc(1, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: new_loc(2, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: new_loc(3, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: new_loc(4, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: new_loc(5, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: new_loc(6, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: new_loc(7, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: new_loc(8, 7), alive: true },
            Piece { white: true,  piecetype: PieceType::Rook,   position: new_loc(1, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::Rook,   position: new_loc(8, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::Knight, position: new_loc(2, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::Knight, position: new_loc(7, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::Bishop, position: new_loc(3, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::Bishop, position: new_loc(6, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::Queen,  position: new_loc(4, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::King,   position: new_loc(5, 1), alive: true },
            Piece { white: false, piecetype: PieceType::Rook,   position: new_loc(1, 8), alive: true },
            Piece { white: false, piecetype: PieceType::Rook,   position: new_loc(8, 8), alive: true },
            Piece { white: false, piecetype: PieceType::Knight, position: new_loc(2, 8), alive: true },
            Piece { white: false, piecetype: PieceType::Knight, position: new_loc(7, 8), alive: true },
            Piece { white: false, piecetype: PieceType::Bishop, position: new_loc(3, 8), alive: true },
            Piece { white: false, piecetype: PieceType::Bishop, position: new_loc(6, 8), alive: true },
            Piece { white: false, piecetype: PieceType::Queen,  position: new_loc(4, 8), alive: true },
            Piece { white: false, piecetype: PieceType::King,   position: new_loc(5, 8), alive: true },
        ];

        let mut map = [[None; 8]; 8];
        for (i, piece) in pieces.iter().enumerate() {
            let pos = piece.position.unwrap();
            map[pos.0.get() as usize - 1][pos.1.get() as usize - 1] = NonZeroU8::new(i as u8 + 1);
        }

        Self { pieces, map }
    }
}
