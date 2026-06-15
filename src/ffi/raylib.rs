use core::ffi::{c_char, c_int};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub unsafe fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        return Color { r, g, b, a };
    }

    pub unsafe fn hex(hex: u32) -> Self {
        return if hex > 0xFFFFFF {
            Self::new(
                ((hex >> 24) & 0xFF) as u8,
                ((hex >> 16) & 0xFF) as u8,
                ((hex >> 8) & 0xFF) as u8,
                (hex & 0xFF) as u8,
            )
        } else {
            Self::new(
                ((hex >> 16) & 0xFF) as u8,
                ((hex >> 8) & 0xFF) as u8,
                (hex & 0xFF) as u8,
                255,
            )
        };
    }
}

extern "C" {
    // Window Handling
    pub fn InitWindow(width: c_int, height: c_int, title: *const c_char);
    pub fn WindowShouldClose() -> bool;
    pub fn SetWindowTitle(title: *const c_char);
    pub fn CloseWindow();
    pub fn GetScreenWidth() -> c_int;
    pub fn GetScreenHeight() -> c_int;
    // Drawing
    pub fn BeginDrawing();
    pub fn EndDrawing();
    pub fn ClearBackground(color: Color);
    pub fn DrawRectangle(x: c_int, y: c_int, width: c_int, height: c_int, color: Color);
    // Text
    pub fn _DrawText(text: *const c_char, x: c_int, y: c_int, size: c_int, color: Color);
    pub fn TextFormat(text: *const c_char, ...) -> *const c_char;
    // Timing
    pub fn SetTargetFPS(fps: c_int);
    // Keyboard Handling
    pub fn IsKeyPressed(key: Keyboard) -> bool;
}

#[repr(C)]
#[expect(unused)]
pub enum Keyboard {
    KeyNull = 0, // Key: NULL, used for no key pressed
    // Alphanumeric keys
    KeyApostrophe = 39,   // Key: '
    KeyComma = 44,        // Key: ,
    KeyMinus = 45,        // Key: -
    KeyPeriod = 46,       // Key: .
    KeySlash = 47,        // Key: /
    KeyZero = 48,         // Key: 0
    KeyOne = 49,          // Key: 1
    KeyTwo = 50,          // Key: 2
    KeyThree = 51,        // Key: 3
    KeyFour = 52,         // Key: 4
    KeyFive = 53,         // Key: 5
    KeySix = 54,          // Key: 6
    KeySeven = 55,        // Key: 7
    KeyEight = 56,        // Key: 8
    KeyNine = 57,         // Key: 9
    KeySemicolon = 59,    // Key: ;
    KeyEqual = 61,        // Key: =
    KeyA = 65,            // Key: A | a
    KeyB = 66,            // Key: B | b
    KeyC = 67,            // Key: C | c
    KeyD = 68,            // Key: D | d
    KeyE = 69,            // Key: E | e
    KeyF = 70,            // Key: F | f
    KeyG = 71,            // Key: G | g
    KeyH = 72,            // Key: H | h
    KeyI = 73,            // Key: I | i
    KeyJ = 74,            // Key: J | j
    KeyK = 75,            // Key: K | k
    KeyL = 76,            // Key: L | l
    KeyM = 77,            // Key: M | m
    KeyN = 78,            // Key: N | n
    KeyO = 79,            // Key: O | o
    KeyP = 80,            // Key: P | p
    KeyQ = 81,            // Key: Q | q
    KeyR = 82,            // Key: R | r
    KeyS = 83,            // Key: S | s
    KeyT = 84,            // Key: T | t
    KeyU = 85,            // Key: U | u
    KeyV = 86,            // Key: V | v
    KeyW = 87,            // Key: W | w
    KeyX = 88,            // Key: X | x
    KeyY = 89,            // Key: Y | y
    KeyZ = 90,            // Key: Z | z
    KeyLeftBracket = 91,  // Key: [
    KeyBackslash = 92,    // Key: '\'
    KeyRightBracket = 93, // Key: ]
    KeyGrave = 96,        // Key: `
    // Function keys
    KeySpace = 32,         // Key: Space
    KeyEscape = 256,       // Key: Esc
    KeyEnter = 257,        // Key: Enter
    KeyTab = 258,          // Key: Tab
    KeyBackspace = 259,    // Key: Backspace
    KeyInsert = 260,       // Key: Ins
    KeyDelete = 261,       // Key: Del
    KeyRight = 262,        // Key: Cursor right
    KeyLeft = 263,         // Key: Cursor left
    KeyDown = 264,         // Key: Cursor down
    KeyUp = 265,           // Key: Cursor up
    KeyPageUp = 266,       // Key: Page up
    KeyPageDown = 267,     // Key: Page down
    KeyHome = 268,         // Key: Home
    KeyEnd = 269,          // Key: End
    KeyCapsLock = 280,     // Key: Caps lock
    KeyScrollLock = 281,   // Key: Scroll down
    KeyNumLock = 282,      // Key: Num lock
    KeyPrintScreen = 283,  // Key: Print screen
    KeyPause = 284,        // Key: Pause
    KeyF1 = 290,           // Key: F1
    KeyF2 = 291,           // Key: F2
    KeyF3 = 292,           // Key: F3
    KeyF4 = 293,           // Key: F4
    KeyF5 = 294,           // Key: F5
    KeyF6 = 295,           // Key: F6
    KeyF7 = 296,           // Key: F7
    KeyF8 = 297,           // Key: F8
    KeyF9 = 298,           // Key: F9
    KeyF10 = 299,          // Key: F10
    KeyF11 = 300,          // Key: F11
    KeyF12 = 301,          // Key: F12
    KeyLeftShift = 340,    // Key: Shift left
    KeyLeftControl = 341,  // Key: Control left
    KeyLeftAlt = 342,      // Key: Alt left
    KeyLeftSuper = 343,    // Key: Super left
    KeyRightShift = 344,   // Key: Shift right
    KeyRightControl = 345, // Key: Control right
    KeyRightAlt = 346,     // Key: Alt right
    KeyRightSuper = 347,   // Key: Super right
    KeyKbMenu = 348,       // Key: KB menu
    // Keypad keys
    KeyKp0 = 320,        // Key: Keypad 0
    KeyKp1 = 321,        // Key: Keypad 1
    KeyKp2 = 322,        // Key: Keypad 2
    KeyKp3 = 323,        // Key: Keypad 3
    KeyKp4 = 324,        // Key: Keypad 4
    KeyKp5 = 325,        // Key: Keypad 5
    KeyKp6 = 326,        // Key: Keypad 6
    KeyKp7 = 327,        // Key: Keypad 7
    KeyKp8 = 328,        // Key: Keypad 8
    KeyKp9 = 329,        // Key: Keypad 9
    KeyKpDecimal = 330,  // Key: Keypad .
    KeyKpDivide = 331,   // Key: Keypad /
    KeyKpMultiply = 332, // Key: Keypad *
    KeyKpSubtract = 333, // Key: Keypad -
    KeyKpAdd = 334,      // Key: Keypad +
    KeyKpEnter = 335,    // Key: Keypad Enter
    KeyKpEqual = 336,    // Key: Keypad =
    // Android key buttons
    KeyBack = 4,        // Key: Android back button
    KeyMenu = 5,        // Key: Android menu button
    KeyVolumeUp = 24,   // Key: Android volume up button
    KeyVolumeDown = 25, // Key: Android volume down button
}
