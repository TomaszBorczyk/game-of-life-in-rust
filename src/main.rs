mod game;

use crate::game::{
    game_loop
};


fn main() {
    // TODO: add board size and wait time between iterations as input arguments
    game_loop(20, 20);
}
