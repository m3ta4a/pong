use dynamo_lib::start;

mod ball;
mod input;
mod player;
mod pong_game;
use pong_game::PongGame;

fn main() {
    let pong_game = PongGame::new();
    start("Pong", Box::new(pong_game));
}
