/// Implementations of the Arbitrary trait. Used by quickcheck tests.

use data::*;
use quickcheck::{Arbitrary, Gen};

impl Arbitrary for Row {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        g.gen::<Row>()
    }
}

impl Arbitrary for RI {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        g.gen::<RI>()
    }
}

impl Arbitrary for CI {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        g.gen::<CI>()
    }
}

impl Arbitrary for BI {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        g.gen::<BI>()
    }
}

impl Arbitrary for SBI {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        g.gen::<SBI>()
    }
}

impl Arbitrary for SRI {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        g.gen::<SRI>()
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
