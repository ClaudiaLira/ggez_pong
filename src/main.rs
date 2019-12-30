use ggez::*;

struct State {}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>{
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>{
        Ok(())
    }
}
fn main() {
    let state = &mut State {};
    
}
