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
    // let seed = random_seed();
    let seed = [3294465295, 97182992, 4241695563, 163574426];
    let mut rng: XorShiftRng = SeedableRng::from_seed(seed);
    run_random_games(&mut rng, 0); // 100
    run_random_game(&mut rng, 0); // 5
    run_solve_1(&mut rng, 0); // 100
    run_solve_2(&mut rng, 1); // 100
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
        h(0, "Solve 1");
        for i in 0 .. trials {
            if VERBOSE { h(1, &format!("Trial #{}", i)); }

            let games = random_games(rng);
            let mut games_iter = games.iter();

            if VERBOSE { h(1, "Game N"); }
            let game_n = games_iter.next_back().unwrap();
            if VERBOSE { pln(game_n); }
            let solution_n = game_n.solve_for(0);
            if VERBOSE { p_solution("N", 0, &solution_n); }

            if VERBOSE { h(1, "Game N-1"); }
            let game_nm1 = games_iter.next_back().unwrap();
            if VERBOSE { pln(game_nm1); }
            let sol_nm1_d1 = game_nm1.solve_for(1);
            if VERBOSE { p_solution("N-1", 1, &sol_nm1_d1); }
        }
    }
}

fn run_solve_2<R: Rng>(rng: &mut R, trials: u16) {
    if trials > 0 {
        h(0, "Solve 2");
        for i in 0 .. trials {
            if VERBOSE { h(1, &format!("Trial #{}", i)); }

            let games = random_games(rng);
            let mut games_iter = games.iter();
            let game_n = games_iter.next_back().unwrap();
            games_iter.next_back().unwrap(); // game_n_1
            let game_n_2 = games_iter.next_back().unwrap();

            if VERBOSE { h(1, "Game N"); }
            if VERBOSE { pln(game_n); }

            if VERBOSE { h(1, "Game N-2"); }
            if VERBOSE { pln(game_n_2); }
            let sol_nm2_d1 = game_n_2.solve_for(1);
            if VERBOSE { p_solution("N-2", 1, &sol_nm2_d1); }
            let sol_nm2_d2 = game_n_2.solve_for(2);
            if VERBOSE { p_solution("N-2", 2, &sol_nm2_d2); }

            if VERBOSE { h(2, "Game N-2+1"); }
            let play = sol_nm2_d2.opt_play.unwrap();
            let game_nm2p1 = game_n_2.play(play).unwrap();
            if VERBOSE { pln(&game_nm2p1); }
            let sol_nm2p1_d2 = game_nm2p1.solve_for(2);
            if VERBOSE { p_solution("N-2+1", 2, &sol_nm2p1_d2); }
        }
    }
}

// -- print functions ----------------------------------------------------------

fn p_solution(k: &str, d: Count, solution: &Solution) {
    println!("{} solution (depth={}): {}\n", k, d, solution.show());
}

// -- str functions ------------------------------------------------------------

fn result_str(op: Option<Player>) -> &'static str {
    match op {
        Some(Player::X) => "X wins",
        Some(Player::O) => "O wins",
        None => "  tie ",
    }
}
