#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
  White,
  Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square(pub u8);

pub fn color_and_piece_to_mailbox(piece: Piece, color: Color) -> u8 {
  let piece_index = match piece {
    Piece::Pawn => 0,
    Piece::Knight => 1,
    Piece::Bishop => 2,
    Piece::Rook => 3,
    Piece::Queen => 4,
    Piece::King => 5,
  };

  let color_offset = match color {
    Color::White => 0,
    Color::Black => 6,
  };

  piece_index + color_offset
}

pub fn mailbox_to_color_and_piece(n: u8) -> Option<(Piece, Color)> {
  if n == EMPTY_MAILBOX {
    return None;
  }

  let color = if n < 6 {
    Color::White
  } else {
    Color::Black
  };

  let piece = match n % 6 {
    0 => Piece::Pawn,
    1 => Piece::Knight,
    2 => Piece::Bishop,
    3 => Piece::Rook,
    4 => Piece::Queen,
    5 => Piece::King,
    _ => return None,
  };

  Some((piece, color))
}

pub const EMPTY_MAILBOX: u8 = 12;

pub const A1: Square = Square(0);
pub const B1: Square = Square(1);
pub const C1: Square = Square(2);
pub const D1: Square = Square(3);
pub const E1: Square = Square(4);
pub const F1: Square = Square(5);
pub const G1: Square = Square(6);
pub const H1: Square = Square(7);
pub const A2: Square = Square(8);
pub const B2: Square = Square(9);
pub const C2: Square = Square(10);
pub const D2: Square = Square(11);
pub const E2: Square = Square(12);
pub const F2: Square = Square(13);
pub const G2: Square = Square(14);
pub const H2: Square = Square(15);
pub const A3: Square = Square(16);
pub const B3: Square = Square(17);
pub const C3: Square = Square(18);
pub const D3: Square = Square(19);
pub const E3: Square = Square(20);
pub const F3: Square = Square(21);
pub const G3: Square = Square(22);
pub const H3: Square = Square(23);
pub const A4: Square = Square(24);
pub const B4: Square = Square(25);
pub const C4: Square = Square(26);
pub const D4: Square = Square(27);
pub const E4: Square = Square(28);
pub const F4: Square = Square(29);
pub const G4: Square = Square(30);
pub const H4: Square = Square(31);
pub const A5: Square = Square(32);
pub const B5: Square = Square(33);
pub const C5: Square = Square(34);
pub const D5: Square = Square(35);
pub const E5: Square = Square(36);
pub const F5: Square = Square(37);
pub const G5: Square = Square(38);
pub const H5: Square = Square(39);
pub const A6: Square = Square(40);
pub const B6: Square = Square(41);
pub const C6: Square = Square(42);
pub const D6: Square = Square(43);
pub const E6: Square = Square(44);
pub const F6: Square = Square(45);
pub const G6: Square = Square(46);
pub const H6: Square = Square(47);
pub const A7: Square = Square(48);
pub const B7: Square = Square(49);
pub const C7: Square = Square(50);
pub const D7: Square = Square(51);
pub const E7: Square = Square(52);
pub const F7: Square = Square(53);
pub const G7: Square = Square(54);
pub const H7: Square = Square(55);
pub const A8: Square = Square(56);
pub const B8: Square = Square(57);
pub const C8: Square = Square(58);
pub const D8: Square = Square(59);
pub const E8: Square = Square(60);
pub const F8: Square = Square(61);
pub const G8: Square = Square(62);
pub const H8: Square = Square(63);
pub const NO_SQUARE: Square = Square(64);

pub const A_FILE: u64 = 0x0101010101010101;
pub const B_FILE: u64 = 0x0202020202020202;
pub const C_FILE: u64 = 0x0404040404040404;
pub const D_FILE: u64 = 0x0808080808080808;
pub const E_FILE: u64 = 0x1010101010101010;
pub const F_FILE: u64 = 0x2020202020202020;
pub const G_FILE: u64 = 0x4040404040404040;
pub const H_FILE: u64 = 0x8080808080808080;

pub const FILES: [u64; 8] = [A_FILE, B_FILE, C_FILE, D_FILE, E_FILE, F_FILE, G_FILE, H_FILE];


pub const RANK_1 : u64 = 0x00000000000000FF;
pub const RANK_2 : u64 = 0x000000000000FF00;
pub const RANK_3 : u64 = 0x0000000000FF0000;
pub const RANK_4 : u64 = 0x00000000FF000000;
pub const RANK_5 : u64 = 0x000000FF00000000;
pub const RANK_6 : u64 = 0x0000FF0000000000;
pub const RANK_7 : u64 = 0x00FF000000000000;
pub const RANK_8 : u64 = 0xFF00000000000000;

pub const RANKS: [u64; 8] = [RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8];


pub const DIAGONAL_A1_H8: u64 = 0x8040201008040201;
pub const DIAGONAL_H1_A8: u64 = 0x0102040810204080;

pub const LIGHT_SQUARES: u64 = 0x55AA55AA55AA55AA;
pub const DARK_SQUARES: u64 = 0xAA55AA55AA55AA55;

pub const CASTLE_WHITE_KINGSIDE:  u8 = 0b0001;
pub const CASTLE_WHITE_QUEENSIDE: u8 = 0b0010;
pub const CASTLE_BLACK_KINGSIDE:  u8 = 0b0100;
pub const CASTLE_BLACK_QUEENSIDE: u8 = 0b1000;

impl Square {
  pub fn from_rank_file(rank: u8, file: u8) -> Self {
    Square(rank * 8 + file)
  }

  pub fn rank(self) -> u8 {
    self.0 / 8
  }

  pub fn file(self) -> u8 {
    self.0 % 8
  }

  pub fn bitboard(self) -> u64 {
    if self.0 >= 64 { 0 } else { 1u64 << self.0 }
  }

  pub fn is_valid(rank: u8, file: u8) -> bool {
    rank < 8 && file < 8
  }

  pub fn to_index(self) -> usize {
    self.0 as usize
  }

  pub fn is_on_board(self) -> bool {
    self.0 < 64
  }
}

pub enum MoveType {
  Quiet,
  DoublePush,
  Capture,
  Castle,
  EnPassant,
  Promotion(Piece),
}

pub struct Move {
  pub from: Square,
  pub to: Square,
  pub kind: MoveType,
}

pub struct Moves(pub Vec<Move>);

impl Moves {
  pub fn new() -> Self {
    Moves(Vec::new())
  }
}

impl IntoIterator for Moves {
  type Item = Move;
  type IntoIter = std::vec::IntoIter<Move>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl Square {
  pub fn to_algebraic(self) -> String {
    let file = (b'a' + self.file()) as char;
    let rank = (b'1' + self.rank()) as char;
    format!("{}{}", file, rank)
  }
}