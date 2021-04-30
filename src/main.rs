use rand::{Rng, SeedableRng, prelude::StdRng};
use raylib::prelude::*;

// Game config
pub const ARENA_COLOR: Color = Color::BLACK;
pub const OBJECT_COLOR: Color = Color::WHITE;

pub const GAME_WIDTH: f32 = 1280.0;
pub const GAME_HEIGHT: f32 = 720.0;

pub const PADDLE_OFFSET: f32 = 50.0;
pub const PADDLE_SPEED: f32 = 500.0;
pub const PADDLE_WIDTH: f32  = 32.0;
pub const PADDLE_HEIGHT: f32 = 150.0;

pub const BALL_SPEED: f32 = 500.0;
pub const BALL_WIDTH: f32 = 32.0;
pub const BALL_HEIGHT: f32 = 32.0;

/** Represents the paddle */
struct Paddle {
    up_key: KeyboardKey,
    down_key: KeyboardKey,

    x: f32,
    y: f32,

    points: u32,
}

impl Paddle {
    /** Construct a Paddle */
    pub fn new(x_pos: f32, up_key: KeyboardKey, down_key: KeyboardKey) -> Self {
        return Self {
            x: x_pos,
            y: GAME_HEIGHT / 2.0,

            up_key: up_key,
            down_key: down_key,

            points: 0,
        };
    }

    /** Update a paddle to the screen */
    pub fn update(&mut self, rl: &mut RaylibHandle) {
        if rl.is_key_down(self.up_key) && self.y > 0.0 {
            self.y -= PADDLE_SPEED * rl.get_frame_time();
        }
        if rl.is_key_down(self.down_key) && self.y + PADDLE_HEIGHT < GAME_HEIGHT {
            self.y += PADDLE_SPEED * rl.get_frame_time();
        }
    }

    /** Draw a paddle to the screen */
    pub fn draw(&mut self, dh: &mut RaylibDrawHandle) {
        dh.draw_rectangle(self.x as i32, self.y as i32, PADDLE_WIDTH as i32, PADDLE_HEIGHT as i32, OBJECT_COLOR);
    }
}

/** Represents the ball */
struct Ball {
    x: f32,
    y: f32,

    dir_x: f32,
    dir_y: f32,
}

impl Ball {
    /** Construct a ball */
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        return Self {
            x: GAME_WIDTH / 2.0,
            y: GAME_HEIGHT / 2.0,

            dir_x: if rng.gen_range(0.0..1.0) == 0.0 {-1.0} else {1.0},
            dir_y: if rng.gen_range(0.0..1.0) == 0.0 {-1.0} else {1.0},
        };
    }

    /** Resets the ball */
    pub fn reset(&mut self, dir: f32) {
        let mut rng = rand::thread_rng();

        self.x = GAME_WIDTH / 2.0;
        self.y = GAME_HEIGHT / 2.0;

        self.dir_x = dir;
        self.dir_y = if rng.gen_range(0.0..1.0) == 0.0 {-1.0} else {1.0};
    }

    /** Update the ball */
    pub fn update(&mut self, paddle_1: &Paddle, paddle_2: &Paddle, rl: &mut RaylibHandle) {
        self.x += self.dir_x * BALL_SPEED * rl.get_frame_time();
        self.y += self.dir_y * BALL_SPEED * rl.get_frame_time();

        let bal_collider = Rectangle::new(self.x, self.y, BALL_WIDTH, BALL_HEIGHT);

        if Rectangle::new(paddle_1.x, paddle_1.y, PADDLE_WIDTH, PADDLE_HEIGHT).check_collision_recs(&bal_collider) && self.dir_x == -1.0 {
            self.dir_x = -self.dir_x;
        }

        if Rectangle::new(paddle_2.x, paddle_2.y, PADDLE_WIDTH, PADDLE_HEIGHT).check_collision_recs(&bal_collider) && self.dir_x == 1.0 {
            self.dir_x = -self.dir_x;
        }

        if (self.y < 0.0 && self.dir_y == -1.0) || (self.y + BALL_HEIGHT > GAME_HEIGHT && self.dir_y == 1.0)  {
            self.dir_y = -self.dir_y;
        }
    }

    /** Draw the ball */
    pub fn draw(&mut self, dh: &mut RaylibDrawHandle) {
        dh.draw_rectangle(self.x as i32, self.y as i32, BALL_WIDTH as i32, BALL_HEIGHT as i32, OBJECT_COLOR);
    }
}

/** Initialize game context */
fn init() -> (RaylibHandle, RaylibThread) {
    return raylib::init()
    .title("Pong Raylib")
    .size(GAME_WIDTH as i32, GAME_HEIGHT as i32)
    .build();
}

/** Check for goals */
fn check_for_goal(mut ball: &mut Ball, paddle_1: &mut Paddle, paddle_2: &mut Paddle) {
    if ball.x > GAME_WIDTH {
        // Gloal player 1
        paddle_1.points += 1;
        Ball::reset(&mut ball, -1.0);
    }

    if ball.x < 0.0 {
        // Goal player 2
        paddle_2.points += 1;
        Ball::reset(&mut ball, 1.0);
    }
}

/** Create objects */
fn create_objects() -> (Paddle, Paddle, Ball) {
    let paddle_1 = Paddle::new(PADDLE_OFFSET, consts::KeyboardKey::KEY_W, consts::KeyboardKey::KEY_S);
    let paddle_2 = Paddle::new(GAME_WIDTH - (PADDLE_OFFSET * 1.5), consts::KeyboardKey::KEY_UP, consts::KeyboardKey::KEY_DOWN);
    let ball = Ball::new();
    return (paddle_1, paddle_2, ball);
}

fn main() {
    // Initialize game
    // Initialize raylib window and context
    let (mut rl, thread) = init();
    // Create paddles and ball
    let (mut paddle_1, mut paddle_2, mut ball) = create_objects();

    // Gameloop
    while !rl.window_should_close() {
        // Update loop

        // Update paddles
        Paddle::update(&mut paddle_1, &mut rl);
        Paddle::update(&mut paddle_2, &mut rl);
        Ball::update(&mut ball, &paddle_1, &paddle_2 ,&mut rl);

        check_for_goal(&mut ball, &mut paddle_1, &mut paddle_2);

        // Draw loop
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text(&format!("{} - {}", paddle_1.points, paddle_2.points), (GAME_WIDTH/2.0) as i32 - 120, 0, 96, OBJECT_COLOR);

        // Draw paddles
        Paddle::draw(&mut paddle_1, &mut d);
        Paddle::draw(&mut paddle_2, &mut d);
        Ball::draw(&mut ball, &mut d);
    }
}
