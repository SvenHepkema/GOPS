use gops::humanplayer::HumanPlayer;
use gops::aiplayers::*;
use gops::game::Game;

fn main() {
    // Should probably not be a struct, but an enum
    let player_a = Box::new(HumanPlayer{value: 1});
    let player_b = Box::new(EqualBuyer{value: 1});
    //let player_b = Box::new(RandomAIPlayer{value: 1});

    let mut game = Game::new(player_a, player_b, true);
    println!("Score: {0}", game.play());
}
