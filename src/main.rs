use macroquad::prelude::*;

struct MainState {
    ball: Rect,
    x_vel: f32,
    y_vel: f32,
    top_paddle: Rect,
    bottom_paddle: Rect,
}

impl MainState {
    // since it accepts a &self parameter,
    // draw() uses an immutable reference
    // to a MainState instance
    fn draw(&self) {
        draw_rectangle(self.ball.x, self.ball.y, self.ball.w, self.ball.h, WHITE);
        draw_rectangle(self.top_paddle.x, self.top_paddle.y, self.top_paddle.w, self.top_paddle.h, WHITE);
        draw_rectangle(self.bottom_paddle.x, self.bottom_paddle.y, self.bottom_paddle.w, self.bottom_paddle.h, WHITE);
    }

    fn update(&mut self) {
        self.ball.x += self.x_vel;
        self.ball.y += self.y_vel;

        if self.ball.right() >= screen_width() || self.ball.left() <= 0.0 {
            self.x_vel *= -1.0;
        }
        if self.ball.bottom() >= screen_height() || self.ball.top() <= 0.0 {
            self.y_vel *= -1.0;
        }
    }
}

#[macroquad::main("Pong Game")]
async fn main() {
    let mut main_state = MainState {
        ball: Rect::new(screen_width() / 2.0, screen_height() / 2.0, 10.0, 10.0),
        x_vel: 5.0,
        y_vel: -5.0,
        top_paddle: Rect::new(screen_width() / 2.0, 15.0, 100.0, 15.0),
        bottom_paddle: Rect::new(screen_width() / 2.0, screen_height() - 30.0, 100.0, 15.0),
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
