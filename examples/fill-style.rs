use js_canvas_rendering_context_2d::*;

fn main(){
    for i in 0..6{
        for j in 0..6 {
            CanvasRenderingContext2D::set_fill_style_rgba((255. - 42.5 * i as f32).floor() as u8, (255.0 - 42.5 * j as f32).floor() as u8, 0, 255);
            CanvasRenderingContext2D::fill_rect(j as f32 * 25., i as f32 * 25., 25., 25.);
        }
    }
}