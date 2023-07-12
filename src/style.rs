#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
    Reset,
}

impl Color {
    pub fn fg_code(&self) -> u8 {
        use Color::*;

        match self {
            Black => 30,
            Red  => 31,
            Green => 32,
            Yellow => 33,
            Blue => 34,
            Magenta => 35,
            Cyan => 36,
            White => 37,
            Default => 39,
            Reset => 0,
        }
    }

    pub fn bg_code(&self) -> u8 {
        use Color::*;

        match self {
            Black => 40,
            Red  => 41,
            Green => 42,
            Yellow => 43,
            Blue => 44,
            Magenta => 45,
            Cyan => 46,
            White => 47,
            Default => 49,
            Reset => 0,
        }
    }
}

pub struct ExtendedColor(pub u8); 
