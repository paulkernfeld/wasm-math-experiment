mod utils;

use csv::Reader;
use js_sys::{Array, ArrayBuffer, Promise, Uint8Array};
use ndarray::{array, Array1, Array2};
use std::collections::HashMap;
use std::convert::TryInto as _;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast as _;
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub async fn fetch_csv(fetch: Promise) -> Result<Frame, JsValue> {
    let array_buffer = JsFuture::from(
        JsFuture::from(fetch)
            .await?
            .dyn_into::<Response>()?
            .array_buffer()?,
    )
    .await?
    .dyn_into::<ArrayBuffer>()?;
    let vec = Uint8Array::new(&array_buffer).to_vec();
    let mut reader = Reader::from_reader(Cursor::new(vec));
    let mut rows = reader.records();
    let mut new_serieses: Vec<Vec<String>> = Vec::new();
    for field in rows.next().unwrap().unwrap().iter() {
        new_serieses.push(vec![field.to_string()]);
    }

    for result in rows {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here..
        let record = result.unwrap(); // TODO handle bad csv data
        for (i, field) in record.iter().enumerate() {
            new_serieses[i].push(field.to_string());
        }
    }
    Ok(Frame {
        serieses: new_serieses
            .into_iter()
            .enumerate()
            .map(|(i, series)| (format!("series_{}", i), Series::from(SeriesString { inner: series })))
            .collect(),
    })
}

fn array_from_js(n_rows: usize, n_cols: usize, js_array: &js_sys::Float32Array) -> Array2<f32> {
    Array2::from_shape_vec([n_rows, n_cols], js_array.to_vec()).unwrap()
}

// TODO users shouldn't need to deal with this (unless they want to)
#[wasm_bindgen]
pub struct Arena {
    arrays: Vec<Array2<f32>>,
}

// TODO can we make these typesafe in Rust, or even into JS? What bugs would this prevent? By
// handing JS objects back, we could let users use method calls rather than always `arena.`.
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

// wasm-bindgen doesn't currently allow ADTs so this wraps around it
#[wasm_bindgen]
pub struct Series(SeriesInner);

impl From<SeriesString> for Series {
    fn from(series_string: SeriesString) -> Self {
        Self(SeriesInner::String(series_string))
    }
}

#[wasm_bindgen]
impl Series {
    pub fn log(&self) {
        match &self.0 {
            SeriesInner::F32(inner) => inner.log(),
            SeriesInner::String(inner) => inner.log(),
        }
    }
}

pub enum SeriesInner {
    F32(SeriesF32),
    String(SeriesString),
}

#[wasm_bindgen]
pub struct SeriesF32 {
    inner: Array1<f32>,
}

#[wasm_bindgen]
impl SeriesF32 {
    pub fn from_js_array(js_array: Array) -> Self {
        Self {
            inner: js_array.iter().map(|s| s.as_f64().unwrap() as f32).collect(),
        }
    }

    pub fn log(&self) {
        web_sys::console::log_1(&format!("{:?}", &self.inner).into());
    }
}

#[wasm_bindgen]
pub struct SeriesString {
    inner: Vec<String>,
}

#[wasm_bindgen]
impl SeriesString {
    pub fn from_js_array(js_array: Array) -> Self {
        Self {
            inner: js_array.iter().map(|s| s.as_string().unwrap()).collect(),
        }
    }

    pub fn log(&self) {
        web_sys::console::log_1(&format!("{:?}", &self.inner).into());
    }
}

#[wasm_bindgen]
pub struct Frame {
    serieses: HashMap<String, Series>,
}

#[wasm_bindgen]
impl Frame {
    pub fn new(&mut self, js_object: js_sys::Object) -> Self {
        Self {
            serieses: js_sys::Object::entries(&js_object)
                .iter()
                .map(|entry| {
                    let entry = entry.dyn_into::<js_sys::Array>().unwrap();
                    (
                        entry.get(0).as_string().unwrap(),
                        SeriesString::from_js_array(entry.get(1).try_into().unwrap()).into(),
                    )
                })
                .collect(),
        }
    }

    pub fn log(&self) {
        // TODO ideally these should be logged in order
        for (series_name, ref series) in self.serieses.iter() {
            web_sys::console::log_1(&series_name.into());
            series.log();
        }
    }
}
