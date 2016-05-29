/// Constructor functions. This module does not include accessors,
/// constants, data structure definitions, or game logic.

use data::{SBoard, Row, Loc, Slot, RI, CI};
use constants::{SE, SX, SO};

// -- game ---------------------------------------------------------------------

// Note: use `Game` struct.

// -- board --------------------------------------------------------------------

// Note: use `SBoard` struct.

// -- sub-board ----------------------------------------------------------------

impl SBoard {
    /// Return a sub-board from 3 rows.
    pub fn from_rows(rows: [Row; 3]) -> SBoard {
        SBoard::from_u8s(
            Row::as_u8(rows[0]),
            Row::as_u8(rows[1]),
            Row::as_u8(rows[2])
        )
    }

    /// Return a sub-board from 9 slots.
    #[allow(unused_variables)]
    pub fn from_slots(slots: [Slot; 9]) -> SBoard {
        let mut s0 = [SE; 3];
        let mut s1 = [SE; 3];
        let mut s2 = [SE; 3];
        s0.copy_from_slice(&slots[0 .. 3]);
        s1.copy_from_slice(&slots[3 .. 6]);
        s2.copy_from_slice(&slots[6 .. 9]);
        SBoard::from_u8s(
            Row::as_u8(Row::from_slots(s0)),
            Row::as_u8(Row::from_slots(s1)),
            Row::as_u8(Row::from_slots(s2)),
        )
    }

    /// Return a sub-board from 3 u8's.
    fn from_u8s(x0: u8, x1: u8, x2: u8) -> SBoard {
        let v2: u16 = ((x2 & 0b11111) as u16) << 10;
        let v1: u16 = ((x1 & 0b11111) as u16) << 5;
        let v0: u16 = (x0 & 0b11111) as u16;
        SBoard(v2 | v1 | v0)
    }
}

// -- row ----------------------------------------------------------------------

impl Row {
    /// Returns a row from an array of three slots.
    pub fn from_slots(slots: [Slot; 3]) -> Row {
        match slots {
            [SE, SE, SE] => Row::EEE,
            [SE, SE, SO] => Row::EEO,
            [SE, SE, SX] => Row::EEX,
            [SE, SO, SE] => Row::EOE,
            [SE, SO, SO] => Row::EOO,
            [SE, SO, SX] => Row::EOX,
            [SE, SX, SE] => Row::EXE,
            [SE, SX, SO] => Row::EXO,
            [SE, SX, SX] => Row::EXX,
            [SO, SE, SE] => Row::OEE,
            [SO, SE, SO] => Row::OEO,
            [SO, SE, SX] => Row::OEX,
            [SO, SO, SE] => Row::OOE,
            [SO, SO, SO] => Row::OOO,
            [SO, SO, SX] => Row::OOX,
            [SO, SX, SE] => Row::OXE,
            [SO, SX, SO] => Row::OXO,
            [SO, SX, SX] => Row::OXX,
            [SX, SE, SE] => Row::XEE,
            [SX, SE, SO] => Row::XEO,
            [SX, SE, SX] => Row::XEX,
            [SX, SO, SE] => Row::XOE,
            [SX, SO, SO] => Row::XOO,
            [SX, SO, SX] => Row::XOX,
            [SX, SX, SE] => Row::XXE,
            [SX, SX, SO] => Row::XXO,
            [SX, SX, SX] => Row::XXX
        }
    }

    /// Returns a row from a unsigned 8-bit integer.
    pub fn from_u8(x: u8) -> Row {
        match x {
            0x00 => Row::EEE,
            0x01 => Row::EEX,
            0x02 => Row::EEO,
            0x03 => Row::EXE,
            0x04 => Row::EXX,
            0x05 => Row::EXO,
            0x06 => Row::EOE,
            0x07 => Row::EOX,
            0x08 => Row::EOO,
            0x09 => Row::XEE,
            0x0A => Row::XEX,
            0x0B => Row::XEO,
            0x0C => Row::XXE,
            0x0D => Row::XXX,
            0x0E => Row::XXO,
            0x0F => Row::XOE,
            0x10 => Row::XOX,
            0x11 => Row::XOO,
            0x12 => Row::OEE,
            0x13 => Row::OEX,
            0x14 => Row::OEO,
            0x15 => Row::OXE,
            0x16 => Row::OXX,
            0x17 => Row::OXO,
            0x18 => Row::OOE,
            0x19 => Row::OOX,
            0x1A => Row::OOO,
            _ => panic!("internal error")
        }
    }
}

// -- board play ---------------------------------------------------------------

// Note: use `Play` struct.

// -- sub-board play -----------------------------------------------------------

// Note: use `SPlay` struct.

// -- board location -----------------------------------------------------------

impl Loc {
    /// Returns a location based on row and column indexes.
    #[allow(unused_variables)]
    pub fn new(row: RI, col: CI) -> Loc {
        Loc(0x00) // TODO: FIX
    }
}

// -- sub-board location -------------------------------------------------------

// Note: use `SLoc` struct.

// -- slot ---------------------------------------------------------------------

// Note: use `Slot` enum.

// -- board indexes ------------------------------------------------------------

// Note: use `RI` or `CI` enum.

// -- sub-board indexes --------------------------------------------------------

// Note: use `SRI` or `SCI` enum.

// -- player -------------------------------------------------------------------

// Note: use `Player` enum.
