use seed::{prelude::*, *};

fn view() {
    log! {"view function"}
}
fn update() {
    log! {"update function"}
}

#[wasm_bindgen(start)]
pub fn start() {
    log! {"start function"}
}
