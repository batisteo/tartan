use crate::tartan_register_shades::SHADES;
use core::iter::repeat;
use std::collections::HashMap;

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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Shade {
    pub colour: Colour,
    pub tone: Tone,
    pub rgb: [u8; 3],
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tone {
    Light,
    Medium,
    Dark,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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
            "N" => Some(Grey), // Neutral
            "K" => Some(Black),
            "Bk" => Some(Black),
            "T" => Some(Brown), // Tan
            _ => None,
        }
    }

    pub fn to_array(&self, palette: &Palette) -> [u8; 3] {
        if let Some(shade) = palette.shades.get(self) {
            shade.rgb
        } else {
            [0, 255, 0]
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Colourway {
    Modern,
    Ancient,
    Reproduction,
    Weathered, // Withered
    Muted,
    Standard,
}
impl Default for Colourway {
    fn default() -> Self {
        Colourway::Modern
    }
}

#[derive(Debug, PartialEq)]
pub enum Variation {
    Hunting,
    Dress,
}

#[derive(Debug)]
pub struct Palette {
    pub colourway: Option<Colourway>,
    pub variation: Option<Variation>,
    pub shades: HashMap<Colour, Shade>,
}

impl Default for Palette {
    fn default() -> Self {
        use Colour::*;

        let shades = HashMap::from([
            (Red, SHADES[7]),
            (Orange, SHADES[20]),
            (Yellow, SHADES[32]),
            (Green, SHADES[62]),
            (Blue, SHADES[89]),
            (Purple, SHADES[99]),
            (White, SHADES[106]),
            (Grey, SHADES[113]),
            (Black, SHADES[119]),
            (Brown, SHADES[131]),
        ]);
        Palette {
            colourway: Some(Colourway::default()),
            variation: None,
            shades,
        }
    }
}
