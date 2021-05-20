#![recursion_limit = "512"]

use log::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Pope {
    internal: i32,
    pub a_float: f32,
}

#[wasm_bindgen]
impl Pope {
    pub fn new(val: i32) -> Pope {
        Pope {
            internal: val,
            a_float: val as f32,
        }
    }

    pub fn init(&self) {
        info!("INIT RUST COMPONENT")
    }

    pub fn update(&self) {
        info!("UPDATE RUST COMPONENT")
    }

    pub fn get(&self) -> i32 {
        self.internal
    }

    pub fn set(&mut self, val: i32) {
        self.internal = val;
    }

    pub fn pope_says_hi() {
        info!("pope_says_hi!");
    }
}

#[wasm_bindgen]
pub fn behavior_init() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
}

// fn main() {
//     wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
//     info!("Hello, world!");
// }
