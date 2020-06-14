import * as wasm from "wasm-math-experiment";

let arena = new wasm.Arena();

// Easiest example
let array1 = arena.new_array_from([[1, 1, 1], [2, 2, 2]]);
let array2 = arena.map_js(array1, x => x + 1);
arena.log_array(array2);
let array3 = arena.new_array_float32(3, 2, new Float32Array([1, 2, 3, 1, 2, 3])); // In row-major order

let array_sum = arena.add_arrays(array1, array2);

arena.log_array(array_sum);

// Benchmark
let long_1 = arena.new_array_float32(10000000, 1, new Float32Array(10000000));

const t0 = performance.now();
let long_2 = arena.map_js(long_1, x => x + 1);
const t1 = performance.now();
console.log(`Mapping took ${t1 - t0} milliseconds.`);
