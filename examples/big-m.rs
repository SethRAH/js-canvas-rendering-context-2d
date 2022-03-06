use js_canvas_rendering_context_2d::*;

fn main(){
// Big M
   CanvasRenderingContext2D::move_to(90.,130.);
   CanvasRenderingContext2D::line_to(95.,25.);
   CanvasRenderingContext2D::line_to(150.,80.);
   CanvasRenderingContext2D::line_to(205.,25.);
   CanvasRenderingContext2D::line_to(210.,130.);
   CanvasRenderingContext2D::set_line_width(15);
   CanvasRenderingContext2D::stroke();
}