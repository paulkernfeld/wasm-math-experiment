mod utils;

use ndarray::{array, Array2};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Arena {
    arrays: Vec<Array2<f32>>,
}

type Handle = usize;

#[wasm_bindgen]
impl Arena {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { arrays: Vec::new() }
    }

    pub fn new_array(&mut self) -> Handle {
        self.arrays.push(
            array![
                [1.,2.,3.],
                [4.,5.,6.],
            ]
        );
        self.arrays.len() - 1
    }

    pub fn add_arrays(&mut self, array1: Handle, array2: Handle) -> Handle {
        self.arrays.push(&self.arrays[array1] + &self.arrays[array2]);
        self.arrays.len() - 1
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
