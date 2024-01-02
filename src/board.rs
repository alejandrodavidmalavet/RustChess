pub const BOARD_LEN: usize = 8;
pub const BOARD_SQUARES: usize = BOARD_LEN * BOARD_LEN;

#[derive(Eq, PartialEq, Hash)]
pub enum Piece {
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteRook,
    BlackRook,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
    WhitePawn,
    BlackPawn,
}
