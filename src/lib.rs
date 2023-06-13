use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn sort(input: &str) -> String {
    let mut input: Vec<u32> = input.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
    input.sort();
    return input.into_iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        assert_eq!(sort("10,3,5"), "3,5,10");
    }
}