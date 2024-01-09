mod board;
mod engine;
mod piece;
mod player;

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn create_board() {
        let pieces: [[u8; 8]; 8] = [
            [19, 18, 17, 20, 21, 17, 18, 19],
            [16, 16, 16, 16, 16, 16, 16, 16],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [8, 8, 8, 8, 8, 8, 8, 8],
            [11, 10, 9, 12, 13, 9, 10, 11],
        ];

        let board = Board::new(pieces);

        assert_eq!(board.pieces[0][0] & 7, piece::ROOK);
        assert_eq!(board.pieces[0][0] & 24, piece::BLACK_TEAM);

        assert_eq!(board.pieces[7][4] & 7, piece::KING);
        assert_eq!(board.pieces[7][4] & 24, piece::WHITE_TEAM);

        assert_eq!(board.pieces[7][3] & 7, piece::QUEEN);
        assert_eq!(board.pieces[7][3] & 24, piece::WHITE_TEAM);
    }
}
