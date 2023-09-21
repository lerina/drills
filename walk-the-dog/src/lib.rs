use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use rand::prelude::*;


#[wasm_bindgen(start, catch)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    console::log_1(&JsValue::from_str("You can see this in the browsers console log"));

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
/*
        let image = web_sys::HtmlImageElement::new().unwrap();
        image.set_src("../resources/pix/Idle (1).png");
        
It turns out you can't draw the image immediately 
after setting the source of an image element 
because the image hasn't been loaded yet. 
In order to wait for the image to be loaded, 
we'll use the onload callback of HtmlImageElement , 
which you can set up using set_onload in Rust.

        context.draw_image_with_html_image_element(&image, 0.0, 0.0);
*/

    wasm_bindgen_futures::spawn_local(async move {
        let (success_tx, success_rx) = futures::channel::oneshot::channel::<()>();
        let image = web_sys::HtmlImageElement::new().unwrap();
        let callback = Closure::once(move || {
            success_tx.send(());
            web_sys::console::log_1(&JsValue::from_str("loaded"));
        });        
        
        image.set_onload(Some(callback.as_ref().unchecked_ref()));
        callback.forget();

        image.set_src("../resources/pix/Idle (1).png");
        context.draw_image_with_html_image_element(&image, 0.0, 0.0);

        sierpinski(&context, 
                   [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)], 
                   (0, 255, 0), 
                   5);
        }); //^-- wasm_bindgen_futures::spawn_local()

    Ok(())
}

fn sierpinski(context: &web_sys::CanvasRenderingContext2d, 
              points: [(f64, f64); 3],
              color: (u8, u8, u8), 
              depth: u8) {

        draw_triangle(&context, points, color);
        
        let depth = depth - 1;
        let [top, left, right] = points;
        
        if depth > 0 {
            let mut rng = thread_rng();

            let next_color = (
                rng.gen_range(0..255),
                rng.gen_range(0..255),
                rng.gen_range(0..255),
            );

            let left_middle = midpoint(top, left);
            let right_middle = midpoint(top, right);
            let bottom_middle = midpoint(left, right);
   
            sierpinski(&context, 
                       [top, left_middle, right_middle], next_color, depth);
            sierpinski(&context, 
                       [left_middle, left, bottom_middle], next_color, depth);
            sierpinski(&context, 
                       [right_middle, bottom_middle, right], next_color, depth);

        }//^-- if depth
}

fn midpoint(point_1: (f64, f64), point_2: (f64, f64)) -> (f64, f64) {
    ((point_1.0 + point_2.0) / 2.0, 
     (point_1.1 + point_2.1)/ 2.0)
}

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d,
                 points: [(f64, f64); 3], 
                 color: (u8, u8, u8),) {
    let [top, left, right] = points;

    // Convert Rust String to JsValue.
    let color_str = format!("rgb({}, {}, {})", color.0, color.1, color.2);
    //lets spy on it
    console::log_1(&JsValue::from_str(&color_str));
    // and use it 
    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&color_str));
    
    context.move_to(top.0, top.1);
    context.begin_path();
    context.line_to(left.0, left.1);
    context.line_to(right.0, right.1);
    context.line_to(top.0, top.1);
    context.close_path();
    context.stroke();
    context.fill();
}
