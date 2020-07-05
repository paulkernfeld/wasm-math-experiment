This contains some experiments on what browser-first WebAssembly data frames might look like. See [`www/index.js`](www/index.js) for sample usage.

This project is built with [`wasm-pack`](https://rustwasm.github.io/docs/wasm-pack/). 

- Current goal: visualize a data set
- Current subgoal: add data viz library

# TODO

- Research how JS vs. Rust GC works.
- Return errors to JS
- Add boolean vectors with `bitvec`
- Set up unit testing with wasm-pack
- Set up benchmarking with wasm-pack
- Add a benchmark with user-provided Webassembly
- Remove 2D arrays
- Remove Arena
- Remove tensor processing code
- Add links to other libraries
