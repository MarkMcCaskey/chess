use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;
use std::ops::Not;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "bool", into = "bool")]
pub enum Player {
    White,
    Black,
}

impl From<bool> for Player {
    fn from(other: bool) -> Self {
        match other {
            true => Self::White,
            false => Self::Black,
        }
    }
}
impl From<Player> for bool {
    fn from(other: Player) -> Self {
        match other {
            Player::White => true,
            Player::Black => false,
        }
    }
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
    #[serde(alias = "white", rename(serialize = "white", deserialize = "white"))]
    player: Player,
    piecetype: PieceType,
    position: Option<BoardLocation>,
    alive: bool,
    // whether the piece has ever been moved before
    moved: bool,
}

impl Piece {
    pub fn color(&self) -> Player {
        self.player
    }
}

impl Default for Piece {
    fn default() -> Self {
        Self {
            player: Player::Black,
            piecetype: PieceType::Pawn,
            position: None,
            alive: true,
            moved: false,
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
pub enum BoardSlot<'a> {
    Empty,
    OutOfBounds,
    Piece(&'a Piece),
}

#[derive(Debug, Clone)]
pub enum MovePieceError {
    // TODO: break down why the move is invalid
    IllegalMove,
    KingIsInCheck,
    NotYourTurn,
}

pub struct PieceIter<'a> {
    pieces: &'a [Piece],
    idx: usize,
}

impl<'a> Iterator for PieceIter<'a> {
    type Item = &'a Piece;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.idx < self.pieces.len() {
                let val = &self.pieces[self.idx];
                self.idx += 1;
                if !val.alive {
                    continue;
                }
                return Some(val);
            } else {
                return None;
            }
        }
    }
}

pub type BoardLocation = (NonZeroU8, NonZeroU8);

impl Board {
    pub fn move_piece(
        &mut self,
        player: Player,
        from: BoardLocation,
        to: BoardLocation,
    ) -> Result<(), MovePieceError> {
        // easier than remembering how we mutate the board, just fully reset it
        // at the cost of a copy on every move check. Good enough for now.
        let backup_board = self.clone();

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
        if piece.player != player {
            return Err(MovePieceError::NotYourTurn);
        }
        let pos = (t_idx1, t_idx2);
        let valid_move = match piece.piecetype {
            PieceType::Pawn => self.valid_pawn_move(&piece, pos),
            PieceType::Rook => self.valid_rook_move(&piece, pos),
            PieceType::Knight => self.valid_knight_move(&piece, pos),
            PieceType::King => self.valid_king_move(&piece, pos),
            PieceType::Queen => self.valid_queen_move(&piece, pos),
            PieceType::Bishop => self.valid_bishop_move(&piece, pos),
        };

        if !valid_move {
            return Err(MovePieceError::IllegalMove);
        }

        // TODO: handle captures
        if let Some(target_idx) = self.map[t_idx1][t_idx2] {
            let t_piece_idx = target_idx.get() as usize - 1;
            let target_piece = &self.pieces[t_piece_idx];
            if !target_piece.color() == piece.color() {
                self.pieces[t_piece_idx].alive = false;
                self.pieces[t_piece_idx].position = None;
                //self.map[t_idx1][t_idx2] = None;
            }

            //return Err(MovePieceError::IllegalMove);
        }

        self.map[t_idx1][t_idx2] = Some(src_idx);
        self.map[f_idx1][f_idx2] = None;
        self.pieces[piece_idx as usize].position = Some(to);
        self.pieces[piece_idx as usize].moved = true;

        let board_ended_in_check = self.is_check(player);
        if board_ended_in_check {
            // reset the board to its original position
            *self = backup_board;
            return Err(MovePieceError::KingIsInCheck);
        }

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

    fn get_king(&self, player: Player) -> &Piece {
        // hacky, probably clean this up
        let king_idx = match player {
            Player::White => 15,
            Player::Black => 31,
        };
        &self.pieces[king_idx]
    }

    /// Is player in check?
    fn is_check(&self, player: Player) -> bool {
        // Determine if king is about to be captured...
        let king = self.get_king(player);
        let (k_x, k_y) = king.position.unwrap();
        let king_pos = (k_x.get() as usize - 1, k_y.get() as usize - 1);
        self.iter_pieces(!player)
            .any(|piece| self.piece_can_attack(piece, king_pos))
    }

    fn piece_can_attack(&self, piece: &Piece, pos: (usize, usize)) -> bool {
        match piece.piecetype {
            PieceType::Pawn => self.valid_pawn_move(&piece, pos),
            PieceType::Rook => self.valid_rook_move(&piece, pos),
            PieceType::Knight => self.valid_knight_move(&piece, pos),
            PieceType::King => self.valid_king_move(&piece, pos),
            PieceType::Queen => self.valid_queen_move(&piece, pos),
            PieceType::Bishop => self.valid_bishop_move(&piece, pos),
        }
    }

    fn get_location(&self, (x, y): (usize, usize)) -> BoardSlot {
        if x > 7 || y > 7 {
            return BoardSlot::OutOfBounds;
        }
        if let Some(piece_idx) = self.map[x][y] {
            BoardSlot::Piece(&self.pieces[piece_idx.get() as usize - 1])
        } else {
            BoardSlot::Empty
        }
    }

    fn valid_pawn_move(&self, piece: &Piece, (target_x, target_y): (usize, usize)) -> bool {
        // TODO: write test cases...
        // Forward one space - done
        // Forward two space on first move - done
        // En Passant - TODO
        // Diagonal to capture - done

        let pos = piece.position.unwrap();
        let p_x = pos.0.get() as usize - 1;
        let p_y = pos.1.get() as usize - 1;
        let correct_y_move = match piece.color() {
            Player::White => {
                p_y + 1 == target_y
                    || (if !piece.moved {
                        p_y + 2 == target_y
                    } else {
                        false
                    })
            }
            Player::Black => {
                (p_y as isize - 1) == target_y as isize
                    || (if !piece.moved {
                        (p_y as isize - 2) == target_y as isize
                    } else {
                        false
                    })
            }
        };
        match self.get_location((target_x, target_y)) {
            BoardSlot::OutOfBounds => {
                return false;
            }
            BoardSlot::Empty => {
                return p_x == target_x && correct_y_move;
            }
            BoardSlot::Piece(target_piece) => {
                if target_piece.color() == piece.color() {
                    return false;
                }
                let x_diff = ((target_x as isize) - (p_x as isize)).abs();

                return x_diff == 1 && correct_y_move;
            }
        }
    }
    fn valid_rook_move(&self, piece: &Piece, (target_x, target_y): (usize, usize)) -> bool {
        // Forward, backward, sideways any number space - DONE
        // Castle (when valid) - TODO
        let pos = piece.position.unwrap();
        let p_x = pos.0.get() as usize - 1;
        let p_y = pos.1.get() as usize - 1;
        let only_moved_in_one_axis =
            (p_x == target_x && p_y != target_y) || (p_x != target_x && p_y == target_y);
        if !only_moved_in_one_axis {
            return false;
        }

        match self.get_location((target_x, target_y)) {
            BoardSlot::OutOfBounds => {
                return false;
            }
            BoardSlot::Empty => {
                return true;
            }
            BoardSlot::Piece(target_piece) => {
                if target_piece.color() == piece.color() {
                    return false;
                }

                return true;
            }
        }
    }
    fn valid_knight_move(&self, piece: &Piece, (target_x, target_y): (usize, usize)) -> bool {
        // Forward 2, side 1
        // (8 possible moves)
        let pos = piece.position.unwrap();
        let p_x = pos.0.get() as usize - 1;
        let p_y = pos.1.get() as usize - 1;
        let x_diff = (p_x as isize - target_x as isize).abs();
        let y_diff = (p_y as isize - target_y as isize).abs();
        let valid_knight_move = (x_diff == 2 && y_diff == 1) || (x_diff == 1 && y_diff == 2);
        if !valid_knight_move {
            return false;
        }

        match self.get_location((target_x, target_y)) {
            BoardSlot::OutOfBounds => {
                return false;
            }
            BoardSlot::Empty => {
                return true;
            }
            BoardSlot::Piece(target_piece) => {
                if target_piece.color() == piece.color() {
                    return false;
                }

                return true;
            }
        }
    }
    fn valid_king_move(&self, piece: &Piece, (target_x, target_y): (usize, usize)) -> bool {
        // One space any direction
        // Castle (when valid)
        let pos = piece.position.unwrap();
        let p_x = pos.0.get() as usize - 1;
        let p_y = pos.1.get() as usize - 1;
        let x_diff = (p_x as isize - target_x as isize).abs();
        let y_diff = (p_y as isize - target_y as isize).abs();
        let only_moved_1_square = x_diff <= 1 && y_diff <= 1;
        if !only_moved_1_square {
            return false;
        }

        match self.get_location((target_x, target_y)) {
            BoardSlot::OutOfBounds => {
                return false;
            }
            BoardSlot::Empty => {
                return true;
            }
            BoardSlot::Piece(target_piece) => {
                if target_piece.color() == piece.color() {
                    return false;
                }

                return true;
            }
        }
    }
    fn valid_queen_move(&self, piece: &Piece, (target_x, target_y): (usize, usize)) -> bool {
        // Forward, backward, sideways, and diagonally any number space
        let pos = piece.position.unwrap();
        let p_x = pos.0.get() as usize - 1;
        let p_y = pos.1.get() as usize - 1;
        let x_diff = (p_x as isize - target_x as isize).abs();
        let y_diff = (p_y as isize - target_y as isize).abs();
        let moved_diagonally = x_diff == y_diff;
        let moved_diagonally = x_diff == y_diff;
        let only_moved_in_one_axis =
            (p_x == target_x && p_y != target_y) || (p_x != target_x && p_y == target_y);
        if !(moved_diagonally || only_moved_in_one_axis) {
            return false;
        }

        match self.get_location((target_x, target_y)) {
            BoardSlot::OutOfBounds => {
                return false;
            }
            BoardSlot::Empty => {
                return true;
            }
            BoardSlot::Piece(target_piece) => {
                if target_piece.color() == piece.color() {
                    return false;
                }

                return true;
            }
        }
    }
    fn valid_bishop_move(&self, piece: &Piece, (target_x, target_y): (usize, usize)) -> bool {
        // Diagonally any number space
        let pos = piece.position.unwrap();
        let p_x = pos.0.get() as usize - 1;
        let p_y = pos.1.get() as usize - 1;
        let x_diff = (p_x as isize - target_x as isize).abs();
        let y_diff = (p_y as isize - target_y as isize).abs();
        let moved_diagonally = x_diff == y_diff;
        if !moved_diagonally {
            return false;
        }

        match self.get_location((target_x, target_y)) {
            BoardSlot::OutOfBounds => {
                return false;
            }
            BoardSlot::Empty => {
                return true;
            }
            BoardSlot::Piece(target_piece) => {
                if target_piece.color() == piece.color() {
                    return false;
                }

                return true;
            }
        }
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
            Piece { player: Player::White,  piecetype: PieceType::Pawn,   position: new_loc(1, 2), ..Default::default()},
            Piece { player: Player::White,  piecetype: PieceType::Pawn,   position: new_loc(2, 2), ..Default::default()},
            Piece { player: Player::White,  piecetype: PieceType::Pawn,   position: new_loc(3, 2), ..Default::default()},
            Piece { player: Player::White,  piecetype: PieceType::Pawn,   position: new_loc(4, 2), ..Default::default()},
            Piece { player: Player::White,  piecetype: PieceType::Pawn,   position: new_loc(5, 2), ..Default::default()},
            Piece { player: Player::White,  piecetype: PieceType::Pawn,   position: new_loc(6, 2), ..Default::default()},
            Piece { player: Player::White,  piecetype: PieceType::Pawn,   position: new_loc(7, 2), ..Default::default()},
            Piece { player: Player::White,  piecetype: PieceType::Pawn,   position: new_loc(8, 2), ..Default::default()},
            Piece { player: Player::White,  piecetype: PieceType::Rook,   position: new_loc(1, 1), ..Default::default()},
            Piece { player: Player::White,  piecetype: PieceType::Rook,   position: new_loc(8, 1), ..Default::default()},
            Piece { player: Player::White,  piecetype: PieceType::Knight, position: new_loc(2, 1), ..Default::default()},
            Piece { player: Player::White,  piecetype: PieceType::Knight, position: new_loc(7, 1), ..Default::default()},
            Piece { player: Player::White,  piecetype: PieceType::Bishop, position: new_loc(3, 1), ..Default::default()},
            Piece { player: Player::White,  piecetype: PieceType::Bishop, position: new_loc(6, 1), ..Default::default()},
            Piece { player: Player::White,  piecetype: PieceType::Queen,  position: new_loc(4, 1), ..Default::default()},
            Piece { player: Player::White,  piecetype: PieceType::King,   position: new_loc(5, 1), ..Default::default()},
            Piece { player: Player::Black,  piecetype: PieceType::Pawn,   position: new_loc(1, 7), ..Default::default()},
            Piece { player: Player::Black,  piecetype: PieceType::Pawn,   position: new_loc(2, 7), ..Default::default()},
            Piece { player: Player::Black,  piecetype: PieceType::Pawn,   position: new_loc(3, 7), ..Default::default()},
            Piece { player: Player::Black,  piecetype: PieceType::Pawn,   position: new_loc(4, 7), ..Default::default()},
            Piece { player: Player::Black,  piecetype: PieceType::Pawn,   position: new_loc(5, 7), ..Default::default()},
            Piece { player: Player::Black,  piecetype: PieceType::Pawn,   position: new_loc(6, 7), ..Default::default()},
            Piece { player: Player::Black,  piecetype: PieceType::Pawn,   position: new_loc(7, 7), ..Default::default()},
            Piece { player: Player::Black,  piecetype: PieceType::Pawn,   position: new_loc(8, 7), ..Default::default()},
            Piece { player: Player::Black,  piecetype: PieceType::Rook,   position: new_loc(1, 8), ..Default::default()},
            Piece { player: Player::Black,  piecetype: PieceType::Rook,   position: new_loc(8, 8), ..Default::default()},
            Piece { player: Player::Black,  piecetype: PieceType::Knight, position: new_loc(2, 8), ..Default::default()},
            Piece { player: Player::Black,  piecetype: PieceType::Knight, position: new_loc(7, 8), ..Default::default()},
            Piece { player: Player::Black,  piecetype: PieceType::Bishop, position: new_loc(3, 8), ..Default::default()},
            Piece { player: Player::Black,  piecetype: PieceType::Bishop, position: new_loc(6, 8), ..Default::default()},
            Piece { player: Player::Black,  piecetype: PieceType::Queen,  position: new_loc(4, 8), ..Default::default()},
            Piece { player: Player::Black,  piecetype: PieceType::King,   position: new_loc(5, 8), ..Default::default()},
        ];

        let mut map = [[None; 8]; 8];
        for (i, piece) in pieces.iter().enumerate() {
            let pos = piece.position.unwrap();
            map[pos.0.get() as usize - 1][pos.1.get() as usize - 1] = NonZeroU8::new(i as u8 + 1);
        }

        Self { pieces, map }
    }
}
