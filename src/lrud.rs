use std::fmt::{Formatter, Display};

#[derive(Debug, Copy, Clone)]
pub enum LRUD {
    Left,
    Right,
    Up,
    Down,
}

impl Display for LRUD {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            LRUD::Left => write!(f, "Left"),
            LRUD::Right => write!(f, "Right"),
            LRUD::Up => write!(f, "Up"),
            LRUD::Down => write!(f, "Down"),
        }
    }
}