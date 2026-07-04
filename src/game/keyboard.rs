use crate::{
    ffi::raylib::{IsKeyPressed, Keyboard},
    game::{game, Difficulty, Direction, Game, Size},
    State,
};

pub unsafe fn listener(
    game: *mut *mut Game,
    state: *mut State,
    _size: Size,
    _difficulty: Difficulty,
) -> bool {
    if (*state).loading || (*game).is_null() {
        return false;
    }

    if IsKeyPressed(Keyboard::KeyBackspace) {
        game::restart_level(*game);
        return false;
    }

    if IsKeyPressed(Keyboard::KeyR) {
        game::play_solution(*game);
        return false;
    }

    if (*(*game)).solution_playing {
        game::step_solution(*game);
        return false;
    }

    if (*(*game)).playing {
        if IsKeyPressed(Keyboard::KeyW) {
            game::move_player(*game, Direction::Up);
        } else if IsKeyPressed(Keyboard::KeyA) {
            game::move_player(*game, Direction::Left);
        } else if IsKeyPressed(Keyboard::KeyS) {
            game::move_player(*game, Direction::Down);
        } else if IsKeyPressed(Keyboard::KeyD) {
            game::move_player(*game, Direction::Right);
        } else if IsKeyPressed(Keyboard::KeyU) {
            game::undo_move(*game);
        }
    }

    if IsKeyPressed(Keyboard::KeyEnter) || IsKeyPressed(Keyboard::KeySpace) {
        (*state).loading = true;
        return true;
    }

    false
}
