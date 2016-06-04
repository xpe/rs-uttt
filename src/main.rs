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
    let seed = random_seed();
    // let seed = [3294465295, 97182992, 4241695563, 163574426];
    let mut rng: XorShiftRng = SeedableRng::from_seed(seed);
    run_random_games(&mut rng, 0);
    run_random_game(&mut rng, 0);
    run_solve_1(&mut rng, 1);
    run_solve_2(&mut rng, 0);
    run_solve_3(&mut rng, 0);
    run_solve_4(&mut rng, 0);
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

fn run_solve_1<R: Rng>(rng: &mut R, trials: u16) {
    if trials > 0 {
        h(0, "Solve N-1");
        for i in 0 .. trials {
            if VERBOSE { h(1, &format!("Trial #{}", i)); }

            let games = random_games(rng);
            let mut games_iter = games.iter();
            let game_n = games_iter.next_back().unwrap();
            let game_nm1 = games_iter.next_back().unwrap();

            if VERBOSE { h(1, "Game N"); }
            if VERBOSE { pln(game_n); }

            if VERBOSE { h(1, "Game N-1"); }
            if VERBOSE { pln(game_nm1); }
            p_solve("N-1", game_nm1, 1);
        }
    }
}

fn run_solve_2<R: Rng>(rng: &mut R, trials: u16) {
    if trials > 0 {
        h(0, "Solve N-2");
        for i in 0 .. trials {
            if VERBOSE { h(1, &format!("Trial #{}", i)); }

            let games = random_games(rng);
            let mut games_iter = games.iter();
            let game_n = games_iter.next_back().unwrap();
            games_iter.next_back().unwrap(); // game_nm1
            let game_nm2 = games_iter.next_back().unwrap();

            if VERBOSE { h(1, "Game N"); }
            if VERBOSE { pln(game_n); }

            if VERBOSE { h(1, "Game N-2"); }
            if VERBOSE { pln(game_nm2); }
            p_solve("N-2", game_nm2, 1);
        }
    }
}

fn run_solve_3<R: Rng>(rng: &mut R, trials: u16) {
    if trials > 0 {
        h(0, "Solve N-3");
        for i in 0 .. trials {
            if VERBOSE { h(1, &format!("Trial #{}", i)); }

            let games = random_games(rng);
            let mut games_iter = games.iter();
            let game_n = games_iter.next_back().unwrap();
            games_iter.next_back().unwrap(); // game_nm1
            games_iter.next_back().unwrap(); // game_nm2
            let game_nm3 = games_iter.next_back().unwrap();

            if VERBOSE { h(1, "Game N"); }
            if VERBOSE { pln(game_n); }

            if VERBOSE { h(1, "Game N-3"); }
            if VERBOSE { pln(game_nm3); }
            p_solve("N-3", game_nm3, 1);
        }
    }
}

fn run_solve_4<R: Rng>(rng: &mut R, trials: u16) {
    if trials > 0 {
        h(0, "Solve N-4");
        for i in 0 .. trials {
            if VERBOSE { h(1, &format!("Trial #{}", i)); }

            let games = random_games(rng);
            let mut games_iter = games.iter();
            let game_n = games_iter.next_back().unwrap();
            games_iter.next_back().unwrap(); // game_nm1
            games_iter.next_back().unwrap(); // game_nm2
            games_iter.next_back().unwrap(); // game_nm3
            let game_nm4 = games_iter.next_back().unwrap();

            if VERBOSE { h(1, "Game N"); }
            if VERBOSE { pln(game_n); }

            if VERBOSE { h(1, "Game N-4"); }
            if VERBOSE { pln(game_nm4); }
            p_solve("N-4", game_nm4, 1);
        }
    }
}

fn p_solve(label: &str, game: &Game, depth: Count) {
    let solution = game.solve_for(depth);
    if VERBOSE { p_solution(label, depth, &solution); }
}

// -- print functions ----------------------------------------------------------

fn p_solution(k: &str, d: Count, solution: &Solution) {
    println!("{} sol d={}: {}\n", k, d, solution.show());
}

// -- str functions ------------------------------------------------------------

fn result_str(op: Option<Player>) -> &'static str {
    match op {
        Some(Player::X) => "X wins",
        Some(Player::O) => "O wins",
        None => "  tie ",
    }
}
