use arcade_util::{
    Coord2D,
    Dir2D,
    Rot2D,
    GameStatus,
};

pub struct Snake {
    pub segments: Vec<Coord2D<i32>>,
    direction: Dir2D,
}

impl Snake {
    pub fn new(start_pos: Coord2D<i32>) -> Self {
        Self {
            segments: vec![start_pos],
            direction: Dir2D::Up,
        }
    }

    pub fn move_forward(&mut self) {
        let new_head = self.segments[0] + self.direction.as_coord();
        self.segments.insert(0, new_head);
        self.segments.pop();
    }
}
