
use ggez::{conf, event, graphics, timer, Context, ContextBuilder, GameResult};
use ggez::graphics::{DrawMode, DrawParam};
use ggez::nalgebra::Point2;
use std::{env, path};
use rand::{thread_rng, Rng};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

struct Ball {
    x: f32,
    y: f32,
    vel_x: f32,
    vel_y: f32,
}

impl Ball {
    fn new() -> Ball {
        let mut rng = thread_rng();
        let vel_x = rng.gen_range(0.0, 10.0);
        let vel_y = rng.gen_range(0.0, 10.0);
        Ball {
            x: WINDOW_WIDTH as f32 / 2.0,
            y: WINDOW_HEIGHT as f32 / 2.0,
            vel_x: vel_x,
            vel_y: vel_y,
        }
    }

    pub fn update(&mut self){
        self.x += self.vel_x;
        self.y += self.vel_y;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let point = Point2::new(self.x, self.y);
        let b1 = graphics::Mesh::new_circle(ctx, DrawMode::fill(), point, 10.0, 1.0, graphics::WHITE)?;
        graphics::draw(ctx, &b1, DrawParam::default())?;
        Ok(())
    }
}
struct Player {
    x: f32,
    y: f32,
}
struct MainState {
    // player_1: Player,
    // player_2: Player,
    ball: Ball,
}

impl MainState {
    fn new() -> GameResult<MainState>{
        let s = MainState {
            ball: Ball::new()
        };

        Ok(s)
    }

    pub fn check_collision(&mut self) {
        if(self.ball.x < 0.0 || self.ball.x > WINDOW_WIDTH as f32){
            self.ball.vel_x = -self.ball.vel_x;
        }
        if(self.ball.y < 0.0 || self.ball.y > WINDOW_HEIGHT as f32){
            self.ball.vel_y = -self.ball.vel_y;
        }
    }
}
impl event::EventHandler for MainState {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        self.ball.draw(ctx)?;
        graphics::present(ctx);
        Ok(())
    }
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.ball.update();
        self.check_collision();
        Ok(())
    }
}
fn main() -> GameResult{
    let mut cb = ggez::ContextBuilder::new("drawing", "ggez")
        .window_setup(conf::WindowSetup::default().title("Pong"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32));

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
