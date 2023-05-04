use std::rc::Rc;

use crossterm::event::Event;

use crate::global_data::TConfig;

use super::tChar::CharGrid;

pub trait Window {
    /// Get config
    fn get_config(&self) -> Rc<TConfig>;
    /// Return a CharGrid that is the
    /// array of pixels that is being displayed onto the screen
    ///
    /// CharGrid is relative to the window and *not* the screen
    fn display(&self) -> CharGrid;

    /// Get the window size in (width, height) format
    fn g_size(&self) -> (i32, i32);
    /// Set the window size in (width, height) format
    fn s_size(&mut self, size: (i32, i32));
    /// Get the location in (rel x, rel y) format
    fn g_loc(&self) -> (i32, i32);

    /// Set the location in (rel x, rel y) format
    fn s_loc(&mut self, loc: (i32, i32));
    /// Get the name of the window
    fn g_name(&self) -> String;

    fn process_key(&mut self, key: Event);

    /// Get cursor position in (rel x, rel y) format
    fn g_cursor(&self) -> (i32, i32);
}
