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
            LRUD::Left => write!(f, " {:5} ", &"Left"),
            LRUD::Right => write!(f, " {:5} ",  &"Right"),
            LRUD::Up => write!(f, " {:5} ", &"Up"),
            LRUD::Down => write!(f, " {:5} ", &"Down"),
        }
    }
}