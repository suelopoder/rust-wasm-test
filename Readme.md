# What

This app is a simple test for Rust on the web via WASM

## Screenshots

This are local screenshots from running this code

![UI](/assets/wasm-rust-ui.png)
![Chrome timing console output](/assets/wasm-rust-console.png)
![Chrome performance recording](/assets/wasm-rust-perf.png)

# How

Run `wasm-pack build --target web` to build the JS glue code.
Then open index.html in a browser.

## Unit test

To test rust code run: `wasm-pack test --node`

# Why

Test rust and JS interactions and wrap my head around when it would be a good idea to use it.

# Resources

https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
https://doc.rust-lang.org/book/title-page.html
