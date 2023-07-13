self.importScripts('./pkg/rust_wasm_test.js'); // declares wasm_bindgen

const LEN = 10000000
const ARRAY = Array.from({ length: LEN }, () => Math.floor(Math.random() * LEN))

function jsSort() {
  const arr = structuredClone(ARRAY)
  console.time("JS sort");
  arr.sort((a, b) => a - b);
  console.timeEnd("JS sort");
}

console.time("worker loaded rust");
wasm_bindgen('./pkg/rust_wasm_test_bg.wasm').then(() => {
  console.timeEnd("worker loaded rust");

  function rustSort() {
    const arr = structuredClone(ARRAY)
    console.time("Rust sort");
    wasm_bindgen.sort(arr);
    console.timeEnd("Rust sort");
  }
  
  onmessage = function(e) {
    console.log('Worker: Message received from main script', e.data);
    if (e.data === 'rust') {
      rustSort()
    } else {
      jsSort()
    }
    postMessage('done');
  }

  console.log('Worker ready');
})
