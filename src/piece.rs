pub static NONE: u8 = 0;
pub static PAWN: u8 = 1;
pub static BISHOP: u8 = 2;
pub static KNIGHT: u8 = 3;
pub static ROOK: u8 = 4;
pub static QUEEN: u8 = 5;
pub static KING: u8 = 6;

pub static WHITE_TEAM: u8 = 8;
pub static BLACK_TEAM: u8 = 16;

// to extract the first 3 bits you can do [num] % [2^(3)]
