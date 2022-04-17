use js_canvas_rendering_context_2d::*;

fn main(){
// Big M
  draw();
}

#[no_mangle]
pub extern "C" fn draw(){
   let (x_offset, y_offset) = get_offsets();
   let coeff = get_coeff();


   local_move_to(5.,95.,x_offset,y_offset,coeff);
   local_line_to(10.,15.,x_offset,y_offset,coeff);
   local_line_to(50.,70.,x_offset,y_offset,coeff);
   local_line_to(90.,15.,x_offset,y_offset,coeff);
   local_line_to(95.,95.,x_offset,y_offset,coeff);

   CanvasRenderingContext2D::set_line_width(15 * coeff.floor() as u32);
   CanvasRenderingContext2D::stroke();
}

fn local_move_to(x:f32, y:f32,x_offset:f32, y_offset: f32, coeff: f32) {
   let local_x = apply_transformation(x, coeff, x_offset);
   let local_y = apply_transformation(y, coeff, y_offset);

   CanvasRenderingContext2D::move_to(local_x, local_y);
}

fn local_line_to(x:f32, y:f32,x_offset:f32, y_offset: f32, coeff: f32) {
   let local_x = apply_transformation(x, coeff, x_offset);
   let local_y = apply_transformation(y, coeff, y_offset);

   CanvasRenderingContext2D::line_to(local_x, local_y);
}

fn apply_transformation(source: f32, coeff: f32, offset: f32) -> f32 {
   offset + (coeff * source)
}

fn get_offsets() -> (f32,f32){
   let canvas_w = CanvasRenderingContext2D::get_canvas_width();
   let canvas_h = CanvasRenderingContext2D::get_canvas_height();

   let square_side = canvas_w.min(canvas_h);
   let offset_x = (canvas_w - square_side) / 2.;
   let offset_y = (canvas_h - square_side) / 2.;
   (offset_x,offset_y)
}

fn get_coeff() -> f32{
   let canvas_w = CanvasRenderingContext2D::get_canvas_width();
   let canvas_h = CanvasRenderingContext2D::get_canvas_height();

   let square_side = canvas_w.min(canvas_h);

   square_side / 100.0
}