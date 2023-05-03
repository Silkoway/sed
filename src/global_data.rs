use std::collections::HashMap;

use crate::wman::tChar::Color;

pub struct AnyT {
    val: String,
}

impl AnyT {
    pub fn new_color(color: Color) -> Self {
        Self {
            val: (color as i32).to_string(),
        }
    }
    pub fn get_string(&self) -> String {
        self.val.clone()
    }
    pub fn get_number(&self) -> i32 {
        self.val.clone().parse::<i32>().unwrap()
    }
    pub fn get_color(&self) -> Color {
        Color::try_from(self.get_number()).unwrap()
    }

    pub fn set_value(&mut self, v: String) {
        self.val = v.clone();
    }
}

pub type TConfig = HashMap<String, AnyT>;
