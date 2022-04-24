use js_canvas_rendering_context_2d::*;

fn main(){
// Big M
  draw();
}

#[no_mangle]
pub extern "C" fn draw(){
   CanvasRenderingContext2D::set_shadow_color_rgba(255, 0, 0, 204);
   CanvasRenderingContext2D::set_shadow_blur(8);
   CanvasRenderingContext2D::set_shadow_offset_x(30);
   CanvasRenderingContext2D::set_shadow_offset_y(20);

   CanvasRenderingContext2D::set_fill_style_rgba(0, 255, 0, 51);
   CanvasRenderingContext2D::fill_rect(10., 10., 150., 100.);

   CanvasRenderingContext2D::set_line_width(10);
   CanvasRenderingContext2D::set_stroke_style_rgba(0, 0, 255, 153);
   CanvasRenderingContext2D::stroke_rect(10.,10.,150.,100.);

}