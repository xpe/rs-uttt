use data::*;
use quickcheck::{Arbitrary, Gen, QuickCheck};

impl Arbitrary for Row {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        g.gen::<Row>()
    }
}

impl Arbitrary for SCI {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        g.gen::<SCI>()
    }
}

impl Arbitrary for Player {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        g.gen::<Player>()
    }
}

#[test]
fn test_play_at() {
    fn prop(row: Row, sci: SCI, player: Player) -> bool {
        row.play_at(sci, player) == row.play_at_2(sci, player)
    }
    QuickCheck::new().tests(1000).quickcheck(
        prop as fn(Row, SCI, Player) -> bool
    );
}
