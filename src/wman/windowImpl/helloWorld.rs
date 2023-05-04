use std::rc::Rc;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::global_data::TConfig;
use crate::wman::tChar::CharGrid;
use crate::wman::window::Window;

pub struct HelloWorldWindow {
    CONFIG: Rc<TConfig>,
    size: (i32, i32),
    pos: (i32, i32),

    y: i32,
    counter: i32,
    data: String,
}
impl HelloWorldWindow {
    pub fn new(config: Rc<TConfig>) -> Self {
        Self {
            CONFIG: (config),
            pos: (0, 0),
            size: (15, 15),
            y: 0,
            counter: 0,
            data: "".to_string(),
        }
    }
}

impl Window for HelloWorldWindow {
    fn display(&self) -> CharGrid {
        let mut grid = CharGrid::new(self.get_config());

        let size = self.g_size();
        grid.resize_clear(size.0, size.1);

        /*grid.set_char(
            TChar::new(
                'H',
                self.CONFIG.get("theme_foreground").unwrap().get_color(),
                self.CONFIG.get("theme_background").unwrap().get_color(),
            ),
            9,
            9,
        );*/
        grid.write_line(
            self.data.clone(),
            self.CONFIG.get("theme_foreground").unwrap().get_color(),
            self.CONFIG.get("theme_background").unwrap().get_color(),
            0,
            10,
        );
        grid.write_line(
            "Hello, world!".to_string(),
            self.CONFIG.get("theme_foreground").unwrap().get_color(),
            self.CONFIG.get("theme_background").unwrap().get_color(),
            0,
            self.y as usize,
        );
        grid
    }

    fn get_config(&self) -> Rc<TConfig> {
        self.CONFIG.clone()
    }

    fn g_size(&self) -> (i32, i32) {
        self.size.clone()
    }
    fn s_size(&mut self, s: (i32, i32)) {
        self.size = s;
    }

    fn g_loc(&self) -> (i32, i32) {
        self.pos.clone()
    }

    fn s_loc(&mut self, pos: (i32, i32)) {
        self.pos = pos;
    }

    fn g_name(&self) -> String {
        format!("{}Hello, world! Program", self.counter)
    }
    fn process_key(&mut self, key: crossterm::event::Event) {
        if let Event::Key(event) = key {
            self.data = format!("{:?}", event.code);
        }
        match key {
            Event::Key(KeyEvent {
                code: KeyCode::Char('j'),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                ..
            }) => {
                self.y += 1;
                self.counter += 1;
                if self.y >= self.g_size().1 {
                    self.y = 0;
                }
            }
            _ => {}
        }
    }

    fn g_cursor(&self) -> (i32, i32) {
        (0, 0)
    }
}
