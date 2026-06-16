use crate::{
    ffi::raylib::{Color, DrawRectangle, DrawText, TextFormat},
    game::{
        consts::{BOX_COLOR, CELL_COLOR, GOAL_COLOR, PLAYER_COLOR, WALL_COLOR},
        get_board,
        logic::{
            get_cells,
            CellBase::{Floor, Goal, Wall},
            CellEntity::{Box, None, Player},
        },
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

            let color = match cell.entity {
                Player => Color::hex(PLAYER_COLOR),
                Box => Color::hex(BOX_COLOR),
                None => match cell.base {
                    Floor => Color::hex(CELL_COLOR),
                    Wall => Color::hex(WALL_COLOR),
                    Goal => Color::hex(GOAL_COLOR),
                },
            };

            DrawRectangle(cell.vec.x, cell.vec.y, cell.width, cell.height, color);
        }
    }

    // Draw score
    let score = (*game).score;
    let top_score = (*game).top_score;
    DrawText(
        TextFormat(c"Score %i/%i".as_ptr(), score, top_score),
        2,
        2,
        20,
        Color::hex(0xEFEFEFFF),
    );
}
