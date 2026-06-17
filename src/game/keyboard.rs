use crate::{
    ffi::raylib::{IsKeyPressed, Keyboard},
    game::{game, Difficulty, Direction, Game},
};

pub unsafe fn listener(game: *mut *mut Game, diff: Difficulty) {
    if (*(*game)).playing {
        if IsKeyPressed(Keyboard::KeyW) {
            game::move_player(*game, Direction::Up);
        } else if IsKeyPressed(Keyboard::KeyA) {
            game::move_player(*game, Direction::Left);
        } else if IsKeyPressed(Keyboard::KeyS) {
            game::move_player(*game, Direction::Down);
        } else if IsKeyPressed(Keyboard::KeyD) {
            game::move_player(*game, Direction::Right);
        }
    }

    if IsKeyPressed(Keyboard::KeyEnter) || IsKeyPressed(Keyboard::KeySpace) {
        Game::destroy(*game);
        *game = Game::new(diff)
    }
}
