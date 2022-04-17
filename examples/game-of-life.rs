use js_canvas_rendering_context_2d::*;



fn main(){
    draw();
}


#[no_mangle]
pub extern "C" fn draw(){
    UNIVERSE.with(|universe|{
        let borrow = universe.borrow();
        borrow.draw();
    });
}


#[no_mangle]
pub extern "C" fn tick(){
    UNIVERSE.with(|universe|{
        let mut borrow = universe.borrow_mut();
        borrow.tick();
    });
}

thread_local! {
    pub static UNIVERSE: std::cell::RefCell<Box<Universe>> = std::cell::RefCell::new(Box::new(Universe{..Default::default()}));
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn draw(&self){
        let (canvas_w,canvas_h) = get_canvas_dimensions();
        let (offset_x, offset_y) = get_offsets(canvas_w, canvas_h);
        let coeff = get_coeff(canvas_w, canvas_h);

        
        CanvasRenderingContext2D::clear_rect(0., 0., canvas_w, canvas_h);

        let cell_width = 100.0 / self.width as f32;
        let cell_height = 100.0 / self.height as f32;
        let mut x_i = 0;
        let mut y_i= 0;

        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                if cell == Cell::Alive{
                    let start_x = apply_transformation(x_i as f32 * cell_width, coeff, offset_x);
                    let start_y = apply_transformation(y_i as f32 * cell_height, coeff, offset_y);


                    CanvasRenderingContext2D::begin_path();
                    CanvasRenderingContext2D::set_fill_style_rgba(200, 200, 200,255);
                    CanvasRenderingContext2D::fill_rect(start_x, start_y,cell_width, cell_height);
                }
                
                x_i += 1;
            }
            x_i = 0;
            y_i += 1;
        }
    }
}

impl Default for Universe {
    fn default() -> Self {
        let width = 16;
        let height = 16;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }
}


fn apply_transformation(source: f32, coeff: f32, offset: f32) -> f32 {
    offset + (coeff * source)
 }
 
 fn get_offsets(canvas_w:f32, canvas_h:f32) -> (f32,f32){
 
    let square_side = canvas_w.min(canvas_h);
    let offset_x = (canvas_w - square_side) / 2.;
    let offset_y = (canvas_h - square_side) / 2.;
    (offset_x,offset_y)
 }
 
 fn get_coeff(canvas_w:f32, canvas_h:f32) -> f32{ 
    let square_side = canvas_w.min(canvas_h);
 
    square_side / 100.0
 }

 fn get_canvas_dimensions() -> (f32, f32){
     (CanvasRenderingContext2D::get_canvas_width(),CanvasRenderingContext2D::get_canvas_height())
 }