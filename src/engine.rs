use crate::board::Board;

struct Engine {
    board: Board,
}

impl Engine {
    fn new(board: Board) -> Engine {
        Engine { board }
    }
}
