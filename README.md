This is a proof-of-concept on doing some simple math in WebAssembly that can be used from the browser. See [`www/index.js`](www/index.js) for sample usage.

This project is built with [`wasm-pack`](https://rustwasm.github.io/docs/wasm-pack/).

# TODO

- File or watch issue for uninitialized types
- Can we separate allocation from computation?
- Read in a CSV file
- Read in a JSON file
- Return errors to JS
- Generate some plots
- Add boolean vectors with `bitvec`
- Set up unit testing with wasm-pack
- Set up benchmarking with wasm-pack
