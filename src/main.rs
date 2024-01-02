use std::collections::HashMap;

mod bit_boards;
mod board;

fn main() {
    let game = Game::new();
    game.print_board();
}

struct Game {
    board: HashMap<board::Piece, u64>,
}

impl Game {
    fn new() -> Game {
        Game {
            board: HashMap::from([
                (board::Piece::WhiteKnight, bit_boards::WHITE_KNIGHTS),
                (board::Piece::BlackKnight, bit_boards::BLACK_KNIGHTS),
                (board::Piece::WhiteBishop, bit_boards::WHITE_BISHOPS),
                (board::Piece::BlackBishop, bit_boards::BLACK_BISHOPS),
                (board::Piece::WhiteRook, bit_boards::WHITE_ROOKS),
                (board::Piece::BlackRook, bit_boards::BLACK_ROOKS),
                (board::Piece::WhiteQueen, bit_boards::WHITE_QUEENS),
                (board::Piece::BlackQueen, bit_boards::BLACK_QUEENS),
                (board::Piece::WhiteKing, bit_boards::WHITE_KING),
                (board::Piece::BlackKing, bit_boards::BLACK_KING),
                (board::Piece::WhitePawn, bit_boards::WHITE_PAWNS),
                (board::Piece::BlackPawn, bit_boards::BLACK_PAWNS),
            ]),
        }
    }

    fn print_board(&self) {
        for i in 0..board::BOARD_LEN {
            for j in 0..board::BOARD_LEN {
                let mut found = false;
                for (piece, board) in &self.board {
                    if board & (1 << (i * board::BOARD_LEN + j)) != 0 {
                        match piece {
                            board::Piece::WhiteKnight => print!("♞ "),
                            board::Piece::BlackKnight => print!("♘ "),
                            board::Piece::WhiteBishop => print!("♝ "),
                            board::Piece::BlackBishop => print!("♗ "),
                            board::Piece::WhiteRook => print!("♜ "),
                            board::Piece::BlackRook => print!("♖ "),
                            board::Piece::WhiteQueen => print!("♛ "),
                            board::Piece::BlackQueen => print!("♕ "),
                            board::Piece::WhiteKing => print!("♚ "),
                            board::Piece::BlackKing => print!("♔ "),
                            board::Piece::WhitePawn => print!("♟ "),
                            board::Piece::BlackPawn => print!("♙ "),
                        }
                        found = true;
                        break;
                    }
                }
                if !found {
                    print!("_ ");
                }
            }
            println!();
        }
    }
}
