pub struct Board {
    pub pieces: [[u8; 8]; 8],
}

impl Board {
    pub fn new(pieces: [[u8; 8]; 8]) -> Board {
        Board { pieces }
    }
}
