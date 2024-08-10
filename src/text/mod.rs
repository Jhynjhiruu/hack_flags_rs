use crate::vi::{Vi, HEIGHT, WIDTH};

mod font;

use font::*;

#[derive(Clone, Copy)]
pub struct Colour(u32);

impl const From<u32> for Colour {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl const Into<u32> for Colour {
    fn into(self) -> u32 {
        self.0
    }
}

/*macro_rules! define_colours {
    ($($name:ident, $r:expr, $g:expr, $b:expr, $a:expr),*) => {
        enum Colour {
            $(define_colour!($name, $r, $g, $b, $a)),
        }
    };
}

define_colours!(
WHITE, 0xFF, 0xFF, 0xFF, 0xFF,
BLACK, 0, 0, 0, 0,
GREY, 0x7F, 0x7F, 0x7F, 0xFF,
BLUE, 0, 0, 0xFF, 0xFF,
GREEN, 0, 0xFF, 0, 0xFF,
RED, 0xFF, 0, 0, 0xFF,
YELLOW, 0xFF, 0xFF, 0, 0xFF,
DARK, 0x32, 0x32, 0x80, 0xFF,
MARK_COLOUR, 0x32, 0xFA, 0x32, 0xFF);*/

macro_rules! define_colour {
    ($name:ident, $r:expr, $g:expr, $b:expr, $a:expr) => {
        pub const $name: Colour = Colour::from_rgba($r, $g, $b, $a);
    };
}

impl Colour {
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        /*Self(
            ((r * 31 / 255) as u16) << 11
                | ((g * 31 / 255) as u16) << 6
                | ((b * 31 / 255) as u16) << 1
                | ((a * 1 / 255) as u16),
        )*/
        Self(((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | ((a as u32) << 0))
    }

    define_colour!(WHITE, 0xFF, 0xFF, 0xFF, 0xFF);
    define_colour!(BLACK, 0, 0, 0, 0);
    define_colour!(GREY, 0x7F, 0x7F, 0x7F, 0xFF);
    define_colour!(BLUE, 0, 0, 0xFF, 0xFF);
    define_colour!(GREEN, 0, 0xFF, 0, 0xFF);
    define_colour!(RED, 0xFF, 0, 0, 0xFF);
    define_colour!(YELLOW, 0xFF, 0xFF, 0, 0xFF);
    define_colour!(DARK, 0x32, 0x32, 0x80, 0xFF);
    define_colour!(MARK_COLOUR, 0x32, 0xFA, 0x32, 0xFF);
}

impl Vi {
    pub fn print_char(&mut self, x: usize, y: usize, col: Colour, mut ch: u8) {
        if !(b' '..=b'~').contains(&ch) {
            ch = b'?';
        }

        ch -= 0x20;

        let data = &FONT[ch as usize];

        let fbuf = self.get_next_framebuffer();

        let x = x * CHAR_WIDTH;
        let y = y * CHAR_HEIGHT;

        for dy in 0..CHAR_HEIGHT {
            let y = y + dy;
            if y >= HEIGHT {
                break;
            }

            for dx in 0..CHAR_WIDTH {
                let x = x + dx;
                if x >= WIDTH {
                    break;
                }

                let index = y * WIDTH + x;

                let bit = ((data[dy] >> (CHAR_WIDTH - dx - 1)) & 1) != 0;

                fbuf[index] = if bit { col } else { Colour::BLACK }.into();
            }
        }
    }

    pub fn print_nibble(&mut self, x: usize, y: usize, col: Colour, val: u8) {
        let nibble_to_char = |n| match n {
            0x00..=0x09 => n + b'0',
            0x0A..=0x0F => (n - 0x0A) + b'A',
            _ => b'?',
        };

        self.print_char(x, y, col, nibble_to_char(val));
    }

    pub fn print_u8(&mut self, x: usize, y: usize, col: Colour, val: u8) {
        self.print_nibble(x, y, col, val >> 4);
        self.print_nibble(x + 1, y, col, val & 0x0F);
    }

    pub fn print_i8(&mut self, x: usize, y: usize, col: Colour, val: i8) {
        let (ch, val) = if val < 0 {
            (b'-', (!(val as u8)).wrapping_add(1))
        } else {
            (b' ', val as u8)
        };
        self.print_char(x, y, col, ch);

        self.print_u8(x + 1, y, col, val);
    }

    pub fn print_u16(&mut self, x: usize, y: usize, col: Colour, val: u16) {
        self.print_u8(x, y, col, (val >> 8) as _);
        self.print_u8(x + 2, y, col, val as u8);
    }

    pub fn print_string(&mut self, x: usize, y: usize, col: Colour, string: &str) {
        for (index, ch) in string.bytes().enumerate() {
            self.print_char(x + index, y, col, ch);
        }
    }
}
