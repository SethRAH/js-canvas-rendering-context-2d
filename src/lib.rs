extern "C" {
    // property getters/setters
    fn js_get_line_cap() -> u32;
    fn js_set_line_cap(value: u32);
    fn js_get_line_join() -> u32;
    fn js_set_line_join(value: u32);
    fn js_get_line_width() -> u32;
    fn js_set_line_width(value: u32);
    fn js_set_stroke_style_rgba(r: u8, g: u8, b: u8, a: u8);

    // functions
    fn js_arc(x: f32, y:f32, radius: f32, start_angle: f32, end_angle: f32, counter_clockwise: bool);
    fn js_arc_to(x1: f32, y1: f32, x2: f32, y2: f32, radius: f32);
    fn js_begin_path();
    fn js_line_to(x: f32, y:f32);
    fn js_move_to(x: f32, y:f32);
    fn js_stroke();
}

pub struct CanvasRenderingContext2D{}

impl CanvasRenderingContext2D{
    /// getter for property that determines the shape used to join two line segments where they meet.
    /// see [CanvasRenderingContext2D.lineJap](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJap)
    pub fn get_line_join() -> LineJoin {        
        unsafe {
            let raw_join_val = js_get_line_join();
            return match raw_join_val {
                1 => LineJoin::Bevel,
                2 => LineJoin::Round,
                3 => LineJoin::Miter,
                _ => LineJoin::Bevel
            };            
        }
    }

    /// setter for property that determines the shape used to join two line segments where they meet.
    /// see [CanvasRenderingContext2D.lineJoin](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJoin)
    /// 
    /// # Parameters
    /// 
    /// * 'value' - the type of line join. Valid options are Bevel, Round, Miter
    pub fn set_line_join(value: LineJoin){
        let join_val = match value {
            LineJoin::Bevel => 1,
            LineJoin::Round => 2,
            LineJoin::Miter => 3,
        };

        unsafe{
            js_set_line_join(join_val);
        }
    }
    /// getter for property that determines the shape used to draw the end points of lines.
    /// see [CanvasRenderingContext2D.lineCap](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)
    pub fn get_line_cap() -> LineCap {        
        unsafe {
            let raw_cap_val = js_get_line_cap();
            return match raw_cap_val {
                1 => LineCap::Butt,
                2 => LineCap::Round,
                3 => LineCap::Square,
                _ => LineCap::Butt
            };            
        }
    }

    /// setter for property that determines the shape used to draw the end points of lines.
    /// see [CanvasRenderingContext2D.lineCap](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)
    /// 
    /// # Parameters
    /// 
    /// * 'value' - the type of line cap. Valid options are Butt, Round, Square
    pub fn set_line_cap(value: LineCap){
        let cap_val = match value {
            LineCap::Butt => 1,
            LineCap::Round => 2,
            LineCap::Square => 3,
        };

        unsafe{
            js_set_line_cap(cap_val);
        }
    }

    /// getter for property that sets the thickness of lines.
    /// see [CanvasRenderingContext2D.lineWidth](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth)
    pub fn get_line_width() -> u32 {
        unsafe {
            return js_get_line_width();
        }
    }

    /// setter for property that sets the thickness of lines.
    /// see [CanvasRenderingContext2D.lineWidth](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth)
    /// 
    /// # Parameters
    /// 
    /// * 'value' - the thickness of lines. Must be a non-zero positive number
    pub fn set_line_width(value: u32){
        if value == 0 {
            panic!("value must not be 0");
        }        
        unsafe{
            js_set_line_width(value);
        }
    }

    /// setter for property that specifies the color to use for the strokes (outlines) around shapes. The default is black.
    /// see [CanvasRenderingContext2D.strokeStyle](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)
    /// 
    /// # Parameters
    /// 
    /// * 'r' - value from 0 to 255 that represents how much red is in the color
    /// * 'g' - value from 0 to 255 that represents how much green is in the color
    /// * 'b' - value from 0 to 255 that represents how much blue is in the color
    /// * 'a' - value from 0 to 255 that represents how opaque the color is
    pub fn set_stroke_style_rgba(r: u8, g: u8, b: u8, a: u8){
        unsafe{
            js_set_stroke_style_rgba(r,g,b,a);
        }
    }


    /// adds a circular arc to the current sub-path. 
    /// see [CanvasRenderingContext2D.arc](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arc)
    /// 
    /// # Parameters
    /// 
    /// * 'x' - the horizontal coordinate of the arc's center
    /// * 'y' - the vertical coordinate of the arc's center
    /// * 'radius' - the arc's radius. Must be positive
    /// * 'start_angle' - the angle at which the arc starts in radians, measured from the positive x-axis
    /// * 'end_angle' - the angle at which the arc ends in radians, measured from the positive x-axis
    /// * 'counter_clockwise' - if true, draws the arc counter-clockwise between the start and end angles
    pub fn arc(x: f32, y:f32, radius: f32, start_angle: f32, end_angle: f32, counter_clockwise: bool){
        if radius <= 0. {
            panic!("radius must be positive");
        }
        unsafe {
            js_arc(x, y, radius, start_angle, end_angle, counter_clockwise)
        }
    }

    /// adds a circular arc to the current sub-path, using the given control points and radius. 
    /// The arc is automatically connected to the path's latest point with a straight line, if necessary for the specified parameters. 
    /// see [CanvasRenderingContext2D.arcTo](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arcTo)
    /// 
    /// # Parameters
    /// 
    /// * 'x1' - the x-axis coordinate of the first control point
    /// * 'y1' - the y-axis coordinate of the first control point.
    /// * 'x2' - the x-axis coordinate of the second control point
    /// * 'y2' - the y-axis coordinate of the second control point.
    /// * 'radius' - the arc's radius. Must be positive
    pub fn arc_to(x1: f32, y1: f32, x2: f32, y2: f32, radius: f32){
        if radius <= 0. {
            panic!("radius must be positive");
        }
        unsafe {
            js_arc_to(x1, y1, x2, y2, radius)
        }
    }    

    /// starts a new path by emptying the list of sub-paths. Call this method when you want to create a new path.
    /// see [CanvasRenderingContext2D.beginPath](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/beginPath)
    /// 
    /// # Parameters (none)
    pub fn begin_path(){
        unsafe {
            js_begin_path()
        }
    }

    /// adds a straight line to the current sub-path by connecting the sub-path's last point to the specified (x, y) coordinates.
    /// see [CanvasRenderingContext2D.lineTo](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineTo)
    /// 
    /// # Parameters
    /// 
    /// * 'x' - the x-axis coordinate of the line's end point
    /// * 'y' - the y-axis coordinate of the line's end point.
    pub fn line_to(x: f32, y: f32){
        unsafe {
            js_line_to(x, y)
        }
    }

    /// begins a new sub-path at the point specified by the given (x, y) coordinates.
    /// see [CanvasRenderingContext2D.moveTo](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/moveTo)
    /// 
    /// # Parameters
    /// 
    /// * 'x' - the x-axis coordinate of the point
    /// * 'y' - the y-axis coordinate of the point.
    pub fn move_to(x: f32, y: f32){
        unsafe {
            js_move_to(x, y)
        }
    }   

    /// strokes (outlines) the current or given path with the current stroke style.
    /// see [CanvasRenderingContext2D.stroke](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/stroke)
    /// 
    /// # Parameters (none)
    pub fn stroke(){
        unsafe {
            js_stroke()
        }
    }
}

pub enum LineCap {
    Butt,
    Round,
    Square
}

pub enum LineJoin {
    Bevel,
    Round,
    Miter
}