use ggez::graphics::{DrawMode, DrawParam};
use ggez::nalgebra::Point2;
use ggez::{conf, event, graphics, timer, Context, ContextBuilder, GameResult};
use rand::{thread_rng, Rng};
use std::{env, path};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

const BALL_SPEED: f32 = 5.0;

const PLAYER_WIDTH: u32 = 30;
const PLAYER_HEIGHT: u32 = 50;

struct Ball {
    x: f32,
    y: f32,
    speed: f32,
    dir_x: f32,
    dir_y: f32,
}

impl Ball {
    fn new() -> Ball {
        let mut rng = thread_rng();
        let dir_x = rng.gen_range(-1.0, 1.0);
        let dir_y = rng.gen_range(-1.0, 1.0);
        Ball {
            x: WINDOW_WIDTH as f32 / 2.0,
            y: WINDOW_HEIGHT as f32 / 2.0,
            speed: BALL_SPEED,
            dir_x: dir_x,
            dir_y: dir_y,
        }
    }

    pub fn update(&mut self) {
        self.x += self.dir_x * self.speed;
        self.y += self.dir_y * self.speed;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let point = Point2::new(self.x, self.y);
        let b1 =
            graphics::Mesh::new_circle(ctx, DrawMode::fill(), point, 10.0, 1.0, graphics::WHITE)?;
        graphics::draw(ctx, &b1, DrawParam::default())?;
        Ok(())
    }
}
enum Side {
    Left,
    Right,
}
struct Player {
    x: f32,
    y: f32,
    side: Side,
}

impl Player {
    fn new(side: Side) -> Player {
        let x = match side {
            Side::Left => 20.0,
            Side::Right => WINDOW_WIDTH as f32 - 50.0,
        };
        Player {
            x: x,
            y: (WINDOW_HEIGHT as f32 - PLAYER_HEIGHT as f32) / 2.0,
            side: side,
        }
    }

    // pub fn update(&mut self) {
    //     self.x += self.vel_x;
    //     self.y += self.vel_y;
    // }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let point = Point2::new(self.x, self.y);
        let rect = graphics::Rect::new(self.x, self.y, PLAYER_WIDTH as f32, PLAYER_HEIGHT as f32);
        let b1 = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, graphics::WHITE)?;
        graphics::draw(ctx, &b1, DrawParam::default())?;
        Ok(())
    }
}
struct MainState {
    player_1: Player,
    player_2: Player,
    ball: Ball,
    score: (u32, u32),
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            ball: Ball::new(),
            player_1: Player::new(Side::Left),
            player_2: Player::new(Side::Right),
            score: (0, 0),
        };

        Ok(s)
    }

    pub fn check_collision(&mut self) {
        if (self.ball.x < 0.0) {
            self.score.1 += 1;
            self.restart_ball(Side::Right);
        }
        if (self.ball.x > WINDOW_WIDTH as f32) {
            self.score.0 += 1;
            self.restart_ball(Side::Left);
        }
        if (self.ball.y < 0.0 || self.ball.y > WINDOW_HEIGHT as f32) {
            self.ball.dir_y = -self.ball.dir_y;
        }
    }

    fn restart_ball(&mut self, side: Side) {
        let mut rng = thread_rng();
        self.ball.x = WINDOW_WIDTH as f32 / 2.0;
        self.ball.y = WINDOW_HEIGHT as f32 / 2.0;
        self.ball.dir_x = match side {
            Side::Left => -1.0,
            Side::Right => 1.0
        };
        self.ball.dir_y = rng.gen_range(-1.0, 1.0);
    }
}
impl event::EventHandler for MainState {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        self.ball.draw(ctx)?;
        self.player_1.draw(ctx)?;
        self.player_2.draw(ctx)?;
        graphics::present(ctx);
        Ok(())
    }
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.ball.update();
        self.check_collision();
        Ok(())
    }
}
fn main() -> GameResult {
    let mut cb = ggez::ContextBuilder::new("drawing", "ggez")
        .window_setup(conf::WindowSetup::default().title("Pong"))
        .window_mode(
            conf::WindowMode::default().dimensions(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
        );

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources/");
        cb = cb.add_resource_path(path);
    } else {
        println!("Not building from cargo?  Ok.");
    }

    let (ctx, events_loop) = &mut cb.build()?;

    let state = &mut MainState::new().unwrap();
    event::run(ctx, events_loop, state)
}
