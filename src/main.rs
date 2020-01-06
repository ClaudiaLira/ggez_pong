use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{DrawMode, DrawParam, Text, TextFragment};
use ggez::input::keyboard;
use ggez::nalgebra::Point2;
use ggez::{conf, event, graphics, Context, GameResult};
use rand::{thread_rng, Rng};


const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

const BALL_SPEED: f32 = 5.0;

const PLAYER_WIDTH: f32 = 30.0;
const PLAYER_HEIGHT: f32 = 50.0;

const DISTANCE_PLAYER_GOAL: f32 = 20.0;

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
            x: WINDOW_WIDTH / 2.0,
            y: WINDOW_HEIGHT / 2.0,
            speed: BALL_SPEED,
            dir_x: dir_x,
            dir_y: dir_y,
        }
    }

    fn update(&mut self) {
        self.x += self.dir_x * self.speed;
        self.y += self.dir_y * self.speed;
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
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

enum Direction {
    Up,
    Down,
}

struct Player {
    x: f32,
    y: f32,
    speed: f32,
}

impl Player {
    fn new(side: Side) -> Player {
        let x = match side {
            Side::Left => DISTANCE_PLAYER_GOAL,
            Side::Right => WINDOW_WIDTH - (PLAYER_WIDTH + DISTANCE_PLAYER_GOAL),
        };
        Player {
            x: x,
            y: (WINDOW_HEIGHT - PLAYER_HEIGHT) / 2.0,
            speed: 10.0,
        }
    }

    fn move_player(&mut self, dir: Direction) {
        self.y += match dir {
            Direction::Up => -self.speed,
            Direction::Down => self.speed,
        };

        if self.y < 0.0 {
            self.y = 0.0;
        }

        if self.y + PLAYER_HEIGHT > WINDOW_HEIGHT {
            self.y = WINDOW_HEIGHT - PLAYER_HEIGHT;
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let rect = graphics::Rect::new(self.x, self.y, PLAYER_WIDTH, PLAYER_HEIGHT);
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
        if self.ball.x < 0.0 {
            self.score.1 += 1;
            self.restart_ball(Side::Right);
        }
        if self.ball.x > WINDOW_WIDTH {
            self.score.0 += 1;
            self.restart_ball(Side::Left);
        }
        if self.ball.y < 0.0 || self.ball.y > WINDOW_HEIGHT {
            self.ball.dir_y = -self.ball.dir_y;
        }

        if self.ball.x <= self.player_1.x + PLAYER_WIDTH && self.ball.x > PLAYER_WIDTH {
            if self.ball.y > self.player_1.y && self.ball.y < self.player_1.y + PLAYER_HEIGHT {
                self.ball.dir_x = -self.ball.dir_x;
            }
        }
        if self.ball.x >= self.player_2.x && self.ball.x < self.player_2.x + PLAYER_WIDTH {
            if self.ball.y > self.player_2.y && self.ball.y < self.player_2.y + PLAYER_HEIGHT {
                self.ball.dir_x = -self.ball.dir_x;
            }
        }
    }

    fn restart_ball(&mut self, side: Side) {
        let mut rng = thread_rng();
        self.ball.x = WINDOW_WIDTH / 2.0;
        self.ball.y = WINDOW_HEIGHT / 2.0;
        self.ball.dir_x = match side {
            Side::Left => -1.0,
            Side::Right => 1.0,
        };
        self.ball.dir_y = rng.gen_range(-1.0, 1.0);
    }
}

impl EventHandler for MainState {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let mut text = Text::new("Score ");
        text.add(TextFragment::new(self.score.0.to_string()));
        text.add(TextFragment::new(" : "));
        text.add(TextFragment::new(self.score.1.to_string()));
        graphics::draw(
            ctx,
            &text,
            (Point2::new(50.0, 10.0), graphics::WHITE),
        )?;
        self.ball.draw(ctx)?;
        self.player_1.draw(ctx)?;
        self.player_2.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.ball.update();
        self.check_collision();

        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.player_1.move_player(Direction::Up);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.player_1.move_player(Direction::Down);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.player_2.move_player(Direction::Up);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.player_2.move_player(Direction::Down);
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("pong", "Claudia Lira")
        .window_setup(conf::WindowSetup::default().title("Pong"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));

    let (ctx, events_loop) = &mut cb.build()?;

    let state = &mut MainState::new().unwrap();
    event::run(ctx, events_loop, state)
}
