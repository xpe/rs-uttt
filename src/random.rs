/// Random functions.

use constants::{EMPTY_GAME};
use data::{Game, Play, Loc, RI, CI};
use rand::{Rng, ThreadRng};
use rand;
use std::collections::LinkedList;

/// Plays a game randomly from start to finish. Returns a linked list of
/// games (where each is a 'step').
pub fn random_game() -> LinkedList<Game> {
    let rng: &mut ThreadRng = &mut rand::thread_rng();
    let mut games: LinkedList<Game> = LinkedList::new();
    let mut game: Game = EMPTY_GAME;
    loop {
        games.push_back(game);
        if Game::is_complete(game) {
            println!("\nGame is over.");
            break;
        } else {
            game = game.play(random_play(game, rng)).unwrap();
        }
        let x = rng.gen::<u8>();
        if x < 10 {
            break;
        }
    }
    games
}

pub fn random_play(game: Game, rng: &mut ThreadRng) -> Play {
    Play {
        loc: random_loc(game, rng),
        player: game.next_player().unwrap(),
    }
}

// TODO: implement this correctly
#[allow(unused_variables)]
pub fn random_loc(game: Game, rng: &mut ThreadRng) -> Loc {
    let ri = rng.gen::<RI>();
    let ci = rng.gen::<CI>();
    Loc::new(ri, ci)
}

impl rand::Rand for RI {
    fn rand<R: Rng>(rng: &mut R) -> RI {
        let x = rng.gen::<u16>();
        match x % 9 {
            0 => RI::R0,
            1 => RI::R1,
            2 => RI::R2,
            3 => RI::R3,
            4 => RI::R4,
            5 => RI::R5,
            6 => RI::R6,
            7 => RI::R7,
            8 => RI::R8,
            _ => panic!("internal error"),
        }
    }
}

impl rand::Rand for CI {
    fn rand<R: Rng>(rng: &mut R) -> CI {
        let x = rng.gen::<u16>();
        match x % 9 {
            0 => CI::C0,
            1 => CI::C1,
            2 => CI::C2,
            3 => CI::C3,
            4 => CI::C4,
            5 => CI::C5,
            6 => CI::C6,
            7 => CI::C7,
            8 => CI::C8,
            _ => panic!("internal error"),
        }
    }
}
