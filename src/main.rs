#![no_std]
#![no_main]
use crate::{
    ffi::{
        libc::{exit, srand, strcmp, time},
        raylib::{
            BeginDrawing, ClearBackground, CloseWindow, Color, EndDrawing, InitWindow,
            SetTargetFPS, WindowShouldClose,
        },
    },
    game::{game::Game, keyboard, Difficulty},
};

mod ffi;
mod game;
mod structs;

#[no_mangle]
unsafe extern "C" fn main(argc: i32, argv: *const *const c_char) -> i32 {
    println!("starting program...");
    srand(time(null_mut()) as u32);

    let mut diff = Difficulty::Easy;
    if argc > 2 {
        if strcmp(*argv.add(1), c"-d".as_ptr()) == 0 {
            diff = Difficulty::from(*argv.add(2))
        }
    }

    InitWindow(800, 600, c"Crusty Sokoban".as_ptr());
    SetTargetFPS(60);
    let mut game = Game::new(diff);
    while !WindowShouldClose() {
        keyboard::listener(&mut game, diff);
        BeginDrawing();
        ClearBackground(Color::hex(0x181818FF));
        game::render(game);
        EndDrawing();
    }

    println!("shutting down...");
    Game::destroy(game);
    CloseWindow();
    exit(0);
}

use core::{ffi::c_char, panic::PanicInfo, ptr::null_mut};

#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
