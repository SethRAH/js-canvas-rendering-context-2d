use js_canvas_rendering_context_2d::*;

fn main() {
    draw();
}


#[no_mangle]
pub extern "C" fn draw(){
    
    // Draw Lines
    CanvasRenderingContext2D::set_stroke_style_rgba(0, 0, 0, 255);
    CanvasRenderingContext2D::set_line_width(10);
    let mut i = 0;
    for join in LineJoin::iterator() {
        CanvasRenderingContext2D::set_line_join(join);
        CanvasRenderingContext2D::begin_path();
        CanvasRenderingContext2D::move_to(-5.0, 5.0 + i as f32 * 40.);
        CanvasRenderingContext2D::line_to(35.0, 45.0 + i as f32 * 40.);
        CanvasRenderingContext2D::line_to(75.0, 5.0 + i as f32 * 40.);
        CanvasRenderingContext2D::line_to(115.0, 45.0 + i as f32 * 40.);
        CanvasRenderingContext2D::line_to(155.0, 5.0 + i as f32 * 40.);
        CanvasRenderingContext2D::stroke();
        i += 1;
    }
}
