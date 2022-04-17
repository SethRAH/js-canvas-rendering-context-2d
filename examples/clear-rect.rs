use js_canvas_rendering_context_2d::*;

fn main(){
    draw();
}


#[no_mangle]
pub extern "C" fn draw(){

    let height = CanvasRenderingContext2D::get_canvas_height();
    let width = CanvasRenderingContext2D::get_canvas_width();

   // draw yellow background
   CanvasRenderingContext2D::begin_path();
   CanvasRenderingContext2D::set_fill_style_rgba(255, 255, 102,255);
   CanvasRenderingContext2D::fill_rect(0., 0.,width, height);
   
   // draw blue triangle
   CanvasRenderingContext2D::begin_path();
   CanvasRenderingContext2D::set_fill_style_rgba(0, 0, 255,255);
   CanvasRenderingContext2D::move_to(20.,20.);
   CanvasRenderingContext2D::line_to(180.,20.);
   CanvasRenderingContext2D::line_to(130.,130.);
   CanvasRenderingContext2D::close_path();
   CanvasRenderingContext2D::fill();

   // clear part of the canvas
   CanvasRenderingContext2D::clear_rect(10., 10., 120., 120.);
}