use data::*;
use quickcheck::{QuickCheck};
use tests::arbitrary::*;

#[test]
fn test_play_at() {
    fn prop(row: Row, sci: SCI, player: Player) -> bool {
        row.play_at(sci, player) == row.play_at_2(sci, player)
    }
    QuickCheck::new().tests(1000).quickcheck(
        prop as fn(Row, SCI, Player) -> bool
    );
}
