#![no_main]

use raylib_sys::{
    BeginDrawing, ClearBackground, CloseWindow, Color, DrawText, EndDrawing, InitWindow,
    SetTargetFPS, WindowShouldClose,
};

#[no_mangle]
unsafe fn main() {
    let color_white = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    let color_black = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    let screen_width = 800;
    let screen_height = 600;

    // Initialize the Raylib window
    InitWindow(screen_width, screen_height, c"Hello World".as_ptr());
    SetTargetFPS(60);

    while !WindowShouldClose() {
        // Begin drawing on the window
        BeginDrawing();

        // Clear the background with a white color
        ClearBackground(color_white);

        // Draw the "Hello World" text at position (12, 12) with a font size of 20 and black color
        DrawText(c"Hello World".as_ptr(), 12, 12, 20, color_black);
        EndDrawing();
    }

    CloseWindow();
}
