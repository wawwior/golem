pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    FRONT,
    BACK,
}

impl Direction {
    pub fn name(&self) -> &str {
        match self {
            Direction::UP => "Up",
            Direction::DOWN => "Down",
            Direction::LEFT => "Left",
            Direction::RIGHT => "Right",
            Direction::FRONT => "Front",
            Direction::BACK => "Back",
        }
    }
}
