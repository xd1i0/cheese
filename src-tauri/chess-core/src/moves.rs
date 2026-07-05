use crate::board::Board;
use crate::types::{Move, MoveType, Moves, Piece, Square, A_FILE, H_FILE, NO_SQUARE, RANK_1, RANK_4, RANK_5, RANK_7, RANK_8};

fn push_moves(moves: &mut Moves, from: u8, mut attacks: u64, them: u64) {
  while attacks != 0 {
    let to = attacks.trailing_zeros() as u8;
    attacks &= attacks - 1;

    let kind = if (1u64 << to) & them != 0 {
      MoveType::Capture
    } else {
      MoveType::Quiet
    };

    moves.0.push(Move {
      from: Square(from),
      to: Square(to),
      kind,
    });
  }
}

pub fn generate_moves(board: &Board) -> Moves {
  let mut moves = Moves::new();
  let (us, them) = if board.white_to_move {
    (board.white, board.black)
  } else {
    (board.black, board.white)
  };

  let occupied = us | them;

  let mut knights = board.knights & us;
  let mut bishops = board.bishops & us;
  let mut rooks = board.rooks & us;
  let mut queens = board.queens & us;
  let mut king = board.kings & us;
  let mut pawns = board.pawns & us;

  while knights != 0 {
    let from = knights.trailing_zeros() as u8;
    knights &= knights - 1;
    push_moves(&mut moves, from, knight_attacks(from) & !us, them);
  }

  while bishops != 0 {
    let from = bishops.trailing_zeros() as u8;
    bishops &= bishops - 1;
    push_moves(&mut moves, from, bishop_attacks(from, occupied) & !us, them);
  }

  while rooks != 0 {
    let from = rooks.trailing_zeros() as u8;
    rooks &= rooks - 1;
    push_moves(&mut moves, from, rook_attacks(from, occupied) & !us, them);
  }

  while queens != 0 {
    let from = queens.trailing_zeros() as u8;
    queens &= queens - 1;
    push_moves(&mut moves, from, (bishop_attacks(from, occupied) | rook_attacks(from, occupied)) & !us, them);
  }

  while king != 0 {
    let from = king.trailing_zeros() as u8;
    king &= king - 1;
    push_moves(&mut moves, from, king_attacks(from) & !us, them);
  }

  if board.white_to_move {
    let single_push = (pawns << 8) & !occupied;

    // single push (non-promotion)
    let mut targets = single_push & !RANK_8;
    while targets != 0 {
      let to = targets.trailing_zeros() as u8;
      targets &= targets - 1;
      moves.0.push(Move { from: Square(to - 8), to: Square(to), kind: MoveType::Quiet });
    }

    // double push
    let mut targets = (single_push << 8) & !occupied & RANK_4;
    while targets != 0 {
      let to = targets.trailing_zeros() as u8;
      targets &= targets - 1;
      moves.0.push(Move { from: Square(to - 16), to: Square(to), kind: MoveType::DoublePush });
    }

    // captures (non-promotion)
    let cap_left  = (pawns << 7) & them & !H_FILE;
    let cap_right = (pawns << 9) & them & !A_FILE;

    let mut targets = cap_left & !RANK_8;
    while targets != 0 {
      let to = targets.trailing_zeros() as u8;
      targets &= targets - 1;
      moves.0.push(Move { from: Square(to - 7), to: Square(to), kind: MoveType::Capture });
    }

    let mut targets = cap_right & !RANK_8;
    while targets != 0 {
      let to = targets.trailing_zeros() as u8;
      targets &= targets - 1;
      moves.0.push(Move { from: Square(to - 9), to: Square(to), kind: MoveType::Capture });
    }

    // promotions (push + captures landing on rank 8)
    let promo_targets = (single_push & RANK_8)
      | (cap_left & RANK_8)
      | (cap_right & RANK_8);

    let mut targets = promo_targets;
    while targets != 0 {
      let to = targets.trailing_zeros() as u8;
      targets &= targets - 1;
      let bb = 1u64 << to;
      let from = if bb & single_push != 0 { to - 8 }
                 else if bb & cap_left != 0  { to - 7 }
                 else                        { to - 9 };
      for piece in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
        moves.0.push(Move { from: Square(from), to: Square(to), kind: MoveType::Promotion(piece) });
      }
    }

    // enpassant
    if board.ep_square != NO_SQUARE {
      let ep_bb = board.ep_square.bitboard();

      let mut attackers = ((ep_bb >> 9) & pawns & !H_FILE)
        | ((ep_bb >> 7) & pawns & !A_FILE);
      while attackers != 0 {
        let from = attackers.trailing_zeros() as u8;
        attackers &= attackers - 1;
        moves.0.push(Move { from: Square(from), to: board.ep_square, kind: MoveType::EnPassant });
      }
    }
  } else {
    let single_push = (pawns >> 8) & !occupied;

    // single push (non-promotion)
    let mut targets = single_push & !RANK_1;
    while targets != 0 {
      let to = targets.trailing_zeros() as u8;
      targets &= targets - 1;
      moves.0.push(Move { from: Square(to + 8), to: Square(to), kind: MoveType::Quiet });
    }

    // double push
    let mut targets = (single_push >> 8) & !occupied & RANK_5;
    while targets != 0 {
      let to = targets.trailing_zeros() as u8;
      targets &= targets - 1;
      moves.0.push(Move { from: Square(to + 16), to: Square(to), kind: MoveType::DoublePush });
    }

    // captures (non-promotion)
    let cap_left  = (pawns >> 9) & them & !H_FILE;
    let cap_right = (pawns >> 7) & them & !A_FILE;

    let mut targets = cap_left & !RANK_1;
    while targets != 0 {
      let to = targets.trailing_zeros() as u8;
      targets &= targets - 1;
      moves.0.push(Move { from: Square(to + 9), to: Square(to), kind: MoveType::Capture });
    }

    let mut targets = cap_right & !RANK_1;
    while targets != 0 {
      let to = targets.trailing_zeros() as u8;
      targets &= targets - 1;
      moves.0.push(Move { from: Square(to + 7), to: Square(to), kind: MoveType::Capture });
    }

    // promotions (push + captures landing on rank 1)
    let promo_targets = (single_push & RANK_1)
      | (cap_left & RANK_1)
      | (cap_right & RANK_1);

    let mut targets = promo_targets;
    while targets != 0 {
      let to = targets.trailing_zeros() as u8;
      targets &= targets - 1;
      let bb = 1u64 << to;
      let from = if bb & single_push != 0 { to + 8 }
                 else if bb & cap_left != 0  { to + 9 }
                 else                        { to + 7 };
      for piece in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
        moves.0.push(Move { from: Square(from), to: Square(to), kind: MoveType::Promotion(piece) });
      }
    }

    // en passant
    if board.ep_square != NO_SQUARE {
      let ep_bb = board.ep_square.bitboard();
      let mut attackers = ((ep_bb << 9) & pawns & !A_FILE)
        | ((ep_bb << 7) & pawns & !H_FILE);
      while attackers != 0 {
        let from = attackers.trailing_zeros() as u8;
        attackers &= attackers - 1;
        moves.0.push(Move { from: Square(from), to: board.ep_square, kind: MoveType::EnPassant });
      }
    }
  }

  moves
}

pub fn knight_attacks(from: u8) -> u64 {
  let knights_offsets: Vec<(i8, i8)> = vec![(-2,-1), (-2,1), (2,-1), (2,1), (1,-2), (1,2), (-1,-2), (-1,2)];
  let sq = Square(from);
  let mut result: u64 = 0;

  for (dr, df) in &knights_offsets {
    let r = sq.rank() as i8 + dr;
    let f = sq.file() as i8 + df;

    if Square::is_valid(r as u8, f as u8) {
      result |= Square::from_rank_file(r as u8, f as u8).bitboard();
    }
  }

  result
}

pub fn bishop_attacks(from: u8, occupied: u64) -> u64 {
  let bishops_dir: Vec<(i8, i8)> = vec![(1,1), (-1,-1), (-1,1), (1,-1)];
  let sq = Square(from);

  let mut result: u64 = 0;

  for (dr, df) in &bishops_dir {
    let mut r = sq.rank() as i8;
    let mut f = sq.file() as i8;

    loop {
      r+=dr;
      f+=df;

      if Square::is_valid(r as u8, f as u8) {
        let bb = Square::from_rank_file(r as u8, f as u8).bitboard();
        result |= bb;
        if bb & occupied != 0 {
          break;
        }
      } else {
        break;
      }
    }
  }

  result
}

pub fn rook_attacks(from: u8, occupied: u64) -> u64 {
  let rooks_dir: Vec<(i8, i8)> = vec![(1,0), (-1,0), (0,1), (0,-1)];
  let sq = Square(from);

  let mut result: u64 = 0;

  for (dr, df) in &rooks_dir {
    let mut r = sq.rank() as i8;
    let mut f = sq.file() as i8;
    loop {
      r+=dr;
      f+=df;

      if Square::is_valid(r as u8, f as u8) {
        let bb = Square::from_rank_file(r as u8, f as u8).bitboard();
        result |= bb;
        if bb & occupied != 0 {
          break;
        }
      } else {
        break;
      }
    }
  }

  result
}

pub fn king_attacks(from: u8) -> u64 {
  let king_dir: Vec<(i8, i8)> = vec![(1,0), (-1,0), (0,1), (0,-1), (1,1), (-1,-1), (-1,1), (1,-1)];
  let sq = Square(from);

  let mut result: u64 = 0;

  for (dr, df) in &king_dir {
    let mut r = sq.rank() as i8 + dr;
    let mut f = sq.file() as i8 + df;

    if Square::is_valid(r as u8, f as u8) {
      let bb = Square::from_rank_file(r as u8, f as u8).bitboard();
      result |= bb;
    }
  }

  result
}

pub fn is_attacked(board: &Board, sq: Square, by_white: bool) -> bool {
  false
}


#[cfg(test)]
mod tests {
  use super::*;

  fn count_moves(fen: &str) -> usize {
    let board = Board::from_fen(fen).expect("valid fen");
    generate_moves(&board).0.len()
  }

  fn has_move(fen: &str, from: &str, to: &str) -> bool {
    let board = Board::from_fen(fen).expect("valid fen");
    generate_moves(&board).0.iter().any(|mv| {
      mv.from.to_algebraic() == from && mv.to.to_algebraic() == to
    })
  }

  fn has_move_kind(fen: &str, from: &str, to: &str, kind: &str) -> bool {
    let board = Board::from_fen(fen).expect("valid fen");
    generate_moves(&board).0.iter().any(|mv| {
      mv.from.to_algebraic() == from && mv.to.to_algebraic() == to && match &mv.kind {
        MoveType::Quiet      => kind == "quiet",
        MoveType::Capture    => kind == "capture",
        MoveType::DoublePush => kind == "double",
        MoveType::EnPassant  => kind == "ep",
        MoveType::Promotion(_) => kind == "promo",
        MoveType::Castle     => kind == "castle",
      }
    })
  }

  // --- startpos ---

  #[test]
  fn startpos_has_20_moves() {
    assert_eq!(count_moves("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"), 20);
  }

  // --- knights ---

  #[test]
  fn knight_center_has_8_moves() {
    // lone white knight on e4, no other pieces
    assert_eq!(count_moves("8/8/8/8/4N3/8/8/8 w - - 0 1"), 8);
  }

  #[test]
  fn knight_corner_has_2_moves() {
    assert_eq!(count_moves("N7/8/8/8/8/8/8/8 w - - 0 1"), 2);
  }

  // --- bishops ---

  #[test]
  fn bishop_center_has_13_moves() {
    assert_eq!(count_moves("8/8/8/8/4B3/8/8/8 w - - 0 1"), 13);
  }

  #[test]
  fn bishop_blocked_by_own_pieces() {
    // bishop on e4, own pawns on d3, f3, d5, f5 — all bishop diagonals blocked immediately
    let board = Board::from_fen("8/8/8/3P1P2/4B3/3P1P2/8/8 w - - 0 1").expect("valid fen");
    let moves = generate_moves(&board);
    let bishop_moves: Vec<_> = moves.0.iter().filter(|mv| mv.from.to_algebraic() == "e4").collect();
    assert_eq!(bishop_moves.len(), 0);
  }

  // --- rooks ---

  #[test]
  fn rook_center_has_14_moves() {
    assert_eq!(count_moves("8/8/8/8/4R3/8/8/8 w - - 0 1"), 14);
  }

  #[test]
  fn rook_captures_enemy() {
    assert_eq!(has_move_kind("8/8/8/8/4Rr2/8/8/8 w - - 0 1", "e4", "f4", "capture"), true);
  }

  // --- queens ---

  #[test]
  fn queen_center_has_27_moves() {
    assert_eq!(count_moves("8/8/8/8/4Q3/8/8/8 w - - 0 1"), 27);
  }

  // --- king ---

  #[test]
  fn king_center_has_8_moves() {
    assert_eq!(count_moves("8/8/8/8/4K3/8/8/8 w - - 0 1"), 8);
  }

  #[test]
  fn king_corner_has_3_moves() {
    assert_eq!(count_moves("K7/8/8/8/8/8/8/8 w - - 0 1"), 3);
  }

  // --- pawns ---

  #[test]
  fn pawn_startpos_double_push() {
    assert_eq!(has_move_kind("8/8/8/8/8/8/4P3/8 w - - 0 1", "e2", "e4", "double"), true);
  }

  #[test]
  fn pawn_not_on_rank2_no_double_push() {
    assert_eq!(has_move_kind("8/8/8/8/8/4P3/8/8 w - - 0 1", "e3", "e5", "double"), false);
  }

  #[test]
  fn pawn_capture() {
    assert_eq!(has_move_kind("8/8/8/8/8/3p4/4P3/8 w - - 0 1", "e2", "d3", "capture"), true);
  }

  #[test]
  fn pawn_blocked_cannot_push() {
    assert_eq!(has_move("8/8/8/8/8/4p3/4P3/8 w - - 0 1", "e2", "e3"), false);
  }

  #[test]
  fn pawn_promotion_emits_4_moves() {
    let board = Board::from_fen("8/4P3/8/8/8/8/8/8 w - - 0 1").expect("valid fen");
    let moves = generate_moves(&board);
    let promos: Vec<_> = moves.0.iter().filter(|mv| {
      mv.from.to_algebraic() == "e7" && matches!(mv.kind, MoveType::Promotion(_))
    }).collect();
    assert_eq!(promos.len(), 4);
  }

  #[test]
  fn en_passant_move_exists() {
    // white pawn on e5, black just pushed d7-d5, ep square is d6
    assert_eq!(has_move_kind("8/8/8/3pP3/8/8/8/8 w - d6 0 1", "e5", "d6", "ep"), true);
  }

  #[test]
  fn black_pawn_moves() {
    // black pawn on e7, white to move = false
    let board = Board::from_fen("8/4p3/8/8/8/8/8/8 b - - 0 1").expect("valid fen");
    let moves = generate_moves(&board);
    assert_eq!(moves.0.len(), 2); // e6 and e5
  }
}