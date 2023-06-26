use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sort(input: &js_sys::Uint8Array) -> Vec<u8> {
    let mut vec = input.to_vec();
    vec.sort();
    return vec;
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    #[wasm_bindgen_test]
    fn test_sort() {
        let arr = js_sys::Array::new();
        arr.push(&JsValue::from(5));
        arr.push(&JsValue::from(10));
        arr.push(&JsValue::from(3));
        let array = js_sys::Uint8Array::new(&arr);
        assert_eq!(sort(&array), [3,5,10]);
    }
}