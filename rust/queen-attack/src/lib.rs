const BOARD_SIZE: i32 = 8;

#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        ((0..BOARD_SIZE).contains(&rank) && (0..BOARD_SIZE).contains(&file))
            .then(|| Self { rank, file })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let rank_distance = (self.position.rank - other.position.rank).abs();
        let file_distance = (self.position.file - other.position.file).abs();

        rank_distance == 0 || file_distance == 0 || rank_distance == file_distance
    }
}
