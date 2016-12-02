use constants::*;
use data::*;
use solver::*;

#[test]
fn test_empty_game() {
    let depth: Count = 0;
    let stack = CPU_Stack::new();
    assert!(
        EMPTY_GAME.solve(depth, &stack) ==
        vec![
            Solution {
                opt_play: None,
                outcome: Outcome::Unknown{ turns: 0 },
            }
        ]);
}
