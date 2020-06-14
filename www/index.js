import * as wasm from "wasm-math-experiment";

let arena = new wasm.Arena();

let array1 = arena.new_array_from([[1, 1, 1], [2, 2, 2]]);
array1 = arena.map_js(array1, x => x + 1);
arena.log_array(array1);
let array2 = arena.new_array_float32(new Float32Array([1, 2, 3, 1, 2, 3])); // In row-major order

let array_sum = arena.add_arrays(array1, array2);

arena.log_array(array_sum);
