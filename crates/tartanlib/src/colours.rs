use core::iter::repeat;
use hex::FromHex;

#[derive(Debug, PartialEq)]
pub enum Pivot {
    True,
    False,
    Maybe,
}

#[derive(Debug, PartialEq)]
pub struct Span {
    pub colour: Colour,
    pub count: usize,
    pub pivot: Pivot,
}

impl Span {
    pub fn new(colour: &str, count: &str) -> Self {
        Self {
            colour: Colour::get(colour).unwrap(),
            count: count.parse::<usize>().unwrap(),
            pivot: Pivot::Maybe,
        }
    }
    pub fn pivot(colour: &str, count: &str) -> Self {
        Self {
            colour: Colour::get(colour).unwrap(),
            count: count.parse::<usize>().unwrap(),
            pivot: Pivot::True,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Weft {
    Identical,
    Different,
}

#[derive(Debug, PartialEq)]
pub struct Sett {
    pub pattern: Vec<Span>,
    pub symmetric: bool,
    pub weft: Weft,
}

impl Sett {
    pub fn new() -> Self {
        Self {
            pattern: vec![],
            symmetric: true,
            weft: Weft::Identical,
        }
    }

    /// Infinite iterator that goes over the colour spans and back.
    pub fn shuttle(&self) -> impl Iterator<Item = &Span> {
        let len = self.pattern.len();
        self.pattern
            .iter()
            .chain(self.pattern.iter().rev().skip(1).take(len - 2))
            .cycle()
    }

    pub fn colours(&self, skip: usize) -> impl Iterator<Item = &Colour> {
        self.shuttle()
            .skip(skip)
            .flat_map(|span| repeat(&span.colour).take(span.count))
    }

    pub fn coord_colours(
        &self,
        width: u32,
        height: u32,
        skip: usize,
    ) -> impl Iterator<Item = (u32, u32, &Colour)> {
        (0..height).flat_map(move |y| {
            (0..width).zip(self.colours(skip)).map(move |(x, colour)| {
                if (x + y) / 2 % 2 == 0 {
                    (x, y, colour)
                } else {
                    (y, x, colour)
                }
            })
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Shade {
    pub colour: Colour,
    pub tone: Tone,
    pub hex: String,
    pub rgb: [u8; 3],
}

#[derive(Debug, PartialEq)]
pub enum Tone {
    Light,
    Medium,
    Dark,
}

#[derive(Debug, PartialEq)]
pub enum Colour {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    White,
    Grey,
    Black,
    Brown,
}

impl Colour {
    pub fn get(letter: &str) -> Option<Self> {
        use Colour::*;

        match letter {
            "R" => Some(Red),
            "O" => Some(Orange),
            "Y" => Some(Yellow),
            "G" => Some(Green),
            "B" => Some(Blue),
            "RB" => Some(Blue), // Royal Blue
            "P" => Some(Purple),
            "W" => Some(White),
            "N" => Some(Grey),
            "K" => Some(Black),
            "Bk" => Some(Black),
            "T" => Some(Brown),
            _ => None,
        }
    }

    pub fn to_array(&self) -> [u8; 3] {
        use Colour::*;

        fn rgb(hex: &str) -> [u8; 3] {
            <[u8; 3]>::from_hex(hex).unwrap()
        }

        match self {
            Red => rgb("c80000"),
            Yellow => rgb("c8c800"),
            Green => rgb("006818"),
            Blue => rgb("1c0070"),
            White => rgb("fcfcfc"),
            Black => rgb("111111"),
            Brown => rgb("402000"),
            _ => rgb("111111"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Variation {
    Modern,
    Ancient,
    Hunting,
    Dress,
    Weathered, // Withered // Muted
}
