use js_canvas_rendering_context_2d::*;
use std::f32::consts::*;

fn main(){
    for i in 0..6{
        for j in 0..6 {
            CanvasRenderingContext2D::set_stroke_style_rgba(0, (255. - 42.5 * i as f32).floor() as u8, (255.0 - 42.5 * j as f32).floor() as u8, 255);
            CanvasRenderingContext2D::begin_path();
            CanvasRenderingContext2D::arc(12.5 + (j * 25) as f32, 12.5 + (i * 25) as f32, 10., 0., PI * 2.0, true);
            CanvasRenderingContext2D::stroke();
        }
    }
}