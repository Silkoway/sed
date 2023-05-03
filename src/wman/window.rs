use std::rc::Rc;

use crossterm::event::Event;

use crate::global_data::TConfig;

use super::tChar::CharGrid;

pub trait Window {
    fn get_config(&self) -> Rc<TConfig>;
    fn display(&self) -> CharGrid;

    fn g_size(&self) -> (i32, i32);
    fn s_size(&mut self, size: (i32, i32));

    fn g_loc(&self) -> (i32, i32);
    fn s_loc(&mut self, loc: (i32, i32));

    fn g_name(&self) -> String;

    fn process_key(&mut self, key: Event);
}
