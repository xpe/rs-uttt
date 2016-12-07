use data::*;
use quickcheck::{QuickCheck};

// -- board --------------------------------------------------------------------

// -- sub-board ----------------------------------------------------------------

#[test]
fn test_sboard_from_rows() {
    assert!(SBoard::from_rows([Row::EEE, Row::EEE, Row::EEE]) ==
            SBoard { encoding: 0b0000000000000000 });
    assert!(SBoard::from_rows([Row::OEX, Row::OEX, Row::EEX]) ==
            SBoard { encoding: 0b0000011001110011 });
}

// -- row ----------------------------------------------------------------------

// -- board play ---------------------------------------------------------------

// -- sub-board play -----------------------------------------------------------

// -- board location -----------------------------------------------------------

#[test]
fn test_loc_new() {
    assert!(Loc::from_row_col(RI::R7, CI::C4) == Loc { encoding: 0b01110100 });
    assert!(Loc::from_row_col(RI::R8, CI::C5) == Loc { encoding: 0b10000101 });
}

#[test]
fn test_loc_new_row_col() {
    fn prop(ri: RI, ci: CI) -> bool {
        let loc = Loc::from_row_col(ri, ci);
        ri == loc.row() && ci == loc.col()
    }
    QuickCheck::new().tests(200).quickcheck(
        prop as fn(RI, CI) -> bool
    );
}

// -- sub-board location -------------------------------------------------------

// -- slot ---------------------------------------------------------------------

// -- board indexes ------------------------------------------------------------

// -- sub-board indexes --------------------------------------------------------

// -- player -------------------------------------------------------------------
