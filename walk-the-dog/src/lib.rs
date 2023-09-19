use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;


// Note that if you import a JS function with Result you need 
// #[wasm_bindgen(catch)] to be annotated on the import 
// (unlike exported functions, which require no extra annotation). 
// This may not be necessary in the future though and it may work "as is"!.
// [Result Type](https://rustwasm.github.io/docs/wasm-bindgen/reference/types/result.html)
// [similar issue that gave the solution](https://github.com/rustwasm/wasm-bindgen/issues/2919)

#[wasm_bindgen(start, catch)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

//    console::log_1(&JsValue::from_str("Hello world!"));
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
                    .get_element_by_id("canvas")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .unwrap();

    let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .unwrap();

        context.move_to(300.0, 0.0); // top of triangle
        context.begin_path();
        context.line_to(0.0, 600.0); // bottom left of triangle
        context.line_to(600.0, 600.0); // bottom right of triangle
        context.line_to(300.0, 0.0); // back to top of triangle
        context.close_path();
        context.stroke();
        context.fill();

    Ok(())
}
