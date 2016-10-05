/// Random functions.

use constants::*;
use data::*;
use rand::{Rand, Rng, thread_rng};
use std::collections::LinkedList;

// -- games --------------------------------------------------------------------

/// Plays a game randomly from start to finish. Returns a doubly linked list of
/// games (where each is a 'step').
pub fn random_games<R: Rng>(rng: &mut R) -> LinkedList<Game> {
    let mut games: LinkedList<Game> = LinkedList::new();
    let mut game: Game = EMPTY_GAME;
    loop {
        games.push_back(game);
        if Game::is_over(game) {
            break;
        } else {
            match random_valid_play(&game, rng) {
                None => break,
                Some(play) => { game.play(play); },

            }
        }
    }
    games
}

// -- game ---------------------------------------------------------------------

/// Returns a random game by making between 1 and 81 moves. It almost certainly
/// does not sample uniformly from possible random games.
///
/// Note: in order to uniformly sample across the game space, it would be
/// necessary to pick randomly from `random_games`, which returns a game
/// progression from start to completion. Such an approach would be slower. The
/// approach here is much faster than that, although it biases the results
/// towards completed games (which is not necessarily undesirable).
pub fn random_game<R: Rng>(rng: &mut R) -> Game {
    let mut i: u8 = 0;
    let mut game = EMPTY_GAME;
    let n = rng.gen_range(1, 82);
    while i < n {
        match random_valid_play(&game, rng) {
            None => break,
            Some(play) => {
                game.play(play);
                i += 1;
            },
        }
    }
    game
}

impl Rand for Game {
    // Returns a random game.
    fn rand<R: Rng>(rng: &mut R) -> Self {
        random_game(rng)
    }
}

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
        rng.choose(&CHOICES).expect("E2601").clone()
    }
}

// -- board play ---------------------------------------------------------------

/// Returns a random play for a given game.
pub fn random_valid_play<R: Rng>(game: &Game, rng: &mut R) -> Option<Play> {
    match game.next_player() {
        None => None,
        Some(_) => {
            let valid_plays: &[Play] = &game.valid_plays()[..];
            let play: &Play = rng.choose(valid_plays).expect("E2602");
            Some(play.clone())
        },
    }
}

// -- sub-board play -----------------------------------------------------------

// -- board location -----------------------------------------------------------

/// Returns a random location. By design, this function does not look for valid
/// locations efficiently.
pub fn random_loc<R: Rng>(rng: &mut R) -> Loc {
    Loc::new(rng.gen::<RI>(), rng.gen::<CI>())
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
        rng.choose(&CHOICES).expect("E2603").clone()
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
        rng.choose(&CHOICES).expect("E2604").clone()
    }
}

impl Rand for BI {
    /// Returns a random board index.
    fn rand<R: Rng>(rng: &mut R) -> Self {
        const CHOICES: [BI; 9] = [
            BI::I0,
            BI::I1,
            BI::I2,
            BI::I3,
            BI::I4,
            BI::I5,
            BI::I6,
            BI::I7,
            BI::I8,
        ];
        rng.choose(&CHOICES).expect("E2605").clone()
    }
}

// -- sub-board indexes --------------------------------------------------------

impl Rand for SRI {
    /// Returns a random sub-board row index.
    fn rand<R: Rng>(rng: &mut R) -> Self {
        const CHOICES: [SRI; 3] = [SRI::R0, SRI::R1, SRI::R2];
        rng.choose(&CHOICES).expect("E2606").clone()
    }
}

impl Rand for SCI {
    /// Returns a random sub-board column index.
    fn rand<R: Rng>(rng: &mut R) -> Self {
        const CHOICES: [SCI; 3] = [SCI::C0, SCI::C1, SCI::C2];
        rng.choose(&CHOICES).expect("E2607").clone()
    }
}

impl Rand for SBI {
    /// Returns a random sub-board index.
    fn rand<R: Rng>(rng: &mut R) -> Self {
        const CHOICES: [SBI; 9] = [
            SBI::I0,
            SBI::I1,
            SBI::I2,
            SBI::I3,
            SBI::I4,
            SBI::I5,
            SBI::I6,
            SBI::I7,
            SBI::I8,
        ];
        rng.choose(&CHOICES).expect("E2608").clone()
    }
}

// -- player -------------------------------------------------------------------

impl Rand for Player {
    /// Returns a random board column index.
    fn rand<R: Rng>(rng: &mut R) -> Self {
        const CHOICES: [Player; 2] = [Player::X, Player::O];
        rng.choose(&CHOICES).expect("E2609").clone()
    }
}

// -- [u32; 4] -----------------------------------------------------------------

/// Returns a random seed, intended for XorShiftRng.
pub fn random_seed() -> [u32; 4] {
    let mut rng = thread_rng();
    let seed = [
        rng.gen::<u32>(),
        rng.gen::<u32>(),
        rng.gen::<u32>(),
        rng.gen::<u32>(),
    ];
    println!("Using random number seed {:?}\n", seed);
    seed
}
