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

lazy_static! {
    pub static ref SHADES: Vec<Shade> = vec![
        Shade {
            tone: Tone::Light,
            colour: Colour::Red,
            hex: "e8ccb8".into(),
            rgb: [232, 204, 184],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Red,
            hex: "e87878".into(),
            rgb: [232, 120, 120],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Red,
            hex: "ec34c4".into(),
            rgb: [236, 52, 196],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Red,
            hex: "a00048".into(),
            rgb: [160, 0, 72],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Red,
            hex: "fa4b00".into(),
            rgb: [250, 75, 0],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Red,
            hex: "ff0000".into(),
            rgb: [255, 0, 0],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Red,
            hex: "dc0000".into(),
            rgb: [220, 0, 0],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Red,
            hex: "c80000".into(),
            rgb: [200, 0, 0],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Red,
            hex: "c82828".into(),
            rgb: [200, 40, 40],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Red,
            hex: "c8002c".into(),
            rgb: [200, 0, 44],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Red,
            hex: "b03000".into(),
            rgb: [176, 48, 0],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Red,
            hex: "a00000".into(),
            rgb: [160, 0, 0],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Red,
            hex: "960000".into(),
            rgb: [150, 0, 0],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Red,
            hex: "960028".into(),
            rgb: [150, 0, 40],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Red,
            hex: "880000".into(),
            rgb: [136, 0, 0],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Red,
            hex: "800028".into(),
            rgb: [128, 0, 40],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Red,
            hex: "781c38".into(),
            rgb: [120, 28, 56],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Red,
            hex: "4c0000".into(),
            rgb: [76, 0, 0],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Red,
            hex: "901c38".into(),
            rgb: [144, 28, 56],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Red,
            hex: "680028".into(),
            rgb: [104, 0, 40],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Orange,
            hex: "ec8048".into(),
            rgb: [236, 128, 72],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Orange,
            hex: "e86000".into(),
            rgb: [232, 96, 0],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Orange,
            hex: "dc943c".into(),
            rgb: [220, 148, 60],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Orange,
            hex: "d87c00".into(),
            rgb: [216, 124, 0],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Orange,
            hex: "be7832".into(),
            rgb: [190, 120, 50],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Orange,
            hex: "b84c00".into(),
            rgb: [184, 76, 0],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Yellow,
            hex: "f8f4d0".into(),
            rgb: [248, 244, 208],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Yellow,
            hex: "f9f5c8".into(),
            rgb: [249, 245, 200],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Yellow,
            hex: "f8e38c".into(),
            rgb: [248, 227, 140],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Yellow,
            hex: "ffff00".into(),
            rgb: [255, 255, 0],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Yellow,
            hex: "ffe600".into(),
            rgb: [255, 230, 0],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Yellow,
            hex: "ffd700".into(),
            rgb: [255, 215, 0],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Yellow,
            hex: "fccc00".into(),
            rgb: [252, 204, 0],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Yellow,
            hex: "e0a126".into(),
            rgb: [224, 161, 38],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Yellow,
            hex: "e8c000".into(),
            rgb: [232, 192, 0],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Yellow,
            hex: "d8b000".into(),
            rgb: [216, 176, 0],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Yellow,
            hex: "bc8c00".into(),
            rgb: [188, 140, 0],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Yellow,
            hex: "d09800".into(),
            rgb: [208, 152, 0],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Yellow,
            hex: "c89800".into(),
            rgb: [200, 152, 0],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Yellow,
            hex: "c88c00".into(),
            rgb: [200, 140, 0],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Green,
            hex: "789484".into(),
            rgb: [120, 148, 132],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Green,
            hex: "c4bc68".into(),
            rgb: [196, 188, 104],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Green,
            hex: "9c9c00".into(),
            rgb: [156, 156, 0],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Green,
            hex: "86c67c".into(),
            rgb: [134, 198, 124],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Green,
            hex: "649848".into(),
            rgb: [100, 152, 72],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            hex: "008b00".into(),
            rgb: [0, 139, 0],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            hex: "408060".into(),
            rgb: [64, 128, 96],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            hex: "289c18".into(),
            rgb: [40, 156, 24],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            hex: "006400".into(),
            rgb: [0, 100, 0],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            hex: "007800".into(),
            rgb: [0, 120, 0],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            hex: "767e52".into(),
            rgb: [112, 128, 72],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            hex: "5c6428".into(),
            rgb: [92, 100, 40],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            hex: "00643c".into(),
            rgb: [0, 100, 60],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            hex: "146400".into(),
            rgb: [20, 100, 0],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            hex: "006818".into(),
            rgb: [0, 104, 24],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            hex: "004c00".into(),
            rgb: [0, 76, 0],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            hex: "285800".into(),
            rgb: [40, 88, 0],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            hex: "005020".into(),
            rgb: [0, 80, 32],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            hex: "005448".into(),
            rgb: [0, 84, 72],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Green,
            hex: "003c14".into(),
            rgb: [0, 60, 20],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Green,
            hex: "003820".into(),
            rgb: [0, 56, 32],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Green,
            hex: "004028".into(),
            rgb: [0, 64, 40],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Green,
            hex: "002814".into(),
            rgb: [0, 40, 20],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Blue,
            hex: "98c8e8".into(),
            rgb: [152, 200, 232],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Blue,
            hex: "00fcfc".into(),
            rgb: [0, 252, 252],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Blue,
            hex: "82cffd".into(),
            rgb: [130, 207, 253],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            hex: "048888".into(),
            rgb: [4, 136, 136],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            hex: "3c82af".into(),
            rgb: [60, 130, 175],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            hex: "5c8ca8".into(),
            rgb: [92, 40, 168],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            hex: "2888c4".into(),
            rgb: [40, 136, 196],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            hex: "48a4c0".into(),
            rgb: [72, 164, 192],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            hex: "2474e8".into(),
            rgb: [36, 116, 232],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            hex: "0596fa".into(),
            rgb: [5, 150, 250],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            hex: "3474fc".into(),
            rgb: [52, 116, 252],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            hex: "0000ff".into(),
            rgb: [0, 0, 255],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            hex: "3850c8".into(),
            rgb: [56, 80, 200],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            hex: "788cb4".into(),
            rgb: [120, 140, 180],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            hex: "5f749c".into(),
            rgb: [95, 116, 156],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            hex: "1870a4".into(),
            rgb: [24, 112, 164],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            hex: "1474b4".into(),
            rgb: [20, 116, 180],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            hex: "0000cd".into(),
            rgb: [0, 0, 205],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            hex: "2c4084".into(),
            rgb: [44, 64, 132],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            hex: "003c64".into(),
            rgb: [0, 60, 100],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            hex: "00008c".into(),
            rgb: [0, 0, 140],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            hex: "000080".into(),
            rgb: [0, 0, 128],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            hex: "2c2c80".into(),
            rgb: [44, 44, 128],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            hex: "1c0070".into(),
            rgb: [28, 0, 112],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            hex: "000064".into(),
            rgb: [0, 0, 100],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            hex: "202060".into(),
            rgb: [32, 32, 96],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            hex: "000048".into(),
            rgb: [0, 0, 72],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            hex: "141e46".into(),
            rgb: [20, 30, 70],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            hex: "1c1c50".into(),
            rgb: [28, 28, 80],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Purple,
            hex: "a8ace8".into(),
            rgb: [168, 172, 232],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Purple,
            hex: "c49cd8".into(),
            rgb: [196, 156, 216],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Purple,
            hex: "9c68a4".into(),
            rgb: [156, 104, 164],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Purple,
            hex: "9058d8".into(),
            rgb: [144, 88, 216],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Purple,
            hex: "aa00ff".into(),
            rgb: [170, 0, 255],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Purple,
            hex: "b458ac".into(),
            rgb: [180, 88, 172],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Purple,
            hex: "6c0070".into(),
            rgb: [108, 0, 112],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Purple,
            hex: "5a008c".into(),
            rgb: [90, 0, 140],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Purple,
            hex: "64008c".into(),
            rgb: [100, 0, 140],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Purple,
            hex: "780078".into(),
            rgb: [120, 0, 120],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Purple,
            hex: "440044".into(),
            rgb: [68, 0, 68],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::White,
            hex: "f0e0c8".into(),
            rgb: [240, 224, 200],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::White,
            hex: "fcfcfc".into(),
            rgb: [252, 252, 252],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::White,
            hex: "ffffff".into(),
            rgb: [255, 255, 255],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::White,
            hex: "f8f8f8".into(),
            rgb: [248, 248, 248],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Grey,
            hex: "e0e0e0".into(),
            rgb: [224, 224, 224],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Grey,
            hex: "c8c8c8".into(),
            rgb: [200, 200, 200],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Grey,
            hex: "c0c0c0".into(),
            rgb: [192, 192, 192],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Grey,
            hex: "b0b0b0".into(),
            rgb: [176, 176, 176],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Grey,
            hex: "b8b8b8".into(),
            rgb: [184, 184, 184],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Grey,
            hex: "a0a0a0".into(),
            rgb: [160, 160, 160],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Grey,
            hex: "808080".into(),
            rgb: [128, 128, 128],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Grey,
            hex: "888888".into(),
            rgb: [136, 136, 136],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Grey,
            hex: "646464".into(),
            rgb: [100, 100, 100],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Grey,
            hex: "505050".into(),
            rgb: [80, 80, 80],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Grey,
            hex: "5c5c5c".into(),
            rgb: [92, 92, 92],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Grey,
            hex: "14283c".into(),
            rgb: [20, 40, 60],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Grey,
            hex: "1c1714".into(),
            rgb: [28, 23, 20],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Grey,
            hex: "1c1c1c".into(),
            rgb: [28, 28, 28],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Black,
            hex: "101010".into(),
            rgb: [16, 16, 16],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Black,
            hex: "000000".into(),
            rgb: [0, 0, 0],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Brown,
            hex: "a08858".into(),
            rgb: [160, 136, 88],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Brown,
            hex: "8c7038".into(),
            rgb: [140, 112, 56],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Brown,
            hex: "a07c58".into(),
            rgb: [160, 124, 88],
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Brown,
            hex: "b07430".into(),
            rgb: [176, 116, 48],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Brown,
            hex: "98481c".into(),
            rgb: [152, 72, 28],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Brown,
            hex: "603800".into(),
            rgb: [96, 56, 0],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Brown,
            hex: "604000".into(),
            rgb: [96, 64, 0],
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Brown,
            hex: "503c14".into(),
            rgb: [80, 60, 20],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Brown,
            hex: "4c3428".into(),
            rgb: [76, 52, 40],
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Brown,
            hex: "441800".into(),
            rgb: [68, 24, 0],
        },
    ];
}
