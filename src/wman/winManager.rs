use crate::global_data::TConfig;
use std::{collections::HashMap, process, rc::Rc};
use uuid::Uuid;

use super::{
    tChar::{CharGrid, TChar},
    window::Window,
};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use terminal_size::{terminal_size, Height, Width};

pub struct WindowManager {
    pub windows: HashMap<Uuid, Box<dyn Window>>,
    CONFIG: Rc<TConfig>,
    selected_window: Option<Uuid>,
}

fn unbox<T>(value: Box<T>) -> T {
    *value
}

impl WindowManager {
    pub fn new(config: Rc<TConfig>) -> Self {
        WindowManager {
            windows: HashMap::new(),
            CONFIG: config,
            selected_window: None,
        }
    }
}

impl WindowManager {
    pub fn push(&mut self, window: Box<dyn Window>) -> Uuid {
        let id = Uuid::new_v4();
        self.windows.insert(id, window);
        id
    }

    pub fn render(&mut self) -> String {
        let mut grid = CharGrid::new(self.CONFIG.clone());
        // u16 in this format: 0x0000
        // 0x(left)(down)(up)(right)
        let mut connections: Vec<Vec<u16>> = vec![];

        let size = terminal_size();

        if let Some((Width(w), Height(h))) = size {
            grid.resize_clear(w.into(), h.into());
            for _j in 0..h {
                let mut v = vec![];
                for _i in 0..w {
                    v.push(0x0000);
                }
                connections.push(v);
            }
        } else {
            panic!("Unable to get terminal size");
        }

        for (key, window) in self.windows.iter() {
            let loc = &window.g_loc();

            let size = &window.g_size();
            // corners
            connections[loc.1 as usize][loc.0 as usize] |= 0x0101;
            connections[loc.1 as usize][(loc.0 + size.0 + 1) as usize] |= 0x1100;
            connections[(loc.1 + size.1 + 1) as usize][loc.0 as usize] |= 0x0011;
            connections[(loc.1 + size.1 + 1) as usize][(loc.0 + size.0 + 1) as usize] |= 0x1010;

            // topline

            if size.0 < 7 {
                for i in 1..size.0 + 1 {
                    connections[loc.1 as usize][(loc.0 + i) as usize] |= 0x1001;
                }
            } else {
                let name = window.g_name();
                let mut name = name[0..(size.0 - 2) as usize].to_string();
                if let Some(n) = self.selected_window {
                    if n == *key {
                        name.pop();
                        name.insert(0, '~')
                    }
                }
                grid.write_line(
                    format!("[{}]", name),
                    self.CONFIG.get("theme_foreground").unwrap().get_color(),
                    self.CONFIG.get("theme_background").unwrap().get_color(),
                    (loc.0 + 1) as usize,
                    (loc.1) as usize,
                )
            }
            for i in 1..size.1 + 1 {
                connections[(loc.1 + i) as usize][loc.0 as usize] |= 0x0110;
            }
            for i in 1..size.1 + 1 {
                connections[(loc.1 + size.1 + 1) as usize][(loc.0 + i) as usize] |= 0x1001;
            }
            for i in 1..size.1 + 1 {
                connections[(loc.1 + i) as usize][(loc.0 + size.0 + 1) as usize] |= 0x0110;
            }

            grid.combine(&(loc.0 + 1, loc.1 + 1), window.display());
        }

        for (j, row) in connections.iter().enumerate() {
            for (i, c) in row.iter().enumerate() {
                let c = match c {
                    0x0000 => ' ',
                    0x0001 => '╶',
                    0x0010 => '╵',
                    0x0011 => '└',
                    0x0100 => '╷',
                    0x0101 => '┌',
                    0x0110 => '│',
                    0x0111 => '├',
                    0x1000 => '╴',
                    0x1001 => '─',
                    0x1010 => '┘',
                    0x1011 => '┴',
                    0x1100 => '┐',
                    0x1101 => '┬',
                    0x1110 => '┤',
                    0x1111 => '┼',
                    _ => panic!("Haven't mapped {} yet to a box character", c),
                };
                if c != ' ' {
                    grid.set_char(
                        TChar::new(
                            c,
                            self.CONFIG.get("theme_foreground").unwrap().get_color(),
                            self.CONFIG.get("theme_background").unwrap().get_color(),
                        ),
                        i,
                        j,
                    )
                }
            }
        }

        let mut str = grid.to_string();
        str.pop();
        str
    }

    pub fn set_selected_window(&mut self, uuid: Uuid) -> Option<()> {
        if self.windows.contains_key(&uuid) {
            self.selected_window = Some(uuid);
            Some(())
        } else {
            None
        }
    }

    pub fn get_selected_window(&mut self, uuid: Uuid) -> Option<&mut Box<(dyn Window)>> {
        self.windows.get_mut(&uuid)
    }

    pub fn process_key(&mut self, ev: Event) {
        match ev {
            Event::Key(KeyEvent {
                code: KeyCode::Char('z'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => process::exit(0),
            _ => {
                if let Some(uuid) = self.selected_window {
                    let window = self.get_selected_window(uuid).unwrap();
                    window.process_key(ev);
                }
            }
        }
    }
}
