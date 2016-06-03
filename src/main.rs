extern crate rand;
extern crate uttt;

use rand::{Rng, XorShiftRng, SeedableRng};
use uttt::data::*;
use uttt::random::*;
use uttt::utility::{p, h};

fn main() {
    let seed = random_seed();
    let mut rng: XorShiftRng = SeedableRng::from_seed(seed);
    // run_random_game(&mut rng, 5);
    // run_random_games(&mut rng, 100);
    run_solve_1(&mut rng);
}

fn run_solve_1<R: Rng>(rng: &mut R) {
    h(0, "Solve 1");
    let games = random_games(rng);
    let mut games_iter = games.iter();
    h(1, "Game at Turn N");
    let game_n = games_iter.next_back().unwrap();
    p_game(game_n);
    h(1, "Game at Turn N - 1");
    let game_n_1 = games_iter.next_back().unwrap();
    p_game(game_n_1);
    // game.solve_1();
}

/// Returns a random seed, intended for XorShiftRng.
fn random_seed() -> [u32; 4] {
    let mut rng = rand::thread_rng();
    let seed = [
        rng.gen::<u32>(),
        rng.gen::<u32>(),
        rng.gen::<u32>(),
        rng.gen::<u32>(),
    ];
    println!("Using random number seed {:?}\n", seed);
    seed
}

#[allow(dead_code)]
fn run_random_game<R: Rng>(rng: &mut R, trials: u16) {
    h(0, "random_game()");
    for i in 0 .. trials {
        h(1, &format!("Game #{}", i));
        let game = random_game(rng);
        p_game(&game);
    }
}

#[allow(dead_code)]
fn p_game(game: &Game) {
    p(game);
    println!("");
}

#[allow(dead_code)]
fn run_random_games<R: Rng>(rng: &mut R, trials: u16) {
    h(0, "random_games()");
    let mut x_wins = 0;
    let mut o_wins = 0;
    let mut ties = 0;
    let mut games_len = 0;
    for i in 0 .. trials {
        let games = random_games(rng);
        let game_len = games.len();
        let game = games.iter().last().unwrap();
        let winner = game.winner();
        println!("Game #{:4}: {} in {}", i, result_str(winner), game_len);
        games_len += game_len;
        match winner {
            None => ties += 1,
            Some(Player::X) => x_wins += 1,
            Some(Player::O) => o_wins += 1,
        }
    }
    println!("");
    println!("X wins: {:4}", x_wins);
    println!("O wins: {:4}", o_wins);
    println!("  ties: {:4}", ties);
    println!("");
    println!("average game length: {}", (games_len as f64) / (trials as f64));
}

#[allow(dead_code)]
fn result_str(op: Option<Player>) -> &'static str {
    match op {
        Some(Player::X) => "X wins",
        Some(Player::O) => "O wins",
        None => "  tie ",
    }
}

#[allow(dead_code)]
fn print_winner(winner: Option<Player>) {
    let p = match winner {
        Some(player) => format!("{:?}", player),
        None => format!("-"),
    };
    println!("                 winner : {}", p);
}
