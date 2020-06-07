mod utils;

use js_sys;
use ndarray::{array, Array2};
use std::convert::TryInto as _;
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
        utils::set_panic_hook();
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

    pub fn new_array_from(&mut self, js_array: js_sys::Array) -> Handle {
        let mut new_array = array![
            [1.,2.,3.],
            [4.,5.,6.],
        ];
        for (row_idx, row) in js_array.iter().enumerate() {
            if js_sys::Array::is_array(&row) {
                let row = js_sys::Array::from(&row);
                for (col_idx, value) in row.iter().enumerate() {
                    if let Some(value) = value.as_f64() {
                        new_array[[row_idx, col_idx]] = value as f32;
                    } else {
                        panic!("Value must be a float")
                    }
                }
            } else {
                panic!("Value must be an array")
            }
        }
        self.push_array(new_array)
    }

    pub fn new_array_float32(&mut self, js_array: js_sys::Float32Array) -> Handle {
        let mut new_array = array![
            [1.,2.,3.],
            [4.,5.,6.],
        ];
        assert_eq!(js_array.length(), new_array.len().try_into().unwrap());
        for row_idx in 0..new_array.nrows() {
            for col_idx in 0..new_array.ncols() {
                let idx_1d = (row_idx * new_array.ncols() + col_idx).try_into().unwrap();
                new_array[[row_idx, col_idx]] = js_array.get_index(idx_1d);
            }
        }
        self.push_array(new_array)
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
