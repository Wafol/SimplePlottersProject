use wasm_bindgen::prelude::*;

use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

#[wasm_bindgen]
pub fn draw() {
    let canvas_backend = CanvasBackend::new("canvas").expect("cannot find canvas");
    let root_drawing_area = canvas_backend.into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();
    
    let mut chart = ChartBuilder::on(&root_drawing_area)
        .build_cartesian_2d(0..100, 0..100)
        .unwrap();

    chart.draw_series(
        LineSeries::new((0..100).map(|x| (x, 100 - x)), &BLACK),
    ).unwrap();
}
