#[macro_use]
extern crate crossterm;

use crossterm::event::read;

use crossterm::terminal::enable_raw_mode;
use std::io::stdout;

use std::{collections::HashMap, rc::Rc};

use global_data::{AnyT, TConfig};
pub mod global_data;
pub mod wman;
use wman::{tChar::Color, winManager::WindowManager, windowImpl::helloWorld::HelloWorldWindow};

fn main() {
    let mut CONFIG: TConfig = HashMap::new();

    {
        CONFIG.insert(
            "theme_foreground".to_string(),
            AnyT::new_color(Color::WhiteFG),
        );
        CONFIG.insert(
            "theme_background".to_string(),
            AnyT::new_color(Color::BlackBG),
        );
    };

    let rc = Rc::new(CONFIG);

    let mut manager = WindowManager::new(rc.clone());
    let h_win = HelloWorldWindow::new(rc.clone());
    let id = manager.push(Box::new(h_win));
    manager.set_selected_window(id);

    let _stdout = stdout();
    //going into raw mode
    enable_raw_mode().unwrap();

    loop {
        print!("\x1b[H{}", manager.render());

        let re = read().unwrap();
        manager.process_key(re);
    }
}
