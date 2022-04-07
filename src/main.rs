use macroquad::prelude::*;

struct MainState {
    x: f32, // 32 bit float
    y: f32,
    x_vel: f32,
    y_vel: f32,
}

#[macroquad::main("Pong Game")]
async fn main() {

    let mut main_state = MainState {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        x_vel: 5.0,
        y_vel: -5.0,
    };


    // equivalent to while(true)
    loop {
        clear_background(BLACK);

        draw_rectangle(main_state.x, main_state.y, 10.0, 10.0, WHITE);
        main_state.x += 10.0;
        main_state.y += 5.0;

        // let macroquad handle frame times,
        // input updates, etc
        next_frame().await
    }
}
