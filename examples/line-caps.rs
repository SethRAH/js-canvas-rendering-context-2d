use js_canvas_rendering_context_2d::*;

fn main() {
    // draw guides
    CanvasRenderingContext2D::begin_path();
    CanvasRenderingContext2D::set_stroke_style_rgba(0, 153, 255, 255);
    CanvasRenderingContext2D::move_to(10., 10.);
    CanvasRenderingContext2D::line_to(140., 10.);
    CanvasRenderingContext2D::move_to(10., 140.);
    CanvasRenderingContext2D::line_to(140., 140.);
    CanvasRenderingContext2D::stroke();
    
    // Draw Lines
    CanvasRenderingContext2D::set_stroke_style_rgba(0, 0, 0, 255);
    let mut i = 0;
    for cap in LineCap::iterator() {
        CanvasRenderingContext2D::set_line_width(15);
        CanvasRenderingContext2D::set_line_cap(cap);
        CanvasRenderingContext2D::begin_path();
        CanvasRenderingContext2D::move_to(25.0 + i as f32 * 50., 10.);
        CanvasRenderingContext2D::line_to(25.0 + i as f32 * 50., 140.);
        CanvasRenderingContext2D::stroke();
        i += 1;
    }

}
