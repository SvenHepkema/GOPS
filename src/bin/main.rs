use gops::aiplayers::*;
use gops::arena::{AIEnum, Arena};
use gops::game::Game;
use gops::humanplayer::HumanPlayer;

fn main() {
    // Should probably not be a struct, but an enum
    /*
    let player_a = Box::new(HumanPlayer{value: 1});
    let player_b = Box::new(EqualBuyer{value: 1});
    //let player_b = Box::new(RandomAIPlayer{value: 1});
    //let player_b = Box::new(RandomAIPlayer{value: 1});

    let mut game = Game::new(player_a, player_b, true);
    game.play();
    */

    let mut arena = Arena::new(100, AIEnum::SimpleMC, AIEnum::EqualBuyer);
    arena.start_tournament();
    arena.show_results();
}
