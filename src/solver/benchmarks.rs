use data::*;
use rand::{SeedableRng, XorShiftRng};
use random::{random_games};
use solver::*;
use test::Bencher;

fn solve_example_1(k: Count, depth: Count) {
    let seed: [u32; 4] = [1456198685, 762656086, 844876651, 1745969790];
    let mut rng: XorShiftRng = SeedableRng::from_seed(seed);
    let games = random_games(&mut rng);
    let mut games_iter = games.iter();
    for _ in 0 .. k {
        games_iter.next_back();
    }
    let game = games_iter.next_back().expect("E1401");
    let stack = CPU_Stack::new();
    game.solve(depth, &stack);
}

#[bench]
fn bench_solve(b: &mut Bencher) {
    solve_example_1(4, 6);
    b.iter(|| solve_example_1(4, 6))
}
