use std::rc::Rc;

use crate::global_data::TConfig;

macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<i32> for $name {
            type Error = ();

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

back_to_enum! {
    #[derive(Clone, Copy)]
    pub enum Color {
        BlackFG = 30,
        RedFG = 31,
        GreenFG = 32,
        YellowFG = 33,
        BlueFG = 34,
        MagentaFG = 35,
        CyanFG = 36,
        WhiteFG = 37,
        BlackBG = 40,
        RedBG = 41,
        GreenBG = 42,
        YellowBG = 43,
        BlueBG = 44,
        MagentaBG = 45,
        CyanBG = 46,
        WhiteBG = 47,
        Reset = 0,
    }
}

pub struct TChar {
    ch: char,
    fg: Color,
    bg: Color,
}

impl TChar {
    pub fn new(ch: char, fg: Color, bg: Color) -> Self {
        Self { ch, fg, bg }
    }
}

pub struct CharGrid {
    grid: Vec<Vec<TChar>>,
    CONFIG: Rc<TConfig>,
}

impl CharGrid {
    pub fn new(config: Rc<TConfig>) -> Self {
        CharGrid {
            grid: vec![],
            CONFIG: config,
        }
    }

    pub fn width(&self) -> usize {
        self.grid.first().unwrap_or(&(vec![] as Vec<TChar>)).len()
    }
    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn resize_clear(&mut self, w: i32, h: i32) {
        self.grid = vec![];
        for _ in 0..h {
            let mut p = vec![];
            for _ in 0..w {
                p.push(TChar {
                    ch: ' ',
                    fg: (*self.CONFIG)
                        .get(&"theme_foreground".to_string())
                        .expect("Couldn't resolve theme_foreground")
                        .get_color(),
                    bg: (*self.CONFIG)
                        .get(&"theme_background".to_string())
                        .expect("Couldn't resolve theme_background")
                        .get_color(),
                })
            }
            self.grid.push(p);
        }
    }

    pub fn set_char(&mut self, ch: TChar, x: usize, y: usize) {
        self.grid[y][x] = ch;
    }

    pub fn write_line(&mut self, str: String, fg: Color, bg: Color, x: usize, y: usize) {
        for (i, ch) in str.chars().enumerate() {
            self.set_char(TChar { ch, fg, bg }, x + i, y);
        }
    }

    pub fn combine(&mut self, pos: &(i32, i32), grid: CharGrid) {
        for (i, row) in grid.grid.into_iter().enumerate() {
            for (j, tch) in row.into_iter().enumerate() {
                self.set_char(tch, pos.0 as usize + j, pos.1 as usize + i);
            }
        }
    }
}

impl ToString for CharGrid {
    fn to_string(&self) -> String {
        let mut out = "".to_string();
        for row in &self.grid {
            for ch in row {
                out += format!("\x1b[{};{}m{}", ch.bg as i32, ch.fg as i32, ch.ch).as_str();
            }
            out += "\n";
        }
        out
    }
}
