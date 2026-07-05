use std::fmt;
use std::str::FromStr;

use crate::board::Board;
use crate::types::*;

#[derive(Debug)]
pub enum FenError {
  InvalidFieldCount,
  InvalidPiece(char),
  InvalidSideToMove,
  InvalidCastling,
  InvalidEnPassant,
  InvalidHalfmoveClock,
  InvalidFullmoveNumber,
  InvalidBoard,
}

impl Board {
  pub fn from_fen(fen: &str) -> Result<Self, FenError> {
    let fields: Vec<&str> = fen.split_whitespace().collect::<Vec<&str>>();

    if fields.len() != 6 {
      return Err(FenError::InvalidFieldCount);
    }

    let placement = fields[0];
    let side = fields[1];
    let castling = fields[2];
    let ep = fields[3];
    let halfmove = fields[4];
    let fullmove = fields[5];

    let mut board = Board::default();

    parse_piece_placement(&mut board, placement)?;
    parse_side_to_move(&mut board, side)?;
    parse_castling(&mut board, castling)?;
    parse_en_passant(&mut board, ep)?;

    board.halfmove_clock = halfmove.parse::<u8>().map_err(|_| FenError::InvalidHalfmoveClock)?;
    board.fullmove_number = fullmove.parse::<u16>().map_err(|_| FenError::InvalidFullmoveNumber)?;

    Ok(board)
  }

  fn encode_rank(&self, rank: u8, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut empty = 0;

    for file in 0..8 {
      let square = Square::from_rank_file(rank, file);
      let m = self.mailbox_at(square);

      if let Some(ch) = mailbox_to_fen_char(m) {
        if empty > 0 {
          write!(f, "{}", empty)?;
          empty = 0;
        }
        write!(f, "{}", ch)?;
      } else {
        empty += 1;
      }
    }

    if empty > 0 {
      write!(f, "{}", empty)?;
    }

    Ok(())
  }

  fn mailbox_at(&self, square: Square) -> u8 {
    self.mailbox[square.to_index()]
  }
}

impl FromStr for Board {
  type Err = FenError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Board::from_fen(s)
  }
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

    // --- PIECE PLACEMENT ---
    for rank in (0..8).rev() {
      self.encode_rank(rank, f)?;
      if rank != 0 {
        write!(f, "/")?;
      }
    }

    write!(f, " ")?;

    // --- SIDE TO MOVE ---
    let stm = if self.white_to_move { "w" } else { "b" };
    write!(f, "{} ", stm)?;

    // --- CASTLING ---
    let mut castling = String::new();
    if self.castling == 0 {
      castling.push('-');
    } else {
      if self.castling & CASTLE_WHITE_KINGSIDE != 0 { castling.push('K'); }
      if self.castling & CASTLE_WHITE_QUEENSIDE != 0 { castling.push('Q'); }
      if self.castling & CASTLE_BLACK_KINGSIDE != 0 { castling.push('k'); }
      if self.castling & CASTLE_BLACK_QUEENSIDE != 0 { castling.push('q'); }
    }
    write!(f, "{} ", castling)?;

    // --- EN PASSANT ---
    if self.ep_square.is_on_board() {
      let file = self.ep_square.file();
      let rank = self.ep_square.rank();

      let file_char = (b'a' + file) as char;
      let rank_char = match rank {
        2 => '3',
        5 => '6',
        _ => '-',
      };

      if rank_char == '-' {
        write!(f, "- ")?;
      } else {
        write!(f, "{}{} ", file_char, rank_char)?;
      }
    } else {
      write!(f, "- ")?;
    }

    // --- HALF/FULL MOVE ---
    write!(f, "{} {}", self.halfmove_clock, self.fullmove_number)?;

    Ok(())
  }
}

fn piece_and_color_to_fen_piece(piece: Piece, color: Color) -> char {
  let c = match piece {
    Piece::Pawn => 'p',
    Piece::Knight => 'n',
    Piece::Bishop => 'b',
    Piece::Rook => 'r',
    Piece::Queen => 'q',
    Piece::King => 'k',
  };

  match color {
    Color::White => c.to_ascii_uppercase(),
    _ => c,
  }
}

fn fen_piece_to_piece_and_color(c: char) -> Result<(Piece, Color), FenError> {
  let mut color = Color::White;
  if c.is_lowercase() {
    color = Color::Black;
  }

  let piece = match c.to_ascii_lowercase() {
    'p' => Piece::Pawn,
    'n' => Piece::Knight,
    'b' => Piece::Bishop,
    'r' => Piece::Rook,
    'q' => Piece::Queen,
    'k' => Piece::King,
    _ => return Err(FenError::InvalidPiece(c)),
  };

  Ok((piece, color))
}

fn parse_piece_placement(
  board: &mut Board,
  placement: &str,
) -> Result<(), FenError>
{
  let ranks = placement.split('/').collect::<Vec<&str>>();

  for (rank_idx, rank_str) in ranks.iter().enumerate() {
    let mut file = 0;
    let board_rank = 7 - rank_idx;

    for ch in rank_str.chars() {
      if ch.is_ascii_digit() {
        let empty = ch.to_digit(10).unwrap();
        file += empty;
      } else {
        let square: Square = Square::from_rank_file(board_rank as u8, file as u8);
        let (piece, color)  = fen_piece_to_piece_and_color(ch)?;
        board.place_piece(square, piece, color);
        file += 1;
      }
      if file > 8 {
        return Err(FenError::InvalidBoard);
      }
    }
    if file != 8 {
      return Err(FenError::InvalidBoard)
    }
  }
  Ok(())
}

fn parse_side_to_move(board: &mut Board, side: &str) -> Result<(), FenError> {
  match side {
    "w" => board.white_to_move = true,
    "b" => board.white_to_move = false,
    _ => return Err(FenError::InvalidSideToMove),
  }
  Ok(())
}

fn parse_castling(board: &mut Board, castling: &str) -> Result<(), FenError> {
  let mut c = 0b0000;
  if castling == "-" {
    board.castling = 0b0000;
    return Ok(());
  }

  for ch in castling.chars() {
    match ch {
      'K' => c |= CASTLE_WHITE_KINGSIDE,
      'Q' => c |= CASTLE_WHITE_QUEENSIDE,
      'k' => c |= CASTLE_BLACK_KINGSIDE,
      'q' => c |= CASTLE_BLACK_QUEENSIDE,
      _ => return Err(FenError::InvalidCastling),
    }
  }

  board.castling = c;
  Ok(())
}

fn parse_en_passant(board: &mut Board, ep: &str) -> Result<(), FenError> {
  if ep == "-" {
    board.ep_square = NO_SQUARE;
    return Ok(())
  }

  let mut chars = ep.chars();
  let file_char = chars.next().ok_or(FenError::InvalidEnPassant)?;
  let rank_char = chars.next().ok_or(FenError::InvalidEnPassant)?;

  if chars.next().is_some() {
    return Err(FenError::InvalidEnPassant);
  }

  let file: u8 = match file_char {
    'a' => 0,
    'b' => 1,
    'c' => 2,
    'd' => 3,
    'e' => 4,
    'f' => 5,
    'g' => 6,
    'h' => 7,
    _ => return Err(FenError::InvalidEnPassant),
  };

  let rank = match rank_char {
    '3' => 2,
    '6' => 5,
    _ => return Err(FenError::InvalidEnPassant),
  };

  board.ep_square = Square::from_rank_file(rank, file);

  Ok(())
}

fn mailbox_to_fen_char(m: u8) -> Option<char> {
  if m == EMPTY_MAILBOX {
    return None;
  }

  let (piece, color) = mailbox_to_color_and_piece(m)?;

  Some(piece_and_color_to_fen_piece(piece, color))
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  #[test]
  fn fen_round_trip_initial_position() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    let board = Board::from_str(fen).expect("valid fen");
    let output = board.to_string();

    assert_eq!(fen, output);
  }
}
