pub struct Board {
    pub pieces: [[i32; 8]; 8],
    pub turn: bool,
}

impl Board {
    pub fn new(pieces: [[i32; 8]; 8]) -> Board {
        Board { pieces, turn: true }
    }
}
