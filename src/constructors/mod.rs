/// Constructor functions. Contains functions starting with 'from_'. This
/// module does not include accessors, constants, data structure definitions,
/// or game logic. 

use data::*;
use constants::*;

#[cfg(test)]
mod tests;

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
    pub fn from_slots(slots: &[Slot; 9]) -> SBoard {
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
        SBoard { encoding: v2 | v1 | v0 }
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
            _ => panic!("E1001"),
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
    pub fn from_row_col(row: RI, col: CI) -> Loc {
        Loc { encoding: row.as_u8() << 4 | col.as_u8() }
    }

    /// Returns a location from a board index and a sub-board index.
    pub fn from_indexes(bi: BI, sbi: SBI) -> Loc {
        match (bi, sbi) {
            (BI::I0, SBI::I0) => Loc::from_row_col(RI::R0, CI::C0),
            (BI::I0, SBI::I1) => Loc::from_row_col(RI::R0, CI::C1),
            (BI::I0, SBI::I2) => Loc::from_row_col(RI::R0, CI::C2),
            (BI::I0, SBI::I3) => Loc::from_row_col(RI::R1, CI::C0),
            (BI::I0, SBI::I4) => Loc::from_row_col(RI::R1, CI::C1),
            (BI::I0, SBI::I5) => Loc::from_row_col(RI::R1, CI::C2),
            (BI::I0, SBI::I6) => Loc::from_row_col(RI::R2, CI::C0),
            (BI::I0, SBI::I7) => Loc::from_row_col(RI::R2, CI::C1),
            (BI::I0, SBI::I8) => Loc::from_row_col(RI::R2, CI::C2),
            (BI::I1, SBI::I0) => Loc::from_row_col(RI::R0, CI::C3),
            (BI::I1, SBI::I1) => Loc::from_row_col(RI::R0, CI::C4),
            (BI::I1, SBI::I2) => Loc::from_row_col(RI::R0, CI::C5),
            (BI::I1, SBI::I3) => Loc::from_row_col(RI::R1, CI::C3),
            (BI::I1, SBI::I4) => Loc::from_row_col(RI::R1, CI::C4),
            (BI::I1, SBI::I5) => Loc::from_row_col(RI::R1, CI::C5),
            (BI::I1, SBI::I6) => Loc::from_row_col(RI::R2, CI::C3),
            (BI::I1, SBI::I7) => Loc::from_row_col(RI::R2, CI::C4),
            (BI::I1, SBI::I8) => Loc::from_row_col(RI::R2, CI::C5),
            (BI::I2, SBI::I0) => Loc::from_row_col(RI::R0, CI::C6),
            (BI::I2, SBI::I1) => Loc::from_row_col(RI::R0, CI::C7),
            (BI::I2, SBI::I2) => Loc::from_row_col(RI::R0, CI::C8),
            (BI::I2, SBI::I3) => Loc::from_row_col(RI::R1, CI::C6),
            (BI::I2, SBI::I4) => Loc::from_row_col(RI::R1, CI::C7),
            (BI::I2, SBI::I5) => Loc::from_row_col(RI::R1, CI::C8),
            (BI::I2, SBI::I6) => Loc::from_row_col(RI::R2, CI::C6),
            (BI::I2, SBI::I7) => Loc::from_row_col(RI::R2, CI::C7),
            (BI::I2, SBI::I8) => Loc::from_row_col(RI::R2, CI::C8),
            (BI::I3, SBI::I0) => Loc::from_row_col(RI::R3, CI::C0),
            (BI::I3, SBI::I1) => Loc::from_row_col(RI::R3, CI::C1),
            (BI::I3, SBI::I2) => Loc::from_row_col(RI::R3, CI::C2),
            (BI::I3, SBI::I3) => Loc::from_row_col(RI::R4, CI::C0),
            (BI::I3, SBI::I4) => Loc::from_row_col(RI::R4, CI::C1),
            (BI::I3, SBI::I5) => Loc::from_row_col(RI::R4, CI::C2),
            (BI::I3, SBI::I6) => Loc::from_row_col(RI::R5, CI::C0),
            (BI::I3, SBI::I7) => Loc::from_row_col(RI::R5, CI::C1),
            (BI::I3, SBI::I8) => Loc::from_row_col(RI::R5, CI::C2),
            (BI::I4, SBI::I0) => Loc::from_row_col(RI::R3, CI::C3),
            (BI::I4, SBI::I1) => Loc::from_row_col(RI::R3, CI::C4),
            (BI::I4, SBI::I2) => Loc::from_row_col(RI::R3, CI::C5),
            (BI::I4, SBI::I3) => Loc::from_row_col(RI::R4, CI::C3),
            (BI::I4, SBI::I4) => Loc::from_row_col(RI::R4, CI::C4),
            (BI::I4, SBI::I5) => Loc::from_row_col(RI::R4, CI::C5),
            (BI::I4, SBI::I6) => Loc::from_row_col(RI::R5, CI::C3),
            (BI::I4, SBI::I7) => Loc::from_row_col(RI::R5, CI::C4),
            (BI::I4, SBI::I8) => Loc::from_row_col(RI::R5, CI::C5),
            (BI::I5, SBI::I0) => Loc::from_row_col(RI::R3, CI::C6),
            (BI::I5, SBI::I1) => Loc::from_row_col(RI::R3, CI::C7),
            (BI::I5, SBI::I2) => Loc::from_row_col(RI::R3, CI::C8),
            (BI::I5, SBI::I3) => Loc::from_row_col(RI::R4, CI::C6),
            (BI::I5, SBI::I4) => Loc::from_row_col(RI::R4, CI::C7),
            (BI::I5, SBI::I5) => Loc::from_row_col(RI::R4, CI::C8),
            (BI::I5, SBI::I6) => Loc::from_row_col(RI::R5, CI::C6),
            (BI::I5, SBI::I7) => Loc::from_row_col(RI::R5, CI::C7),
            (BI::I5, SBI::I8) => Loc::from_row_col(RI::R5, CI::C8),
            (BI::I6, SBI::I0) => Loc::from_row_col(RI::R6, CI::C0),
            (BI::I6, SBI::I1) => Loc::from_row_col(RI::R6, CI::C1),
            (BI::I6, SBI::I2) => Loc::from_row_col(RI::R6, CI::C2),
            (BI::I6, SBI::I3) => Loc::from_row_col(RI::R7, CI::C0),
            (BI::I6, SBI::I4) => Loc::from_row_col(RI::R7, CI::C1),
            (BI::I6, SBI::I5) => Loc::from_row_col(RI::R7, CI::C2),
            (BI::I6, SBI::I6) => Loc::from_row_col(RI::R8, CI::C0),
            (BI::I6, SBI::I7) => Loc::from_row_col(RI::R8, CI::C1),
            (BI::I6, SBI::I8) => Loc::from_row_col(RI::R8, CI::C2),
            (BI::I7, SBI::I0) => Loc::from_row_col(RI::R6, CI::C3),
            (BI::I7, SBI::I1) => Loc::from_row_col(RI::R6, CI::C4),
            (BI::I7, SBI::I2) => Loc::from_row_col(RI::R6, CI::C5),
            (BI::I7, SBI::I3) => Loc::from_row_col(RI::R7, CI::C3),
            (BI::I7, SBI::I4) => Loc::from_row_col(RI::R7, CI::C4),
            (BI::I7, SBI::I5) => Loc::from_row_col(RI::R7, CI::C5),
            (BI::I7, SBI::I6) => Loc::from_row_col(RI::R8, CI::C3),
            (BI::I7, SBI::I7) => Loc::from_row_col(RI::R8, CI::C4),
            (BI::I7, SBI::I8) => Loc::from_row_col(RI::R8, CI::C5),
            (BI::I8, SBI::I0) => Loc::from_row_col(RI::R6, CI::C6),
            (BI::I8, SBI::I1) => Loc::from_row_col(RI::R6, CI::C7),
            (BI::I8, SBI::I2) => Loc::from_row_col(RI::R6, CI::C8),
            (BI::I8, SBI::I3) => Loc::from_row_col(RI::R7, CI::C6),
            (BI::I8, SBI::I4) => Loc::from_row_col(RI::R7, CI::C7),
            (BI::I8, SBI::I5) => Loc::from_row_col(RI::R7, CI::C8),
            (BI::I8, SBI::I6) => Loc::from_row_col(RI::R8, CI::C6),
            (BI::I8, SBI::I7) => Loc::from_row_col(RI::R8, CI::C7),
            (BI::I8, SBI::I8) => Loc::from_row_col(RI::R8, CI::C8),
        }
    }
}

// -- sub-board location -------------------------------------------------------

// Note: use `SLoc` struct.

// -- slot ---------------------------------------------------------------------

// Note: use `Slot` enum.

// -- board indexes ------------------------------------------------------------

// Note: use `RI` or `CI` enum.

impl RI {
    /// Convert a u8 value to a row index.
    pub fn from_u8(x: u8) -> RI {
        match x {
            0 => RI::R0,
            1 => RI::R1,
            2 => RI::R2,
            3 => RI::R3,
            4 => RI::R4,
            5 => RI::R5,
            6 => RI::R6,
            7 => RI::R7,
            8 => RI::R8,
            _ => panic!("E1002"),
        }
    }
}

impl CI {
    /// Convert a u8 value to a column index.
    pub fn from_u8(x: u8) -> CI {
        match x {
            0 => CI::C0,
            1 => CI::C1,
            2 => CI::C2,
            3 => CI::C3,
            4 => CI::C4,
            5 => CI::C5,
            6 => CI::C6,
            7 => CI::C7,
            8 => CI::C8,
            _ => panic!("E1003"),
        }
    }
}

impl BI {
    pub fn from_loc(loc: Loc) -> BI {
        BI::from_row_col(loc.row(), loc.col())
    }

    /// Returns a board index for a given board row and col.
    pub fn from_row_col(row: RI, col: CI) -> BI {
        match (row, col) {
            (RI::R0, CI::C0) => BI::I0, //
            (RI::R0, CI::C1) => BI::I0,
            (RI::R0, CI::C2) => BI::I0,
            (RI::R0, CI::C3) => BI::I1,
            (RI::R0, CI::C4) => BI::I1,
            (RI::R0, CI::C5) => BI::I1,
            (RI::R0, CI::C6) => BI::I2,
            (RI::R0, CI::C7) => BI::I2,
            (RI::R0, CI::C8) => BI::I2,
            (RI::R1, CI::C0) => BI::I0,
            (RI::R1, CI::C1) => BI::I0,
            (RI::R1, CI::C2) => BI::I0,
            (RI::R1, CI::C3) => BI::I1,
            (RI::R1, CI::C4) => BI::I1,
            (RI::R1, CI::C5) => BI::I1,
            (RI::R1, CI::C6) => BI::I2,
            (RI::R1, CI::C7) => BI::I2,
            (RI::R1, CI::C8) => BI::I2,
            (RI::R2, CI::C0) => BI::I0,
            (RI::R2, CI::C1) => BI::I0,
            (RI::R2, CI::C2) => BI::I0,
            (RI::R2, CI::C3) => BI::I1,
            (RI::R2, CI::C4) => BI::I1,
            (RI::R2, CI::C5) => BI::I1,
            (RI::R2, CI::C6) => BI::I2,
            (RI::R2, CI::C7) => BI::I2,
            (RI::R2, CI::C8) => BI::I2,
            (RI::R3, CI::C0) => BI::I3, //
            (RI::R3, CI::C1) => BI::I3,
            (RI::R3, CI::C2) => BI::I3,
            (RI::R3, CI::C3) => BI::I4,
            (RI::R3, CI::C4) => BI::I4,
            (RI::R3, CI::C5) => BI::I4,
            (RI::R3, CI::C6) => BI::I5,
            (RI::R3, CI::C7) => BI::I5,
            (RI::R3, CI::C8) => BI::I5,
            (RI::R4, CI::C0) => BI::I3,
            (RI::R4, CI::C1) => BI::I3,
            (RI::R4, CI::C2) => BI::I3,
            (RI::R4, CI::C3) => BI::I4,
            (RI::R4, CI::C4) => BI::I4,
            (RI::R4, CI::C5) => BI::I4,
            (RI::R4, CI::C6) => BI::I5,
            (RI::R4, CI::C7) => BI::I5,
            (RI::R4, CI::C8) => BI::I5,
            (RI::R5, CI::C0) => BI::I3,
            (RI::R5, CI::C1) => BI::I3,
            (RI::R5, CI::C2) => BI::I3,
            (RI::R5, CI::C3) => BI::I4,
            (RI::R5, CI::C4) => BI::I4,
            (RI::R5, CI::C5) => BI::I4,
            (RI::R5, CI::C6) => BI::I5,
            (RI::R5, CI::C7) => BI::I5,
            (RI::R5, CI::C8) => BI::I5,
            (RI::R6, CI::C0) => BI::I6, //
            (RI::R6, CI::C1) => BI::I6,
            (RI::R6, CI::C2) => BI::I6,
            (RI::R6, CI::C3) => BI::I7,
            (RI::R6, CI::C4) => BI::I7,
            (RI::R6, CI::C5) => BI::I7,
            (RI::R6, CI::C6) => BI::I8,
            (RI::R6, CI::C7) => BI::I8,
            (RI::R6, CI::C8) => BI::I8,
            (RI::R7, CI::C0) => BI::I6,
            (RI::R7, CI::C1) => BI::I6,
            (RI::R7, CI::C2) => BI::I6,
            (RI::R7, CI::C3) => BI::I7,
            (RI::R7, CI::C4) => BI::I7,
            (RI::R7, CI::C5) => BI::I7,
            (RI::R7, CI::C6) => BI::I8,
            (RI::R7, CI::C7) => BI::I8,
            (RI::R7, CI::C8) => BI::I8,
            (RI::R8, CI::C0) => BI::I6,
            (RI::R8, CI::C1) => BI::I6,
            (RI::R8, CI::C2) => BI::I6,
            (RI::R8, CI::C3) => BI::I7,
            (RI::R8, CI::C4) => BI::I7,
            (RI::R8, CI::C5) => BI::I7,
            (RI::R8, CI::C6) => BI::I8,
            (RI::R8, CI::C7) => BI::I8,
            (RI::R8, CI::C8) => BI::I8,
        }
    }

    /// Convert a u8 value to a row index.
    pub fn from_u8(x: u8) -> BI {
        match x {
            0 => BI::I0,
            1 => BI::I1,
            2 => BI::I2,
            3 => BI::I3,
            4 => BI::I4,
            5 => BI::I5,
            6 => BI::I6,
            7 => BI::I7,
            8 => BI::I8,
            _ => panic!("E1004"),
        }
    }
}

// -- sub-board indexes --------------------------------------------------------

// Note: use `SRI` or `SCI` enum.

impl SRI {
    /// Convert a sub-board index to a sub-board row index.
    pub fn from_idx(sbi: SBI) -> SRI {
        match sbi {
            SBI::I0 => SRI::R0,
            SBI::I1 => SRI::R0,
            SBI::I2 => SRI::R0,
            SBI::I3 => SRI::R1,
            SBI::I4 => SRI::R1,
            SBI::I5 => SRI::R1,
            SBI::I6 => SRI::R2,
            SBI::I7 => SRI::R2,
            SBI::I8 => SRI::R2,
        }
    }
}

impl SCI {
    /// Convert a sub-board index to a sub-board column index.
    pub fn from_idx(sbi: SBI) -> SCI {
        match sbi {
            SBI::I0 => SCI::C0,
            SBI::I1 => SCI::C1,
            SBI::I2 => SCI::C2,
            SBI::I3 => SCI::C0,
            SBI::I4 => SCI::C1,
            SBI::I5 => SCI::C2,
            SBI::I6 => SCI::C0,
            SBI::I7 => SCI::C1,
            SBI::I8 => SCI::C2,
        }
    }
}

impl SBI {
    /// Returns a sub-board index for a given board location.
    pub fn from_loc(loc: Loc) -> SBI {
        SBI::from_row_col(loc.row(), loc.col())
    }

    /// Returns a sub-board index for a given sub-board location.
    pub fn from_sloc(sloc: SLoc) -> SBI {
        match sloc {
            SLoc { row: SRI::R0, col: SCI::C0 } => SBI::I0,
            SLoc { row: SRI::R0, col: SCI::C1 } => SBI::I1,
            SLoc { row: SRI::R0, col: SCI::C2 } => SBI::I2,
            SLoc { row: SRI::R1, col: SCI::C0 } => SBI::I3,
            SLoc { row: SRI::R1, col: SCI::C1 } => SBI::I4,
            SLoc { row: SRI::R1, col: SCI::C2 } => SBI::I5,
            SLoc { row: SRI::R2, col: SCI::C0 } => SBI::I6,
            SLoc { row: SRI::R2, col: SCI::C1 } => SBI::I7,
            SLoc { row: SRI::R2, col: SCI::C2 } => SBI::I8,
        }
    }

    /// Returns a sub-board index for a given board row and col.
    pub fn from_row_col(row: RI, col: CI) -> SBI {
        match (row, col) {
            (RI::R0, CI::C0) => SBI::I0, // RI::R0
            (RI::R0, CI::C1) => SBI::I1,
            (RI::R0, CI::C2) => SBI::I2,
            (RI::R0, CI::C3) => SBI::I0,
            (RI::R0, CI::C4) => SBI::I1,
            (RI::R0, CI::C5) => SBI::I2,
            (RI::R0, CI::C6) => SBI::I0,
            (RI::R0, CI::C7) => SBI::I1,
            (RI::R0, CI::C8) => SBI::I2,
            (RI::R1, CI::C0) => SBI::I3, // RI::R1
            (RI::R1, CI::C1) => SBI::I4,
            (RI::R1, CI::C2) => SBI::I5,
            (RI::R1, CI::C3) => SBI::I3,
            (RI::R1, CI::C4) => SBI::I4,
            (RI::R1, CI::C5) => SBI::I5,
            (RI::R1, CI::C6) => SBI::I3,
            (RI::R1, CI::C7) => SBI::I4,
            (RI::R1, CI::C8) => SBI::I5,
            (RI::R2, CI::C0) => SBI::I6, // RI::R2
            (RI::R2, CI::C1) => SBI::I7,
            (RI::R2, CI::C2) => SBI::I8,
            (RI::R2, CI::C3) => SBI::I6,
            (RI::R2, CI::C4) => SBI::I7,
            (RI::R2, CI::C5) => SBI::I8,
            (RI::R2, CI::C6) => SBI::I6,
            (RI::R2, CI::C7) => SBI::I7,
            (RI::R2, CI::C8) => SBI::I8,
            (RI::R3, CI::C0) => SBI::I0, // RI::R3
            (RI::R3, CI::C1) => SBI::I1,
            (RI::R3, CI::C2) => SBI::I2,
            (RI::R3, CI::C3) => SBI::I0,
            (RI::R3, CI::C4) => SBI::I1,
            (RI::R3, CI::C5) => SBI::I2,
            (RI::R3, CI::C6) => SBI::I0,
            (RI::R3, CI::C7) => SBI::I1,
            (RI::R3, CI::C8) => SBI::I2,
            (RI::R4, CI::C0) => SBI::I3, // RI::R4
            (RI::R4, CI::C1) => SBI::I4,
            (RI::R4, CI::C2) => SBI::I5,
            (RI::R4, CI::C3) => SBI::I3,
            (RI::R4, CI::C4) => SBI::I4,
            (RI::R4, CI::C5) => SBI::I5,
            (RI::R4, CI::C6) => SBI::I3,
            (RI::R4, CI::C7) => SBI::I4,
            (RI::R4, CI::C8) => SBI::I5,
            (RI::R5, CI::C0) => SBI::I6, // RI::R5
            (RI::R5, CI::C1) => SBI::I7,
            (RI::R5, CI::C2) => SBI::I8,
            (RI::R5, CI::C3) => SBI::I6,
            (RI::R5, CI::C4) => SBI::I7,
            (RI::R5, CI::C5) => SBI::I8,
            (RI::R5, CI::C6) => SBI::I6,
            (RI::R5, CI::C7) => SBI::I7,
            (RI::R5, CI::C8) => SBI::I8,
            (RI::R6, CI::C0) => SBI::I0, // RI::R6
            (RI::R6, CI::C1) => SBI::I1,
            (RI::R6, CI::C2) => SBI::I2,
            (RI::R6, CI::C3) => SBI::I0,
            (RI::R6, CI::C4) => SBI::I1,
            (RI::R6, CI::C5) => SBI::I2,
            (RI::R6, CI::C6) => SBI::I0,
            (RI::R6, CI::C7) => SBI::I1,
            (RI::R6, CI::C8) => SBI::I2,
            (RI::R7, CI::C0) => SBI::I3, // RI::R7
            (RI::R7, CI::C1) => SBI::I4,
            (RI::R7, CI::C2) => SBI::I5,
            (RI::R7, CI::C3) => SBI::I3,
            (RI::R7, CI::C4) => SBI::I4,
            (RI::R7, CI::C5) => SBI::I5,
            (RI::R7, CI::C6) => SBI::I3,
            (RI::R7, CI::C7) => SBI::I4,
            (RI::R7, CI::C8) => SBI::I5,
            (RI::R8, CI::C0) => SBI::I6, // RI::R8
            (RI::R8, CI::C1) => SBI::I7,
            (RI::R8, CI::C2) => SBI::I8,
            (RI::R8, CI::C3) => SBI::I6,
            (RI::R8, CI::C4) => SBI::I7,
            (RI::R8, CI::C5) => SBI::I8,
            (RI::R8, CI::C6) => SBI::I6,
            (RI::R8, CI::C7) => SBI::I7,
            (RI::R8, CI::C8) => SBI::I8,
        }
    }

    /// Convert a u8 value to a row index.
    pub fn from_u8(x: u8) -> SBI {
        match x {
            0 => SBI::I0,
            1 => SBI::I1,
            2 => SBI::I2,
            3 => SBI::I3,
            4 => SBI::I4,
            5 => SBI::I5,
            6 => SBI::I6,
            7 => SBI::I7,
            8 => SBI::I8,
            _ => panic!("E10005"),
        }
    }
}

// -- player -------------------------------------------------------------------

// Note: use `Player` enum.
