import * as wasm from "wasm-math-experiment";

let arena = new wasm.Arena();

// Easiest example
let array1 = arena.new_array_from([[1, 1, 1], [2, 2, 2]]);
let array2 = arena.map_js(array1, x => x + 1);
arena.log_array(array2);
let array3 = arena.new_array_float32(3, 2, new Float32Array([1, 2, 3, 1, 2, 3])); // In row-major order

let array_sum = arena.add_arrays(array1, array2);

arena.log_array(array_sum);

let long = 1000000;

// Benchmark
{
    let long_1 = arena.new_array_float32(long, 1, new Float32Array(long));
    const t0 = performance.now();
    let long_2 = arena.map(long_1);
    const t1 = performance.now();
    console.log(`Mapping with pure Wasm took ${t1 - t0} milliseconds.`);
}

{
    let long_1_js = new Float32Array(long)
    const t0 = performance.now();
    let long_2_js = long_1_js.map(x => x + 1);
    const t1 = performance.now();
    console.log(`Mapping with pure JS took ${t1 - t0} milliseconds.`);
}

{
    let simple = arena.new_array_float32(long, 1, new Float32Array(long));
    const t0 = performance.now();
    let tract_sum = arena.tract_add_3(simple);
    const t1 = performance.now();
    console.log(`Mapping with tract took ${t1 - t0} milliseconds.`);
}

{
    let long_1_wasm = arena.new_array_float32(long, 1, new Float32Array(long));
    const t0 = performance.now();
    let long_2_wasm = arena.map_js(long_1_wasm, x => x + 1);
    const t1 = performance.now();
    console.log(`Mapping with Wasm calling JS took ${t1 - t0} milliseconds.`);
}
