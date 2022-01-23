use crate::colours::{Colour, Shade, Tone};

lazy_static! {
    pub static ref SHADES: Vec<Shade> = vec![
        Shade {
            tone: Tone::Light,
            colour: Colour::Red,
            rgb: [232, 204, 184], // #e8ccb8
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Red,
            rgb: [232, 120, 120], // #e87878
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Red,
            rgb: [236, 52, 196], // #ec34c4
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Red,
            rgb: [160, 0, 72], // #a00048
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Red,
            rgb: [250, 75, 0], // #fa4b00
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Red,
            rgb: [255, 0, 0], // #ff0000
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Red,
            rgb: [220, 0, 0], // #dc0000
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Red,
            rgb: [200, 0, 0], // #c80000
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Red,
            rgb: [200, 40, 40], // #c82828
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Red,
            rgb: [200, 0, 44], // #c8002c
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Red,
            rgb: [176, 48, 0], // #b03000
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Red,
            rgb: [160, 0, 0], // #a00000
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Red,
            rgb: [150, 0, 0], // #960000
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Red,
            rgb: [150, 0, 40], // #960028
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Red,
            rgb: [136, 0, 0], // #880000
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Red,
            rgb: [128, 0, 40], // #800028
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Red,
            rgb: [120, 28, 56], // #781c38
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Red,
            rgb: [76, 0, 0], // #4c0000
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Red,
            rgb: [144, 28, 56], // #901c38
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Red,
            rgb: [104, 0, 40], // #680028
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Orange,
            rgb: [236, 128, 72], // #ec8048
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Orange,
            rgb: [232, 96, 0], // #e86000
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Orange,
            rgb: [220, 148, 60], // #dc943c
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Orange,
            rgb: [216, 124, 0], // #d87c00
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Orange,
            rgb: [190, 120, 50], // #be7832
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Orange,
            rgb: [184, 76, 0], // #b84c00
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Yellow,
            rgb: [248, 244, 208], // #f8f4d0
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Yellow,
            rgb: [249, 245, 200], // #f9f5c8
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Yellow,
            rgb: [248, 227, 140], // #f8e38c
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Yellow,
            rgb: [255, 255, 0], // #ffff00
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Yellow,
            rgb: [255, 230, 0], // #ffe600
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Yellow,
            rgb: [255, 215, 0], // #ffd700
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Yellow,
            rgb: [252, 204, 0], // #fccc00
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Yellow,
            rgb: [224, 161, 38], // #e0a126
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Yellow,
            rgb: [232, 192, 0], // #e8c000
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Yellow,
            rgb: [216, 176, 0], // #d8b000
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Yellow,
            rgb: [188, 140, 0], // #bc8c00
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Yellow,
            rgb: [208, 152, 0], // #d09800
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Yellow,
            rgb: [200, 152, 0], // #c89800
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Yellow,
            rgb: [200, 140, 0], // #c88c00
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Green,
            rgb: [120, 148, 132], // #789484
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Green,
            rgb: [196, 188, 104], // #c4bc68
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Green,
            rgb: [156, 156, 0], // #9c9c00
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Green,
            rgb: [134, 198, 124], // #86c67c
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Green,
            rgb: [100, 152, 72], // #649848
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            rgb: [0, 139, 0], // #008b00
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            rgb: [64, 128, 96], // #408060
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            rgb: [40, 156, 24], // #289c18
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            rgb: [0, 100, 0], // #006400
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            rgb: [0, 120, 0], // #007800
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            rgb: [112, 128, 72], // #767e52
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            rgb: [92, 100, 40], // #5c6428
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            rgb: [0, 100, 60], // #00643c
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            rgb: [20, 100, 0], // #146400
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            rgb: [0, 104, 24], // #006818
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            rgb: [0, 76, 0], // #004c00
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            rgb: [40, 88, 0], // #285800
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            rgb: [0, 80, 32], // #005020
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Green,
            rgb: [0, 84, 72], // #005448
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Green,
            rgb: [0, 60, 20], // #003c14
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Green,
            rgb: [0, 56, 32], // #003820
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Green,
            rgb: [0, 64, 40], // #004028
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Green,
            rgb: [0, 40, 20], // #002814
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Blue,
            rgb: [152, 200, 232], // #98c8e8
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Blue,
            rgb: [0, 252, 252], // #00fcfc
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Blue,
            rgb: [130, 207, 253], // #82cffd
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            rgb: [4, 136, 136], // #048888
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            rgb: [60, 130, 175], // #3c82af
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            rgb: [92, 40, 168], // #5c8ca8
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            rgb: [40, 136, 196], // #2888c4
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            rgb: [72, 164, 192], // #48a4c0
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            rgb: [36, 116, 232], // #2474e8
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            rgb: [5, 150, 250], // #0596fa
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            rgb: [52, 116, 252], // #3474fc
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            rgb: [0, 0, 255], // #0000ff
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            rgb: [56, 80, 200], // #3850c8
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            rgb: [120, 140, 180], // #788cb4
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            rgb: [95, 116, 156], // #5f749c
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            rgb: [24, 112, 164], // #1870a4
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            rgb: [20, 116, 180], // #1474b4
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            rgb: [0, 0, 205], // #0000cd
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Blue,
            rgb: [44, 64, 132], // #2c4084
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            rgb: [0, 60, 100], // #003c64
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            rgb: [0, 0, 140], // #00008c
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            rgb: [0, 0, 128], // #000080
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            rgb: [44, 44, 128], // #2c2c80
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            rgb: [28, 0, 112], // #1c0070
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            rgb: [0, 0, 100], // #000064
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            rgb: [32, 32, 96], // #202060
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            rgb: [0, 0, 72], // #000048
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            rgb: [20, 30, 70], // #141e46
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Blue,
            rgb: [28, 28, 80], // #1c1c50
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Purple,
            rgb: [168, 172, 232], // #a8ace8
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Purple,
            rgb: [196, 156, 216], // #c49cd8
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Purple,
            rgb: [156, 104, 164], // #9c68a4
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Purple,
            rgb: [144, 88, 216], // #9058d8
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Purple,
            rgb: [170, 0, 255], // #aa00ff
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Purple,
            rgb: [180, 88, 172], // #b458ac
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Purple,
            rgb: [108, 0, 112], // #6c0070
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Purple,
            rgb: [90, 0, 140], // #5a008c
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Purple,
            rgb: [100, 0, 140], // #64008c
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Purple,
            rgb: [120, 0, 120], // #780078
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Purple,
            rgb: [68, 0, 68], // #440044
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::White,
            rgb: [240, 224, 200], // #f0e0c8
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::White,
            rgb: [252, 252, 252], // #fcfcfc
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::White,
            rgb: [255, 255, 255], // #ffffff
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::White,
            rgb: [248, 248, 248], // #f8f8f8
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Grey,
            rgb: [224, 224, 224], // #e0e0e0
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Grey,
            rgb: [200, 200, 200], // #c8c8c8
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Grey,
            rgb: [192, 192, 192], // #c0c0c0
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Grey,
            rgb: [176, 176, 176], // #b0b0b0
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Grey,
            rgb: [184, 184, 184], // #b8b8b8
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Grey,
            rgb: [160, 160, 160], // #a0a0a0
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Grey,
            rgb: [128, 128, 128], // #808080
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Grey,
            rgb: [136, 136, 136], // #888888
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Grey,
            rgb: [100, 100, 100], // #646464
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Grey,
            rgb: [80, 80, 80], // #505050
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Grey,
            rgb: [92, 92, 92], // #5c5c5c
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Grey,
            rgb: [20, 40, 60], // #14283c
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Grey,
            rgb: [28, 23, 20], // #1c1714
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Grey,
            rgb: [28, 28, 28], // #1c1c1c
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Black,
            rgb: [16, 16, 16], // #101010
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Black,
            rgb: [0, 0, 0], // #000000
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Brown,
            rgb: [160, 136, 88], // #a08858
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Brown,
            rgb: [140, 112, 56], // #8c7038
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Brown,
            rgb: [160, 124, 88], // #a07c58
        },
        Shade {
            tone: Tone::Light,
            colour: Colour::Brown,
            rgb: [176, 116, 48], // #b07430
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Brown,
            rgb: [152, 72, 28], // #98481c
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Brown,
            rgb: [96, 56, 0], // #603800
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Brown,
            rgb: [96, 64, 0], // #604000
        },
        Shade {
            tone: Tone::Medium,
            colour: Colour::Brown,
            rgb: [80, 60, 20], // #503c14
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Brown,
            rgb: [76, 52, 40], // #4c3428
        },
        Shade {
            tone: Tone::Dark,
            colour: Colour::Brown,
            rgb: [68, 24, 0], // #441800
        },
    ];
}
