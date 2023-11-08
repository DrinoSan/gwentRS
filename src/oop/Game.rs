use super::Interface::Interface;

pub struct Game {
    io: Interface,
}

impl Game {
    pub fn new(io: Interface) -> Self {
        Game { io }
    }
}

