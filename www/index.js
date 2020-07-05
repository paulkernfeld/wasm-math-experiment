import * as wasm from "wasm-math-experiment";
import {Frame, SeriesString} from "wasm-math-experiment";
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
      let strings = new SeriesString(["a", "b", "c"]);
      // TODO make an assertion
  },

  "create string frame": function() {
      let frame = new Frame({"letters": ["a", "b", "c"]});
      // TODO make an assertion
  },

  "fetch CSV file": async function() {
    // TODO this test depends on external data, which could make it flaky
    let frame = await wasm.fetch_csv(fetch("https://data.cityofnewyork.us/api/views/zt9s-n5aj/rows.csv?accessType=DOWNLOAD"));
    // TODO add frame.to_vega fn. Maybe like:
    // let values = frame.to_vega({ "category": "series_1", "amount": "series_2" });

    let values = [];
    for (let r = 0; r < 2; r++) {
    // TODO show more schools
//    for (let r = 0; r < frame.len(); r++) {
        // TODO use human-friendly names
        let category = frame.s("series_1").v(r);
        let amount = frame.s("series_2").v(r);
        values.push({ "category": category, "amount": amount });
    }

    let vegaSpec = {
      "$schema": "https://vega.github.io/schema/vega/v5.json",
      "description": "A basic bar chart example, with value labels shown upon mouse hover.",
      "width": 400,
      "height": 200,
      "padding": 5,

      "data": [
        {
          "name": "table",
          "values": values
        }
      ],

      "signals": [
        {
          "name": "tooltip",
          "value": {},
          "on": [
            {"events": "rect:mouseover", "update": "datum"},
            {"events": "rect:mouseout",  "update": "{}"}
          ]
        }
      ],

      "scales": [
        {
          "name": "xscale",
          "type": "band",
          "domain": {"data": "table", "field": "category"},
          "range": "width",
          "padding": 0.05,
          "round": true
        },
        {
          "name": "yscale",
          "domain": {"data": "table", "field": "amount"},
          "nice": true,
          "range": "height"
        }
      ],

      "axes": [
        { "orient": "bottom", "scale": "xscale" },
        { "orient": "left", "scale": "yscale" }
      ],

      "marks": [
        {
          "type": "rect",
          "from": {"data":"table"},
          "encode": {
            "enter": {
              "x": {"scale": "xscale", "field": "category"},
              "width": {"scale": "xscale", "band": 1},
              "y": {"scale": "yscale", "field": "amount"},
              "y2": {"scale": "yscale", "value": 0}
            },
            "update": {
              "fill": {"value": "steelblue"}
            },
            "hover": {
              "fill": {"value": "red"}
            }
          }
        },
        {
          "type": "text",
          "encode": {
            "enter": {
              "align": {"value": "center"},
              "baseline": {"value": "bottom"},
              "fill": {"value": "#333"}
            },
            "update": {
              "x": {"scale": "xscale", "signal": "tooltip.category", "band": 0.5},
              "y": {"scale": "yscale", "signal": "tooltip.amount", "offset": -2},
              "text": {"signal": "tooltip.amount"},
              "fillOpacity": [
                {"test": "datum === tooltip", "value": 0},
                {"value": 1}
              ]
            }
          }
        }
      ]
    };

    var view;

    function render(spec) {
      view = new vega.View(vega.parse(spec), {
        renderer:  'canvas',  // renderer (canvas or svg)
        container: '#view',   // parent DOM container
        hover:     true       // enable hover processing
      });
      return view.runAsync();
    }

    render(vegaSpec);

    // TODO make some assertions
  },
});

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
