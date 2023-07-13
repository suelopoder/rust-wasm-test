# What

This app is a simple test for Rust on the web via WASM.
I test a simple sort of a big array in plain JS and in a compiled rust lib. Then I tried both again but loaded from a web worker.

![Chrome timing console output](/assets/wasm-rust-console.png)
![Chrome performance recording](/assets/wasm-rust-perf.png)

### Performance results
As expected Rust beats JS sorting a JS array for big arrays. It's worth noting that both block the main thread. The workaround is to sort within a web worker, with similar timings but not blocking.
![Results](/assets/main-thread-vs-worker.png)

![Demo](/assets/demo.gif)

# How

Run `wasm-pack build --target no-modules` to build the JS glue code.
Then open index.html in a browser.

## Unit test

To test rust code run: `wasm-pack test --node`

# Why

Test rust and JS interactions and wrap my head around when it would be a good idea to use it.

# Resources

* https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
https://doc.rust-lang.org/book/title-page.html
* https://rustwasm.github.io/wasm-bindgen/examples/wasm-in-web-worker.html
* https://doc.rust-lang.org/stable/book/

# Next steps

I wonder if we could return a promise in Rust that will execute in background. This shouldn't improve performance though.