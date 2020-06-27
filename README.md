This contains some experiments on what browser-first WebAssembly data frames might look like. See [`www/index.js`](www/index.js) for sample usage.

This project is built with [`wasm-pack`](https://rustwasm.github.io/docs/wasm-pack/). 

- Current goal: visualize a data set
- Current subgoal: load a CSV file

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
