use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const BACKGROUND_COLOR: Color = BLUE;

struct Ball {
    color: Color,
    size: i32,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Default for Ball {
    fn default() -> Ball {
        Ball {
            color: RED,
            size: 32,
            x: WINDOW_WIDTH / 2,
            y: WINDOW_HEIGHT / 2,
            dx: 5,
            dy: 5,
        }
    }
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("Big Peepee"),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut ball = Ball::default();

    loop {
        // UPDATE
        ball.x += ball.dx;
        if ball.x < 0 || ball.x > WINDOW_WIDTH - ball.size {
            ball.dx *= -1;
        }
        ball.y += ball.dy;
        if ball.y < 0 || ball.y > WINDOW_HEIGHT - ball.size {
            ball.dy *= -1;
        }

        // DRAW
        clear_background(BACKGROUND_COLOR);
        draw_rectangle(
            ball.x as f32,
            ball.y as f32,
            ball.size as f32,
            ball.size as f32,
            ball.color,
        );

        next_frame().await
    }
}
