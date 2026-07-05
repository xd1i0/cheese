use std::ptr::{null, NonNull};
use crate::board::BoardErr::InvalidSquare;
use crate::types::{color_and_piece_to_mailbox, mailbox_to_color_and_piece, Color, Move, MoveType, Piece, Square, CASTLE_BLACK_KINGSIDE, CASTLE_BLACK_QUEENSIDE, CASTLE_WHITE_KINGSIDE, CASTLE_WHITE_QUEENSIDE, EMPTY_MAILBOX, NO_SQUARE};

pub enum BoardErr {
  InvalidMove,
  InvalidSquare,
  InconsistentState,
}

#[derive(Debug, Clone)]
pub struct Board {
  pub pawns: u64,
  pub knights: u64,
  pub bishops: u64,
  pub rooks: u64,
  pub queens: u64,
  pub kings: u64,

  pub white: u64,
  pub black: u64,

  pub white_to_move: bool,

  pub castling: u8,
  pub halfmove_clock: u8,
  pub fullmove_number: u16,
  pub ep_square: Square,

  pub hash: u64,

  pub mailbox: [u8; 64],
}

impl Default for Board {
  fn default() -> Self {
    Self {
      pawns: 0,
      knights: 0,
      bishops: 0,
      rooks: 0,
      queens: 0,
      kings: 0,

      white: 0,
      black: 0,

      white_to_move: true,

      castling: 0,
      halfmove_clock: 0,
      fullmove_number: 1,
      ep_square: NO_SQUARE,

      hash: 0,

      mailbox: [EMPTY_MAILBOX; 64],
    }
  }
}

impl Board {
  pub fn startpos() -> Self {
    Self {
      pawns: 0x00FF00000000FF00,
      knights: 0x4200000000000042,
      bishops: 0x2400000000000024,
      rooks: 0x8100000000000081,
      queens: 0x0800000000000008,
      kings: 0x1000000000000010,

      white: 0x000000000000FFFF,
      black: 0xFFFF000000000000,

      white_to_move: true,

      castling: 0b1111,
      halfmove_clock: 0,
      fullmove_number: 1,
      ep_square: NO_SQUARE,

      hash: 0,

      mailbox: [EMPTY_MAILBOX; 64],
    }
  }

  pub fn place_piece(&mut self, square: Square, piece: Piece, color: Color) -> Result<(), BoardErr> {
    if !square.is_on_board() {
      return Err(InvalidSquare);
    }

    let idx = square.to_index();

    if self.mailbox[idx] != EMPTY_MAILBOX {
      return Err(InvalidSquare);
    }

    self.mailbox[idx] = color_and_piece_to_mailbox(piece, color);

    // write down color position
    match color {
      Color::White => {self.white |= square.bitboard()}
      Color::Black => {self.black |= square.bitboard()}
    }

    //write down piece
    match piece {
      Piece::Pawn => {self.pawns |= square.bitboard()}
      Piece::Knight => {self.knights |= square.bitboard()}
      Piece::Bishop => {self.bishops |= square.bitboard()}
      Piece::Rook => {self.rooks |= square.bitboard()}
      Piece::Queen => {self.queens |= square.bitboard()}
      Piece::King => {self.kings |= square.bitboard()}
    }
    Ok(())
  }

  pub fn remove_piece(&mut self, square: Square) -> Result<(), BoardErr> {
    if !square.is_on_board() {
      return Err(InvalidSquare);
    }
    let idx = square.to_index();
    if self.mailbox[idx] == EMPTY_MAILBOX {
      return Err(InvalidSquare);
    }

    let (piece, color) = mailbox_to_color_and_piece(self.mailbox[idx]).unwrap();

    if color == Color::White {
      self.white &= !square.bitboard();
    } else {
      self.black &= !square.bitboard();
    }

    match piece {
      Piece::Pawn => {self.pawns &= !square.bitboard();}
      Piece::Knight => {self.knights &= !square.bitboard();}
      Piece::Bishop => {self.bishops &= !square.bitboard();}
      Piece::Rook => {self.rooks &= !square.bitboard();}
      Piece::Queen => {self.queens &= !square.bitboard();}
      Piece::King => {self.kings &= !square.bitboard();}
    }

    self.mailbox[idx] = EMPTY_MAILBOX;

    Ok(())
  }

  pub fn move_piece(&mut self, from: Square, to: Square) -> Result<(), BoardErr> {
    let (piece, color) = mailbox_to_color_and_piece(self.mailbox[from.to_index()]).unwrap();
    self.remove_piece(from)?;
    self.place_piece(to, piece, color)?;
    Ok(())
  }

  pub fn make_move(&mut self, mv: &Move) -> Result<(), BoardErr> {
    let (piece, color) = mailbox_to_color_and_piece(self.mailbox[mv.from.to_index()])
      .ok_or(BoardErr::InvalidMove)?;

    self.ep_square = NO_SQUARE;

    match &mv.kind {
      MoveType::Quiet => {
        self.move_piece(mv.from, mv.to)?;
      }
      MoveType::Capture => {
        self.remove_piece(mv.to)?;
        self.move_piece(mv.from, mv.to)?;
      }
      MoveType::DoublePush => {
        self.move_piece(mv.from, mv.to)?;
        // ep square is the square the pawn passed through
        let ep = if color == Color::White { mv.to.0 - 8 } else { mv.to.0 + 8 };
        self.ep_square = Square(ep);
      }
      MoveType::EnPassant => {
        self.move_piece(mv.from, mv.to)?;
        // captured pawn is behind the ep square
        let captured = if color == Color::White { mv.to.0 - 8 } else { mv.to.0 + 8 };
        self.remove_piece(Square(captured))?;
      }
      MoveType::Promotion(promo_piece) => {
        // remove pawn, place promoted piece
        self.remove_piece(mv.from)?;
        if self.mailbox[mv.to.to_index()] != EMPTY_MAILBOX {
          self.remove_piece(mv.to)?;
        }
        self.place_piece(mv.to, *promo_piece, color)?;
      }
      MoveType::Castle => {
        self.move_piece(mv.from, mv.to)?;
        // move the rook based on which side
        let (rook_from, rook_to) = match mv.to.0 {
          6  => (Square(7),  Square(5)),   // white kingside
          2  => (Square(0),  Square(3)),   // white queenside
          62 => (Square(63), Square(61)),  // black kingside
          58 => (Square(56), Square(59)),  // black queenside
          _  => return Err(BoardErr::InvalidMove),
        };
        self.move_piece(rook_from, rook_to)?;
      }
    }

    // update castling rights
    match piece {
      Piece::King => {
        if color == Color::White {
          self.castling &= !(CASTLE_WHITE_KINGSIDE | CASTLE_WHITE_QUEENSIDE);
        } else {
          self.castling &= !(CASTLE_BLACK_KINGSIDE | CASTLE_BLACK_QUEENSIDE);
        }
      }
      Piece::Rook => {
        match mv.from.0 {
          0  => self.castling &= !CASTLE_WHITE_QUEENSIDE,
          7  => self.castling &= !CASTLE_WHITE_KINGSIDE,
          56 => self.castling &= !CASTLE_BLACK_QUEENSIDE,
          63 => self.castling &= !CASTLE_BLACK_KINGSIDE,
          _  => {}
        }
      }
      _ => {}
    }

    if piece == Piece::Pawn || matches!(mv.kind, MoveType::Capture) {
      self.halfmove_clock = 0;
    } else {
      self.halfmove_clock += 1;
    }

    if !self.white_to_move {
      self.fullmove_number += 1;
    }

    self.white_to_move = !self.white_to_move;

    Ok(())
  }

  pub fn check_consistency(&self) -> Result<(), BoardErr> {
    for idx in 0..64 {
      let square = Square(idx as u8);
      let bb = square.bitboard();
      let mailbox = self.mailbox[idx];

      // --- EMPTY SQUARE CASE ---
      if mailbox == EMPTY_MAILBOX {
        if (self.white & bb) != 0
          || (self.black & bb) != 0
          || (self.pawns & bb) != 0
          || (self.knights & bb) != 0
          || (self.bishops & bb) != 0
          || (self.rooks & bb) != 0
          || (self.queens & bb) != 0
          || (self.kings & bb) != 0
        {
          return Err(BoardErr::InconsistentState);
        }

        continue;
      }

      // --- NON-EMPTY SQUARE CASE ---
      let (piece, color) = mailbox_to_color_and_piece(mailbox)
        .ok_or(BoardErr::InconsistentState)?;

      // check color bitboard
      let color_ok = match color {
        Color::White => (self.white & bb) != 0,
        Color::Black => (self.black & bb) != 0,
      };

      if !color_ok {
        return Err(BoardErr::InconsistentState);
      }

      // check piece bitboard
      let piece_ok = match piece {
        Piece::Pawn => (self.pawns & bb) != 0,
        Piece::Knight => (self.knights & bb) != 0,
        Piece::Bishop => (self.bishops & bb) != 0,
        Piece::Rook => (self.rooks & bb) != 0,
        Piece::Queen => (self.queens & bb) != 0,
        Piece::King => (self.kings & bb) != 0,
      };

      if !piece_ok {
        return Err(BoardErr::InconsistentState);
      }
    }

    Ok(())
  }
}
