use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;
use std::ops::Not;

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

impl Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

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

pub struct PieceIter<'a> {
    pieces: &'a [Piece],
    idx: usize,
}

impl<'a> Iterator for PieceIter<'a> {
    type Item = &'a Piece;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.pieces.len() {
            let val = &self.pieces[self.idx];
            self.idx += 1;
            Some(val)
        } else {
            None
        }
    }
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
        let piece_idx = src_idx.get() - 1;

        let piece = self.pieces[piece_idx as usize].clone();
        let pos = (t_idx1, t_idx2);
        let valid_move = match piece.piecetype {
            PieceType::Pawn => self.valid_pawn_move(&piece, pos),
            PieceType::Rook => self.valid_rook_move(&piece, pos),
            PieceType::Knight => self.valid_knight_move(&piece, pos),
            PieceType::King => self.valid_king_move(&piece, pos),
            PieceType::Queen => self.valid_queen_move(&piece, pos),
            PieceType::Bishop => self.valid_bishop_move(&piece, pos),
        };

        if let Some(_target_idx) = self.map[t_idx1][t_idx2] {
            return Err(MovePieceError::IllegalMove);
        }

        self.map[t_idx1][t_idx2] = Some(src_idx);
        self.map[f_idx1][f_idx2] = None;
        self.pieces[piece_idx as usize].position = Some(to);

        Ok(())
    }

    fn iter_pieces(&self, player: Player) -> PieceIter {
        let mid = self.pieces.len() / 2;
        match player {
            Player::White => PieceIter {
                idx: 0,
                pieces: &self.pieces[..mid],
            },
            Player::Black => PieceIter {
                idx: 0,
                pieces: &self.pieces[mid..],
            },
        }
    }

    /// Is player in check?
    fn is_check(&self, player: Player) -> bool {
        // Determine if king is about to be captured...
        for piece in self.iter_pieces(!player) {}

        false
    }

    fn valid_pawn_move(&self, piece: &Piece, dest: (usize, usize)) -> bool {
        // Forward one space
        // Forward two space on first move
        // En Passant
        // Diagonal to capture
        true
    }
    fn valid_rook_move(&self, piece: &Piece, dest: (usize, usize)) -> bool {
        // Forward, backward, sideways any number space
        // Castle (when valid)
        //
        true
    }
    fn valid_knight_move(&self, piece: &Piece, dest: (usize, usize)) -> bool {
        // Forward 2, side 1
        // (8 possible moves)
        true
    }
    fn valid_king_move(&self, piece: &Piece, dest: (usize, usize)) -> bool {
        // One space any direction
        // Castle (when valid)
        true
    }
    fn valid_queen_move(&self, piece: &Piece, dest: (usize, usize)) -> bool {
        // Forward, backward, sideways, and diagonally any number space
        true
    }
    fn valid_bishop_move(&self, piece: &Piece, dest: (usize, usize)) -> bool {
        // Diagonally any number space
        true
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
            Piece { white: true,  piecetype: PieceType::Rook,   position: new_loc(1, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::Rook,   position: new_loc(8, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::Knight, position: new_loc(2, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::Knight, position: new_loc(7, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::Bishop, position: new_loc(3, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::Bishop, position: new_loc(6, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::Queen,  position: new_loc(4, 1), alive: true },
            Piece { white: true,  piecetype: PieceType::King,   position: new_loc(5, 1), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: new_loc(1, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: new_loc(2, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: new_loc(3, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: new_loc(4, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: new_loc(5, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: new_loc(6, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: new_loc(7, 7), alive: true },
            Piece { white: false, piecetype: PieceType::Pawn,   position: new_loc(8, 7), alive: true },
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
