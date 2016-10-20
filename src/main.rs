extern crate rand;
extern crate uttt;

#[macro_use]
extern crate chan;

extern crate chan_signal;

use chan::{Sender, Receiver};
use chan_signal::Signal;
use rand::{Rng, XorShiftRng, SeedableRng};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use uttt::data::*;
use uttt::random::*;
use uttt::solver::*;
use uttt::utility::{h, p, pln};

const VERBOSE: bool = true;

// -- main ---------------------------------------------------------------------

fn main() {
    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    let (tx_done, rx_done) = chan::sync(0);
    let (tx_quit, rx_quit) = chan::sync(0);
    let stack: Arc<Mutex<Stack>> = Arc::new(Mutex::new(SSD_CPU_Stack::new()));
    let stack_2 = stack.clone();
    let child = thread::spawn(move || run(stack_2, rx_quit, tx_done));
    loop {
        chan_select! {
            signal.recv() -> signal => {
                println!("Received signal: {:?}", signal);
                tx_quit.send(());
            },
            rx_done.recv() => {
                print!("Waiting for main thread to finish... ");
                child.join().expect("E99XX");
                print!("done.\nFlushing the solver stack... ");
                match stack.lock() {
                    Ok(guard) => {
                        let stack: &Stack = guard.deref();
                        let (success, count) = stack.flush();
                        if success {
                            println!("success ({} items).", count);
                        } else {
                            println!("failure ({} items).", count);
                        }
                    },
                    Err(_) => panic!("E99XX"),
                }
                println!("Program completed.");
                return;
            },
        }
    }
}

fn run(stack: Arc<Mutex<Stack>>, quit: Receiver<()>, _done: Sender<()>) {
    match stack.lock() {
        Ok(guard) => {
            let stack: &Stack = guard.deref();
            let mut rng = make_rng();
            run_random_games(0, &mut rng);
            run_random_game(0, &mut rng);
            run_solve(0, &stack, &mut rng, 5, 7);
            run_backwards_solve(0, &stack, &mut rng, 81, 10);
            run_full_backwards_solve(0, &stack, &mut rng);
            run_ongoing_backwards_solve(true, &stack, quit, &mut rng, 8, 10);
        },
        Err(_) => panic!("E99XX"),
    }
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
            let game = games.iter().last().expect("E99XX");
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
            let game_n = games_iter.next_back().expect("E99XX");
            if VERBOSE { h(2, "Game N"); }
            if VERBOSE { pln(game_n); }

            // TODO: It would be nice to extract out this chunk of code;
            // however, the types have flummoxed me so far.
            //
            // Back up `back - 1` times.
            for _ in 0 .. (back - 1) {
                games_iter.next_back().expect("E99XX");
            }
            // Back up one more time.
            let game = games_iter.next_back().expect("E99XX");

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
            let game_n = games_iter.next_back().expect("E99XX");
            if VERBOSE { h(2, "Game N"); }
            if VERBOSE { pln(game_n); }
            for i in 1 .. (n + 1) {
                let label = &format!("N-{}", i);
                if VERBOSE { h(2, label) }
                let game = games_iter.next_back().expect("E99XX");
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
    (active: bool, stack: &Stack, quit: Receiver<()>,
        rng: &mut R, depth: Count, n: Count) {
    if active {
        let mut trial: u32 = 0;
        h(0, "Backwards Solve (Ongoing)");
        'outer: loop {
            trial += 1;
            if VERBOSE { h(1, &format!("Trial #{}", trial)); }
            let games = random_games(rng);
            let mut games_iter = games.iter();
            let game_n = games_iter.next_back().expect("E99XX");
            if VERBOSE { h(2, &format!("Trial #{} Game N", trial)); }
            if VERBOSE { p_cache(stack); }
            if VERBOSE { pln(game_n); }
            for i in 1 .. (n + 1) {
                chan_select! {
                    default => {
                        let label = &format!("Trial #{} Game N-{}", trial, i);
                        if VERBOSE { h(2, label); }
                        if VERBOSE { p_cache(stack); }
                        let game = games_iter.next_back().expect("E99XX");
                        if VERBOSE { pln(game); }
                        let solutions = solve(stack, &game, depth);
                        if VERBOSE { p_solutions(label, depth, &solutions); }
                    },
                    quit.recv() => {
                        h(0, "Ending Backwards Solve");
                        break 'outer;
                    },
                }
            }
        }
    }
}

// -- solve function(s) --------------------------------------------------------

fn solve(stack: &Stack, game: &Game, depth: Count) -> Vec<Solution> {
    stack.get_and_put(game, depth)
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
    let device = stack.devices.get(0).expect("E99XX");
    println!("SSD RAM cache_1 size : {}", SSD::cache_1_len(&device));
    println!("SSD RAM cache_2 size : {}\n", SSD::cache_2_len(&device));
}
