mod utils;

use ndarray::{array, Array2};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast as _;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
        utils::set_panic_hook(); // TODO is there a more principled place to call this?
        Self { arrays: Vec::new() }
    }

    pub fn new_array_from(&mut self, js_array: js_sys::Array) -> Handle {
        // TODO support varied shapes
        let mut new_array = array![[1., 2., 3.], [4., 5., 6.],];
        for (row_idx, row) in js_array.iter().enumerate() {
            if js_sys::Array::is_array(&row) {
                let row = js_sys::Array::from(&row);
                for (col_idx, value) in row.iter().enumerate() {
                    if let Some(value) = value.as_f64() {
                        new_array[[row_idx, col_idx]] = value as f32;
                    } else {
                        // TODO more informative errors
                        panic!("Value must be a float")
                    }
                }
            } else {
                panic!("Value must be an array")
            }
        }
        self.push_array(new_array)
    }

    pub fn new_array_float32(&mut self, n_rows: usize, n_cols: usize, js_array: js_sys::Float32Array) -> Handle {
        // TODO infer the size of the array
        self.push_array(Array2::from_shape_vec([n_rows, n_cols], js_array.to_vec()).unwrap())
    }

    pub fn map_js(&mut self, array: Handle, f: wasm_bindgen::JsValue) -> Handle {
        let f = f.dyn_ref::<js_sys::Function>().unwrap();
        self.push_array(self.arrays[array].map(|&value| {
            f.call1(&JsValue::NULL, &(value as f64).into())
                .unwrap()
                .as_f64()
                .unwrap() as f32  // TODO blindly converting f64 to f32 could lead to issues
        }))
    }

    pub fn map(&mut self, array: Handle) -> Handle {
        self.push_array(self.arrays[array].map(|&value| value + 1.0))
    }

    pub fn add_arrays(&mut self, array1: Handle, array2: Handle) -> Handle {
        self.push_array(&self.arrays[array1] + &self.arrays[array2])
    }

    pub fn log_array(&self, array: Handle) {
        web_sys::console::log_1(&format!("{}", &self.arrays[array]).into());
    }

    pub fn autograd_test(&self) {
        use autograd as ag;
        ag::with(|g: &mut ag::Graph<_>| {
            let a: ag::Tensor<f32> = g.ones(&[60]);
            let b: ag::Tensor<f32> = g.ones(&[24]);
            let c: ag::Tensor<f32> = g.reshape(a, &[3, 4, 5]);
            let d: ag::Tensor<f32> = g.reshape(b, &[4, 3, 2]);
            let e: ag::Tensor<f32> = g.tensordot(c, d, &[1, 0], &[0, 1]);
            web_sys::console::log_1(&format!("{:?}", &e.eval(&[])).into());
        });
    }
}
