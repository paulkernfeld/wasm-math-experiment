mod utils;

use ndarray::{array, Array2};
use wasm_bindgen::prelude::*;
use web_sys;

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

impl Arena {
    fn push_array(&mut self, array: Array2<f32>) -> Handle {
        self.arrays.push(array);
        self.arrays.len() - 1
    }
}

#[wasm_bindgen]
impl Arena {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { arrays: Vec::new() }
    }

    pub fn new_array(&mut self) -> Handle {
        self.push_array(
            array![
                [1.,2.,3.],
                [4.,5.,6.],
            ]
        )
    }

    pub fn add_arrays(&mut self, array1: Handle, array2: Handle) -> Handle {
        self.push_array(&self.arrays[array1] + &self.arrays[array2])
    }

    pub fn log_array(&self, array: Handle) {
        use web_sys::console;

        console::log_1(&format!("{}", &self.arrays[array]).into());
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
