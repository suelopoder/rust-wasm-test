use wasm_bindgen::prelude::*;
use std::collections::BinaryHeap;

#[wasm_bindgen]
pub fn naive_sort(input: &str) -> String {
    let chars = input.replace("[", "").replace("]", "");
    // println!("==== {}", chars);
    let mut chars: Vec<u32> = chars
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    
    chars.sort();

    let res = chars
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>().join(",");
    return format!("[{}]", res);
}

#[wasm_bindgen]
pub fn sort(input: &str) -> String {
    let mut heap = BinaryHeap::new();
    let chars = input.replace("[", "").replace("]", "");

    chars
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .for_each(|f| heap.push(f));

    let chars = heap.into_sorted_vec();

    let res = chars
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>().join(",");
    return format!("[{}]", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        assert_eq!(sort("[10,3,5]"), "[3,5,10]");
    }
}