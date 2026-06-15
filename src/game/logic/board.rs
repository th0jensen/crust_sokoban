use crate::{
    ffi::raylib::{GetScreenHeight, GetScreenWidth},
    game::{
        consts::{CELL_HEIGHT, CELL_WIDTH},
        logic::{Cell, CellType},
    },
    structs::{append, Array, Vector2},
};

pub type Cells = Array<Array<Cell>>;

#[derive(Clone, Copy)]
pub struct Board {
    cells: Cells,
}

pub unsafe fn get_cells(board: *mut Board) -> *mut Cells {
    &mut (*board).cells
}

impl Board {
    pub unsafe fn new(rows: i32, cols: i32, start_pos: Vector2, boxes: Array<Vector2>) -> Self {
        let mut outer: Array<Array<Cell>> = Array::new();
        let pos_x = (GetScreenWidth() - (cols * CELL_WIDTH + (cols - 1))) / 2;
        let pos_y = (GetScreenHeight() - (rows * CELL_HEIGHT + (rows - 1))) / 2;
        let mut x: i32 = pos_x;
        let mut y: i32 = pos_y;

        for r in 0..rows {
            let mut inner: Array<Cell> = Array::new();
            for c in 0..cols {
                let mut has_box = false;

                for i in 0..boxes.count {
                    let box_pos = *boxes.items.add(i);
                    if box_pos.x == c && box_pos.y == r {
                        has_box = true;
                        break;
                    }
                }

                let cell = if r == start_pos.y && c == start_pos.x {
                    Cell::new(Vector2::new(x, y), CellType::Player)
                } else if has_box {
                    Cell::new(Vector2::new(x, y), CellType::Box)
                } else {
                    Cell::new(Vector2::new(x, y), CellType::Empty)
                };
                append(&mut inner, cell);
                x += CELL_WIDTH + 1;
            }
            append(&mut outer, inner);
            x = pos_x;
            y += CELL_HEIGHT + 1;
        }

        Self { cells: outer }
    }
}
