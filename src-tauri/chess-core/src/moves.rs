use crate::board::Board;
use crate::types::Moves;

pub fn generate_moves(board: &Board) -> Moves {
  let moves = Moves::new();
  let mut us: u64 = 0;
  let mut them: u64 = 0;

  if board.white_to_move {
    us = board.white;
    them = board.black;
  } else {
    us = board.black;
    them = board.white;
  }

  let mut pawns = board.pawns & us;
  let mut knights = board.knights & us;
  let mut rooks = board.rooks & us;
  let mut queens = board.queens & us;
  let mut bishops = board.bishops & us;
  let mut king = board.kings & us;

  while (knights != 0) {
    let square = knights.trailing_zeros();
    knights &= knights - 1;
  }
  moves
}
