use crate::{
    ffi::raylib::{GetScreenHeight, GetScreenWidth},
    game::{
        consts::CELL_SIZE,
        logic::{
            Cell,
            CellBase::{Floor, Goal, Wall},
            CellEntity::{Box, None, Player},
        },
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
    pub unsafe fn new(
        rows: i32,
        cols: i32,
        start_pos: Vector2,
        boxes: Array<Vector2>,
        goals: Array<Vector2>,
        walls: Array<Vector2>,
    ) -> Self {
        let mut outer: Array<Array<Cell>> = Array::new();
        let pos_x = (GetScreenWidth() - (cols * CELL_SIZE + (cols - 1))) / 2;
        let pos_y = (GetScreenHeight() - (rows * CELL_SIZE + (rows - 1))) / 2;
        let mut x: i32 = pos_x;
        let mut y: i32 = pos_y;

        for r in 0..rows {
            let mut inner: Array<Cell> = Array::new();
            for c in 0..cols {
                let pos = Vector2::new(c, r);

                let base = if has_pos(goals, pos) {
                    Goal
                } else if has_pos(walls, pos) {
                    Wall
                } else {
                    Floor
                };

                let entity = if r == start_pos.y && c == start_pos.x {
                    Player
                } else if has_pos(boxes, pos) {
                    Box
                } else {
                    None
                };

                append(&mut inner, Cell::new(Vector2::new(x, y), base, entity));
                x += CELL_SIZE + 1;
            }
            append(&mut outer, inner);
            x = pos_x;
            y += CELL_SIZE + 1;
        }

        Self { cells: outer }
    }

    pub unsafe fn destroy(board: *mut Self) {
        if board.is_null() {
            return;
        }

        let cells = &mut (*board).cells;
        for i in 0..cells.count {
            let row = cells.items.add(i);
            Array::destroy(row);
        }

        Array::destroy(cells);
    }
}

unsafe fn has_pos(arr: Array<Vector2>, vec: Vector2) -> bool {
    for i in 0..arr.count {
        let current_pos = *arr.items.add(i);
        if current_pos.x == vec.x && current_pos.y == vec.y {
            return true;
        }
    }
    false
}
