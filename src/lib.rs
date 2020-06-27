mod utils;

use ndarray::{array, Array2};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast as _;
use std::collections::HashMap;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn array_from_js(n_rows: usize, n_cols: usize, js_array: &js_sys::Float32Array) -> Array2<f32> {
    Array2::from_shape_vec([n_rows, n_cols], js_array.to_vec()).unwrap()
}

#[wasm_bindgen]
pub struct Arena {
    arrays: Vec<Array2<f32>>,
    serieses_string: Vec<Vec<String>>, // TODO intern string
}

// TODO can we make these typesafe in Rust, or even into JS? What bugs would this prevent?
type Handle = usize;
type SeriesStringHandle = usize;

impl Arena {
    fn push_array(&mut self, array: Array2<f32>) -> Handle {
        self.arrays.push(array);
        self.arrays.len() - 1
    }

    fn push_series_string(&mut self, series_string: Vec<String>) -> SeriesStringHandle {
        self.serieses_string.push(series_string);
        self.serieses_string.len() - 1
    }
}

#[wasm_bindgen]
impl Arena {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        utils::set_panic_hook(); // TODO is there a more principled place to call this?
        Self { arrays: Vec::new(), serieses_string: Vec::new() }
    }

    // pub fn read_csv(&mut self, csv_text: &str) -> Frame {}

    pub fn new_series_string(&mut self, js_array: js_sys::Array) -> SeriesStringHandle {
        // TODO maybe JsString::try_from would be faster
        self.push_series_string(js_array.iter().map(|s| s.as_string().unwrap()).collect())
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

    pub fn new_array_from_json(&mut self, json: &str) -> Result<Handle, JsValue> {
        Ok(self.push_array(serde_json::from_str(json).map_err(|e| e.to_string())?))
    }

    pub fn new_array_float32(
        &mut self,
        n_rows: usize,
        n_cols: usize,
        js_array: js_sys::Float32Array,
    ) -> Handle {
        // TODO infer the size of the array
        self.push_array(array_from_js(n_rows, n_cols, &js_array))
    }

    pub fn map_js(&mut self, array: Handle, f: wasm_bindgen::JsValue) -> Handle {
        let f = f.dyn_ref::<js_sys::Function>().unwrap();
        self.push_array(self.arrays[array].map(|&value| {
            f.call1(&JsValue::NULL, &(value as f64).into())
                .unwrap()
                .as_f64()
                .unwrap() as f32 // TODO blindly converting f64 to f32 could lead to issues
        }))
    }

    pub fn map_batch(&mut self, array: Handle, f: wasm_bindgen::JsValue) -> Handle {
        let f = f.dyn_ref::<js_sys::Function>().unwrap();
        let array_js = self.get_array_float32(array);
        let array = &self.arrays[array];
        let map_result = f.call1(&JsValue::NULL, &array_js).unwrap();
        let array_mapped_js = map_result.dyn_into::<js_sys::Float32Array>().unwrap();
        let n_rows = array.nrows();
        let n_cols = array.ncols();
        self.push_array(array_from_js(n_rows, n_cols, &array_mapped_js))
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

    pub fn get_array_float32(&self, array: Handle) -> js_sys::Float32Array {
        // TODO use https://docs.rs/js-sys/0.3.40/js_sys/struct.Float32Array.html#method.view
        // ...but how do we make it safe?
        // TODO does &mut self let us make this safe?
        let array = &self.arrays[array];
        array.as_slice().unwrap().into()
    }

    pub fn tract_add_3(&mut self, array: Handle) -> Handle {
        let array = &self.arrays[array];
        use tract_core::internal::*;

        // TODO build computation graph in advance for performance
        // build a simple model that just add 3 to each input component
        let mut model = TypedModel::default();

        let shape = array.shape();
        let input_fact = TypedFact::dt_shape(f32::datum_type(), shape).unwrap();
        let input = model.add_source("input", input_fact).unwrap();
        let three = model.add_const("three".to_string(), tensor0(3f32)).unwrap();
        let _add = model
            .wire_node(
                "add".to_string(),
                tract_core::ops::math::add::bin_typed(),
                [input, three].as_ref(),
            )
            .unwrap();

        model.auto_outputs().unwrap();

        // We build an execution plan. Default inputs and outputs are inferred from
        // the model graph.
        let plan = SimplePlan::new(&model).unwrap();

        // run the computation.
        // TODO we want a conversion method that never fails
        let input = Tensor::from(array.to_owned());
        let mut outputs = plan.run(tvec![input]).unwrap();

        // take the first and only output tensor
        let tensor = outputs.pop().unwrap();

        // TODO don't try_unwrap
        self.push_array(
            Arc::try_unwrap(tensor)
                .unwrap()
                .into_array()
                .unwrap()
                .into_dimensionality()
                .unwrap(),
        )
    }
}

pub struct Frame {
    serieses: HashMap<String, SeriesStringHandle>,
}

impl Frame {
    // TODO maybe we want to just do this from a JS object, maps are less pretty to create
    // pub fn new(js_object: js_sys::Map) -> Self {
    //     Self {
    //         serieses: js_object.entries().into_iter().map(|entry| {
    //             let array = entry.unwrap().dyn_into::<js_sys::Array>().unwrap();
    //             let key = array.get(0);
    //             let value = array.get(1);
    //             ()
    //         }).collect()
    //     }
    // }
    // pub fn new(js_object: js_sys::Object) -> Self {
    //     Self {
    //         serieses: Object::entries(js_object).into_iter().map(|entry| {
    //             let array = entry.unwrap().dyn_into::<js_sys::Array>().unwrap();
    //             let key = array.get(0);
    //             let value = array.get(1);
    //             ()
    //         }).collect()
    //     }
    // }
}