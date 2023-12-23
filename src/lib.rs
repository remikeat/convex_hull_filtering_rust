pub mod convex_hull;
pub mod edge;
pub mod point;

use serde::Deserialize;

use convex_hull::ConvexHull;
use web_sys::CanvasRenderingContext2d;

use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

#[derive(Debug, Deserialize)]
pub struct ConvexHulls {
    #[serde(alias = "convex hulls")]
    pub convex_hulls: Vec<ConvexHull>,
}

pub struct DrawingInfo<'a> {
    context: &'a CanvasRenderingContext2d,
    offset_x: f64,
    offset_y: f64,
    scale: f64,
}

#[wasm_bindgen]
pub fn draw_convex_hulls(offset_x: f64, offset_y: f64, scale: f64, convex_hull: JsValue) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let convex_hulls: ConvexHulls = serde_wasm_bindgen::from_value(convex_hull).unwrap();
    let drawing_info = DrawingInfo {
        context: &context,
        offset_x,
        offset_y,
        scale,
    };
    for convex_hull in convex_hulls.convex_hulls.iter() {
        draw_convex_hull(&drawing_info, convex_hull, String::from("blue"));
    }
    let (_, inter) =
        ConvexHull::intersection(&convex_hulls.convex_hulls[1], &convex_hulls.convex_hulls[2]);
    draw_convex_hull(&drawing_info, &inter, String::from("red"))
}

pub fn draw_convex_hull(drawing_info: &DrawingInfo, convex_hull: &ConvexHull, color: String) {
    drawing_info
        .context
        .set_stroke_style(&JsValue::from_str(&color));
    drawing_info.context.begin_path();
    for pt in convex_hull.apexes.iter() {
        drawing_info.context.line_to(
            drawing_info.offset_x + drawing_info.scale * pt.x,
            drawing_info.offset_y + drawing_info.scale * pt.y,
        )
    }
    drawing_info.context.close_path();
    drawing_info.context.stroke();
}
