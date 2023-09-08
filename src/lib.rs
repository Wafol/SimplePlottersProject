use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

mod func_plot;

/// Type alias for the result of a drawing function.
#[wasm_bindgen]
pub fn myDraw() {
    func_plot::draw("canvas", 1);
}
