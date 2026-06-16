#![no_std]
#![no_main]
use crate::{
    ffi::{
        libc::exit,
        raylib::{
            BeginDrawing, ClearBackground, CloseWindow, Color, EndDrawing, InitWindow,
            SetTargetFPS, WindowShouldClose,
        },
    },
    game::{game::Game, keyboard},
};

mod ffi;
mod game;
mod structs;

#[no_mangle]
unsafe extern "C" fn main() {
    println!("starting program...");

    InitWindow(800, 600, c"Crusty Sokoban".as_ptr());
    SetTargetFPS(60);
    let mut game = Game::new();
    while !WindowShouldClose() {
        if (*game).playing {
            keyboard::listener(&mut game);
        }
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

use core::panic::PanicInfo;

#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
