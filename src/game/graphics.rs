use crate::{
    ffi::raylib::{Color, DrawRectangle},
    game::{
        consts::{BOX_COLOR, CELL_COLOR, GOAL_COLOR, PLAYER_COLOR},
        get_board,
        logic::{get_cells, CellType},
        Game,
    },
};

pub unsafe fn render(game: *mut Game) {
    // Draw board
    let cells = *get_cells(get_board(game));
    for i in 0..cells.count {
        let inner_array = &cells.items.add(i).read();
        for j in 0..inner_array.count {
            let cell = *inner_array.items.add(j);
            DrawRectangle(
                cell.vec.x,
                cell.vec.y,
                cell.width,
                cell.height,
                match cell.cell_type {
                    CellType::Player => Color::hex(PLAYER_COLOR),
                    CellType::Empty => Color::hex(CELL_COLOR),
                    CellType::Box => Color::hex(BOX_COLOR),
                    CellType::Goal => Color::hex(GOAL_COLOR),
                },
            );
        }
    }
}
