import * as wasm from "wasm-math-experiment";
import txt from "./FoodData_Central_csv_2019-04-02/food.csv";
import TinyTest from "./vendor/tinytest.js";

let arena = new wasm.Arena();

TinyTest.run({
  'read JSON': function() {
      // Read JSON. TODO: would users like this ndarray format?
      let array_json = arena.new_array_from_json('{"v":1,"dim":[2,3],"data":[3,1,2.2,3.1,4,7]}');
      arena.log_array(array_json);
      // TODO make an assertion
  },

  "create string series": function() {
      let strings = arena.new_series_string(["a", "b", "c"]);
      // TODO make an assertion
  },

  "create string frame": function() {
      let frame = arena.new_frame({"letters": ["a", "b", "c"]});
      TinyTest.assertEquals(frame.s("letters"), frame.s("letters"));
      // TODO assert on value equality (above is reference equality)
  },

  "fetch CSV file": async function() {
    // TODO this test depends on external data, which could make it flaky
    let frameAndArena = await wasm.fetch_csv(arena, fetch("https://data.cityofnewyork.us/api/views/zt9s-n5aj/rows.csv?accessType=DOWNLOAD"));
    // TODO hrrrnnnnnnnghhhh
    frameAndArena.take_frame().log(frameAndArena.take_arena());
    // TODO make some assertions
  },
});


// TODO add CSV reading
// let foods = arena.read_csv(txt);

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
    let long_1 = arena.new_array_float32(long, 1, new Float32Array(long));
    const t0 = performance.now();
    let long_2 = arena.map_batch(long_1, array => array.map(x => x + 1));
    const t1 = performance.now();
    console.log(`Mapping with batched JS took ${t1 - t0} milliseconds.`);
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
