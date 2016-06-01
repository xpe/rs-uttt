/// Random functions.

use constants::*;
use data::*;
use rand::{Rand, Rng, ThreadRng, thread_rng};
use std::collections::LinkedList;

// -- games --------------------------------------------------------------------

/// Plays a game randomly from start to finish. Returns a linked list of
/// games (where each is a 'step').
pub fn random_game() -> LinkedList<Game> {
    let rng: &mut ThreadRng = &mut thread_rng();
    let mut games: LinkedList<Game> = LinkedList::new();
    let mut game: Game = EMPTY_GAME;
    loop {
        games.push_back(game);
        if Game::is_complete(game) {
            println!("\nGame is over.");
            break;
        } else {
            game = game.play(random_valid_play(game, rng)).unwrap();
        }
        let x = rng.gen::<u8>();
        if x < 10 {
            break;
        }
    }
    games
}

// -- game ---------------------------------------------------------------------

// -- board --------------------------------------------------------------------

// -- rows ---------------------------------------------------------------------

// -- row ----------------------------------------------------------------------

impl Rand for Row {
    /// Returns a random board column index.
    fn rand<R: Rng>(rng: &mut R) -> Self {
        const CHOICES: [Row; 27] = [
            Row::EEE, Row::EEO, Row::EEX,
            Row::EOE, Row::EOO, Row::EOX,
            Row::EXE, Row::EXO, Row::EXX,
            Row::OEE, Row::OEO, Row::OEX,
            Row::OOE, Row::OOO, Row::OOX,
            Row::OXE, Row::OXO, Row::OXX,
            Row::XEE, Row::XEO, Row::XEX,
            Row::XOE, Row::XOO, Row::XOX,
            Row::XXE, Row::XXO, Row::XXX,
        ];
        rng.choose(&CHOICES).unwrap().clone()
    }
}

// -- board play ---------------------------------------------------------------

/// Returns a random play for a given game.
pub fn random_valid_play(game: Game, rng: &mut ThreadRng) -> Play {
    Play {
        loc: random_valid_loc(game, rng),
        player: game.next_player().unwrap(),
    }
}

// -- sub-board play -----------------------------------------------------------

// -- board location -----------------------------------------------------------

/// Returns a random valid location for a play in a game.
#[allow(unused_variables)]
pub fn random_valid_loc(game: Game, rng: &mut ThreadRng) -> Loc {
    // TODO: implement this correctly
    let ri = rng.gen::<RI>();
    let ci = rng.gen::<CI>();
    Loc::new(ri, ci)
}

// -- sub-board location -------------------------------------------------------

// -- slots --------------------------------------------------------------------

// -- slot --------------------------------------------------------------------

// -- board indexes ------------------------------------------------------------

impl Rand for RI {
    /// Returns a random board row index.
    fn rand<R: Rng>(rng: &mut R) -> Self {
        const CHOICES: [RI; 9] = [
            RI::R0,
            RI::R1,
            RI::R2,
            RI::R3,
            RI::R4,
            RI::R5,
            RI::R6,
            RI::R7,
            RI::R8,
        ];
        rng.choose(&CHOICES).unwrap().clone()
    }
}

impl Rand for CI {
    /// Returns a random board column index.
    fn rand<R: Rng>(rng: &mut R) -> Self {
        const CHOICES: [CI; 9] = [
            CI::C0,
            CI::C1,
            CI::C2,
            CI::C3,
            CI::C4,
            CI::C5,
            CI::C6,
            CI::C7,
            CI::C8,
        ];
        rng.choose(&CHOICES).unwrap().clone()
    }
}

// -- sub-board indexes --------------------------------------------------------


impl Rand for SRI {
    /// Returns a random sub-board row index.
    fn rand<R: Rng>(rng: &mut R) -> Self {
        const CHOICES: [SRI; 3] = [SRI::R0, SRI::R1, SRI::R2];
        rng.choose(&CHOICES).unwrap().clone()
    }
}

impl Rand for SCI {
    /// Returns a random sub-board column index.
    fn rand<R: Rng>(rng: &mut R) -> Self {
        const CHOICES: [SCI; 3] = [SCI::C0, SCI::C1, SCI::C2];
        rng.choose(&CHOICES).unwrap().clone()
    }
}

// -- player -------------------------------------------------------------------

impl Rand for Player {
    /// Returns a random board column index.
    fn rand<R: Rng>(rng: &mut R) -> Self {
        const CHOICES: [Player; 2] = [Player::X, Player::O];
        rng.choose(&CHOICES).unwrap().clone()
    }
}