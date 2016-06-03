use data::*;
use quickcheck::{QuickCheck};

#[test]
fn test_play_at() {
    fn prop(row: Row, sci: SCI, player: Player) -> bool {
        row.play_at(sci, player) == row.play_at_2(sci, player)
    }
    QuickCheck::new().tests(1000).quickcheck(
        prop as fn(Row, SCI, Player) -> bool
    );
}

#[test]
fn test_loc_from_indexes() {
    fn prop(bi: BI, sbi: SBI) -> bool {
        let loc = Loc::from_indexes(bi, sbi);
        (bi == BI::from_loc(loc)) && (sbi == SBI::from_loc(loc))
    }
    QuickCheck::new().tests(1000).quickcheck(
        prop as fn(BI, SBI) -> bool
    );
}

#[test]
fn test_game_state() {
    fn prop(game: Game) -> bool {
        match game.state() {
            GameState::Won(player) => {
                game.is_over() && game.winner() == Some(player)
            },
            GameState::Tied => {
                game.is_over() && game.winner() == None
            },
            GameState::Ongoing => {
                !game.is_over() && game.winner() == None
            },
        }
    }
    QuickCheck::new().tests(100).quickcheck(
        prop as fn(Game) -> bool
    );
}
