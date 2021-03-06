use macroquad::prelude::*;

struct MainState {
    ball: Rect,
    x_vel: f32,
    y_vel: f32,
    top_paddle: Rect,
    bottom_paddle: Rect,
    top_score: u32,
    bottom_score: u32,
}

impl MainState {

    // since it accepts a &self parameter, draw() uses an immutable reference to a MainState instance
    fn draw(&self) {
        draw_rectangle(self.ball.x, self.ball.y, self.ball.w, self.ball.h, YELLOW);
        draw_rectangle(self.top_paddle.x, self.top_paddle.y, self.top_paddle.w, self.top_paddle.h, RED);
        draw_rectangle(self.bottom_paddle.x, self.bottom_paddle.y, self.bottom_paddle.w, self.bottom_paddle.h, BLUE);

        let t1: String = format!("{}{}", "Red Score: ", &self.top_score.to_string());
        draw_text(&t1.to_owned(), screen_width() - 120.0, 50.0, 10.0, WHITE);
        let t2: String = format!("{}{}", "Blue Score: ", &self.bottom_score.to_string());
        draw_text(&t2.to_owned(), screen_width() - 120.0, 70.0, 10.0, WHITE);
    }

    fn update(&mut self) {
        self.ball.x += self.x_vel;
        self.ball.y += self.y_vel;

        // Ball bouncing off edges
        if self.ball.right() >= screen_width() || self.ball.left() <= 0.0 {
            self.x_vel *= -1.0;
        }
        if self.ball.bottom() >= (screen_height() - 15.0) {
            self.y_vel *= -1.0;
            self.top_score += 1;
        }
        if self.ball.top() <= 15.0 {
            self.y_vel *= -1.0;
            self.bottom_score += 1;
        }

        // Ball bouncing off paddle
        if self.ball.overlaps(&self.top_paddle) || self.ball.overlaps(&self.bottom_paddle) {
            self.y_vel *= -1.0;
        }

        // Keypress actions
        if is_key_down(KeyCode::Left) && self.bottom_paddle.x >= 0.0 {
            self.bottom_paddle.x -= 10.0
        }
        if is_key_down(KeyCode::Right) && self.bottom_paddle.x <= (screen_width() - 120.0) {
            self.bottom_paddle.x += 10.0
        }
        if is_key_down(KeyCode::A) && self.top_paddle.x >= 0.0 {
            self.top_paddle.x -= 10.0
        }
        if is_key_down(KeyCode::D) && self.top_paddle.x <= (screen_width() - 120.0) {
            self.top_paddle.x += 10.0
        }
    }
}

#[macroquad::main("Pong Game")]
async fn main() {
    let mut main_state = MainState {
        ball: Rect::new(screen_width() / 2.0, screen_height() / 2.0, 10.0, 10.0),
        x_vel: 5.0,
        y_vel: -5.0,
        top_paddle: Rect::new(screen_width() / 2.0 - 60.0, 15.0, 120.0, 15.0),
        bottom_paddle: Rect::new(screen_width() / 2.0 - 60.0, screen_height() - 30.0, 120.0, 15.0),
        top_score: 0,
        bottom_score: 0,
    };

    // equivalent to while(true)
    loop {
        clear_background(BLACK);

        main_state.draw();
        main_state.update();

        // let macroquad handle frame times,
        // input updates, etc
        next_frame().await
    }
}
