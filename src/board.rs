pub struct Board {
    pub pieces: [[u8; 8]; 8],
    pub turn: bool,
}

impl Board {
    pub fn new(pieces: [[u8; 8]; 8]) -> Board {
        Board { pieces }
    }
}
