This contains some experiments on what browser-first WebAssembly data frames might look like. See [`www/index.js`](www/index.js) for sample usage. The current goal is to load a simple data set and visualize some aspect of it.

This project is built with [`wasm-pack`](https://rustwasm.github.io/docs/wasm-pack/).

# TODO

- File or watch issue for uninitialized types
- Can we separate allocation from computation?
- Read in a CSV file
- Return errors to JS
- Generate some plots
- Fix dumb JS vulnerabilities
- Add boolean vectors with `bitvec`
- Set up unit testing with wasm-pack
- Set up benchmarking with wasm-pack
