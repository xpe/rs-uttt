extern crate rand;
extern crate uttt;

use rand::{Rng, XorShiftRng, SeedableRng};
use uttt::data::*;
use uttt::random::*;
use uttt::show::*;
use uttt::solver::*;
use uttt::utility::{h, p, pln};

const VERBOSE: bool = true;

// -- main ---------------------------------------------------------------------

fn main() {
    // let seed: [u32; 4] = [1979915768, 300767643, 3885545663, 2473070596];
    // let seed: [u32; 4] = [3387584637, 3821802413, 3724964352, 3288162107];
    let seed: [u32; 4] = [1456198685, 762656086, 844876651, 1745969790];
    // let seed = random_seed();
    let mut rng: XorShiftRng = SeedableRng::from_seed(seed);
    run_random_games(&mut rng, 0);
    run_random_game(&mut rng, 0);
    run_solve_for(&mut rng, 6, 8, 0);
    run_backwards_solver(&mut rng, 20, 15);
}

// -- main sub-functions -------------------------------------------------------

fn run_random_games<R: Rng>(rng: &mut R, trials: u16) {
    if trials > 0 {
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
}

fn run_random_game<R: Rng>(rng: &mut R, trials: u16) {
    if trials > 0 {
        h(0, "random_game()");
        for i in 0 .. trials {
            h(1, &format!("Game #{}", i));
            let game = random_game(rng);
            p(&game);
        }
    }
}

fn run_solve_for<R: Rng>(rng: &mut R, k: Count, depth: Count, trials: u16) {
    if trials > 0 && k > 0 {
        let cache = &mut new_cache(1000);
        h(0, "Solve N-4");
        for i in 0 .. trials {
            if VERBOSE { h(1, &format!("Trial #{}", i)); }

            let games = random_games(rng);
            let mut games_iter = games.iter();
            let game_n = games_iter.next_back().unwrap();
            if VERBOSE { h(1, "Game N"); }
            if VERBOSE { pln(game_n); }

            for _ in 0 .. (k - 1) {
                games_iter.next_back().unwrap();
            }
            let game = games_iter.next_back().unwrap();
            let label = format!("Game N-{}", k);
            if VERBOSE { h(1, &label); }
            if VERBOSE { pln(game); }
            let solution = game.solve_for(depth, cache);
            if VERBOSE { p_solution(&label, depth, &solution); }
        }
    }
}

fn run_backwards_solver<R: Rng>(rng: &mut R, depth: Count, n: Count) {
    if n > 0 {
        h(0, "Solving Back to Front");
        let games = random_games(rng);
        let mut games_iter = games.iter();
        let game_n = games_iter.next_back().unwrap();
        if VERBOSE { h(1, "Game N"); }
        if VERBOSE { pln(game_n); }
        let cache = &mut new_cache(500000000);
        for i in 1 .. (n + 1) {
            let label = &format!("N-{}", i);
            if VERBOSE { h(1, label) }
            let game = games_iter.next_back().unwrap();
            if VERBOSE { pln(game); }
            let solution = game.solve_for(depth, cache);
            // let solution = game.solve_for_uncached(depth);
            if VERBOSE { p_solution(label, depth, &solution); }
            if VERBOSE { p_cache(cache); }
        }
    }
}

// -- print functions ----------------------------------------------------------

fn p_solution(k: &str, d: Count, solution: &Solution) {
    println!("{} sol d={}: {}\n", k, d, solution.show());
}

fn p_cache(cache: &Cache) {
    println!("cache utilization: {} of {}\n", cache.len(), cache.capacity());
}

// -- str functions ------------------------------------------------------------

fn result_str(op: Option<Player>) -> &'static str {
    match op {
        Some(Player::X) => "X wins",
        Some(Player::O) => "O wins",
        None => "  tie ",
    }
}
