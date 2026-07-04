#![no_std]
#![no_main]
use crate::{
    ffi::{
        libc::{exit, free, malloc, srand, strcmp, time},
        raylib::{
            BeginDrawing, ClearBackground, CloseWindow, Color, EndDrawing, InitWindow,
            SetTargetFPS, WindowShouldClose,
        },
    },
    game::{game::Game, keyboard, Difficulty, Size},
};

mod ffi;
mod game;
mod structs;

pub struct State {
    pub loading: bool,
}

#[no_mangle]
unsafe extern "C" fn main(argc: i32, argv: *const *const c_char) -> i32 {
    println!("starting program...");
    srand(time(null_mut()) as u32);
    let state_ptr = malloc(size_of::<State>());

    let mut size = Size::Small;
    let mut difficulty = Difficulty::Easy;
    let mut arg_index = 1;

    while arg_index + 1 < argc {
        let flag = *argv.add(arg_index as usize);
        let value = *argv.add((arg_index + 1) as usize);

        if strcmp(flag, c"-s".as_ptr()) == 0 || strcmp(flag, c"--size".as_ptr()) == 0 {
            size = Size::from(value);
            arg_index += 2;
            continue;
        }

        if strcmp(flag, c"-d".as_ptr()) == 0 || strcmp(flag, c"--difficulty".as_ptr()) == 0 {
            difficulty = Difficulty::from(value);
            arg_index += 2;
            continue;
        }

        arg_index += 1;
    }

    InitWindow(800, 600, c"Crusty Sokoban".as_ptr());
    SetTargetFPS(60);

    *state_ptr = State { loading: true };
    let mut game: *mut Game = null_mut();

    BeginDrawing();
    ClearBackground(Color::hex(0x181818FF));
    game::render(game, state_ptr);
    EndDrawing();

    game = Game::new(size, difficulty, state_ptr);
    while !WindowShouldClose() {
        let reload_requested = keyboard::listener(&mut game, state_ptr, size, difficulty);

        BeginDrawing();
        ClearBackground(Color::hex(0x181818FF));
        game::render(game, state_ptr);
        EndDrawing();

        if reload_requested {
            Game::destroy(game);
            game = Game::new(size, difficulty, state_ptr);
        }
    }

    println!("shutting down...");
    Game::destroy(game);
    free(state_ptr);
    CloseWindow();
    exit(0);
}

use core::{ffi::c_char, panic::PanicInfo, ptr::null_mut};

#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
