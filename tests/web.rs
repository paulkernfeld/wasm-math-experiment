//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    // let arena = crate::Arena::default();

    // assert_eq!(a.ndim(), 2);         // get the number of dimensions of array a
    // assert_eq!(a.len(), 6);          // get the number of elements in array a
    // assert_eq!(a.shape(), [2, 3]);   // get the shape of array a
    // assert_eq!(a.is_empty(), false); // check if the array has zero elements
    // assert_eq!(1 + 1, 2);
}
