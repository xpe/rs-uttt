extern crate rand;
extern crate uttt;

use rand::{Rng, XorShiftRng, SeedableRng};
use uttt::data::*;
use uttt::random::*;
use uttt::solver::*;
use uttt::utility::{h, p, pln};

const VERBOSE: bool = true;

// -- main ---------------------------------------------------------------------

fn main() {
    let mut rng = make_rng();
    let stack = SSD_CPU_Stack::new();
    run_random_games(0, &mut rng);
    run_random_game(0, &mut rng);
    run_solve(0, &stack, &mut rng, 5, 7);
    run_backwards_solve(0, &stack, &mut rng, 81, 10);
    run_full_backwards_solve(0, &stack, &mut rng);
    run_ongoing_backwards_solve(true, &stack, &mut rng, 7, 9);
}

// -- main sub-function(s) -----------------------------------------------------

fn make_rng() -> XorShiftRng {
    let seed = random_seed();
    // let seed: [u32; 4] = [1950144991, 3815769152, 584337888, 1474954538];
    SeedableRng::from_seed(seed)
}

fn run_random_games<R: Rng>(trials: u16, rng: &mut R) {
    if trials > 0 {
        h(0, "random_games()");
        let mut x_wins = 0;
        let mut o_wins = 0;
        let mut ties = 0;
        let mut games_len = 0;
        for i in 0 .. trials {
            let games = random_games(rng);
            let game_len = games.len();
            let game = games.iter().last().expect("E9901");
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
        println!("average game length: {}",
                 (games_len as f64) / (trials as f64));
    }
}

fn run_random_game<R: Rng>(trials: u16, rng: &mut R) {
    if trials > 0 {
        h(0, "random_game()");
        for i in 0 .. trials {
            h(1, &format!("Game #{}", i));
            let game = random_game(rng);
            p(&game);
        }
    }
}

fn run_solve<R: Rng>(trials: u16, stack: &Stack, rng: &mut R,
                     back: Count, depth: Count) {
    if trials > 0 && back > 0 {
        h(0, "Solve N-4");
        for i in 0 .. trials {
            if VERBOSE { h(1, &format!("Trial #{}", i)); }
            let games = random_games(rng);
            let mut games_iter = games.iter();
            // Get the last move in the sequence of games.
            let game_n = games_iter.next_back().expect("E9902");
            if VERBOSE { h(2, "Game N"); }
            if VERBOSE { pln(game_n); }

            // TODO: It would be nice to extract out this chunk of code;
            // however, the types have flummoxed me so far.
            //
            // Back up `back - 1` times.
            for _ in 0 .. (back - 1) {
                games_iter.next_back().expect("E9903");
            }
            // Back up one more time.
            let game = games_iter.next_back().expect("E9904");

            let label = format!("Game N-{}", back);
            if VERBOSE { h(2, &label); }
            if VERBOSE { pln(game); }
            let solutions = solve(stack, &game, depth);
            if VERBOSE { p_solutions(&label, depth, &solutions); }
        }
    }
}

fn run_backwards_solve<R: Rng>(trials: u16, stack: &Stack, rng: &mut R,
                               depth: Count, n: Count) {
    if trials > 0 && n > 0 {
        h(0, "Solving Back to Front");
        for trial in 1 .. trials + 1 {
            if VERBOSE { h(1, &format!("Trial #{}", trial)); }
            let games = random_games(rng);
            let mut games_iter = games.iter();
            let game_n = games_iter.next_back().expect("E9905");
            if VERBOSE { h(2, "Game N"); }
            if VERBOSE { pln(game_n); }
            for i in 1 .. (n + 1) {
                let label = &format!("N-{}", i);
                if VERBOSE { h(2, label) }
                let game = games_iter.next_back().expect("E9906");
                if VERBOSE { pln(game); }
                let solutions = solve(stack, &game, depth + i);
                if VERBOSE { p_solutions(label, depth + i, &solutions); }
            }
        }
    }
}

fn run_full_backwards_solve<R: Rng>(trials: u16, stack: &Stack, rng: &mut R) {
    let depth = 81;
    if trials > 0 {
        h(0, "Fully Solving Back to Front");
        for trial in 1 .. (trials + 1) {
            if VERBOSE { h(1, &format!("Trial #{}", trial)); }
            let games = random_games(rng);
            let mut i = 0;
            for game in games.iter().rev() {
                let label = &format!("Game N-{}", i);
                if VERBOSE { h(2, label) }
                if VERBOSE { pln(game); }
                let solutions = solve(stack, &game, depth);
                if VERBOSE { p_cache(stack); }
                if VERBOSE { p_solutions(label, depth, &solutions); }
                i = i + 1;
            }
        }
    }
}

fn run_ongoing_backwards_solve<R: Rng>
    (active: bool, stack: &Stack, rng: &mut R, depth: Count, n: Count) {
    if active {
        let mut trial: usize = 0;
        h(0, "Backwards Solve (Ongoing)");
        loop {
            trial += 1;
            if VERBOSE { h(1, &format!("Trial #{}", trial)); }
            let games = random_games(rng);
            let mut games_iter = games.iter();
            let game_n = games_iter.next_back().expect("E9907");
            if VERBOSE { h(2, &format!("Trial #{} Game N", trial)); }
            if VERBOSE { pln(game_n); }
            for i in 1 .. (n + 1) {
                let label = &format!("Trial #{} Game N-{}", trial, i);
                if VERBOSE { h(2, label); }
                let game = games_iter.next_back().expect("E9908");
                if VERBOSE { pln(game); }
                let solutions = solve(stack, &game, depth);
                if VERBOSE { p_solutions(label, depth, &solutions); }
            }
        }
    }
}

// -- solve function(s) --------------------------------------------------------

fn solve(stack: &Stack, game: &Game, depth: Count) -> Vec<Solution> {
    stack.get_and_put(game, depth, stack)
}

// -- str function(s) ----------------------------------------------------------

fn result_str(op: Option<Player>) -> &'static str {
    match op {
        Some(Player::X) => "X wins",
        Some(Player::O) => "O wins",
        None => "  tie ",
    }
}

fn p_cache(stack: &Stack) {
    let device = stack.devices.get(0).expect("E9909");
    println!("SSD RAM cache size : {}", SSD::cache_len(&device));
}
