use crate::{
    ffi::raylib::{Color, DrawRectangle, DrawText, GetScreenHeight, TextFormat},
    game::{
        consts::*,
        get_board, get_undo_stack,
        logic::{
            get_cells,
            CellBase::{Floor, Goal, Wall},
            CellEntity::{Box, None, Player},
        },
        Game,
    },
    State,
};

pub unsafe fn render(game: *mut Game, state: *mut State) {
    if game.is_null() {
        if (*state).loading {
            DrawText(
                c"Loading...".as_ptr(),
                2,
                GetScreenHeight() - 22,
                20,
                Color::hex(TEXT_COLOR),
            );
        }
        return;
    }

    // Draw board
    let cells = *get_cells(get_board(game));
    for i in 0..cells.count {
        let inner_array = &cells.items.add(i).read();
        for j in 0..inner_array.count {
            let cell = *inner_array.items.add(j);

            let base_color = match cell.base {
                Floor => Color::hex(CELL_COLOR),
                Wall => Color::hex(WALL_COLOR),
                Goal => match cell.entity {
                    None => Color::hex(GOAL_COLOR),
                    Player => Color::hex(GOAL_COLOR),
                    Box => Color::hex(GOAL_SUCCESS_COLOR),
                },
            };

            DrawRectangle(cell.vec.x, cell.vec.y, CELL_SIZE, CELL_SIZE, base_color);

            let inset = 2;
            let entity_size = CELL_SIZE - inset * 2;

            match cell.entity {
                Player => DrawRectangle(
                    cell.vec.x + inset,
                    cell.vec.y + inset,
                    entity_size,
                    entity_size,
                    Color::hex(PLAYER_COLOR),
                ),
                Box => DrawRectangle(
                    cell.vec.x + inset,
                    cell.vec.y + inset,
                    entity_size,
                    entity_size,
                    match cell.base {
                        Floor => Color::hex(BOX_COLOR),
                        Goal => Color::hex(BOX_SUCCESS_COLOR),
                        Wall => Color::hex(BOX_COLOR),
                    },
                ),
                None => {}
            };
        }
    }

    // Draw score
    let score = (*game).score;
    let top_score = (*game).top_score;
    DrawText(
        TextFormat(c"Score: %i/%i".as_ptr(), score, top_score),
        2,
        2,
        20,
        Color::hex(TEXT_COLOR),
    );

    // Draw moves
    let moves = (*get_undo_stack(game)).stack.count;
    let top_moves = (*game).top_moves;
    DrawText(
        TextFormat(c"Moves: %i/%i".as_ptr(), moves, top_moves),
        2,
        2 + 20 + 2,
        20,
        Color::hex(TEXT_COLOR),
    );

    if (*state).loading {
        DrawText(
            c"Loading...".as_ptr(),
            2,
            GetScreenHeight() - (20 + 2),
            20,
            Color::hex(TEXT_COLOR),
        );
    }
}
