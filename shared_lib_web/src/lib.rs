use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn add(left: u64, right: u64) -> u64 {
    let res = sharedlib::add(left, right);
    return match res {
        Ok(r) => {
            r
        }
        Err(_) => {
            0
        }
    }
}

#[wasm_bindgen]
pub fn create_ed25519() -> String {
    return sharedlib::create_ed25519();
}