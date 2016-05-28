use data::{Board, SBoard, Row, Slot, RI, CI, SRI, SCI};
use constants::{SE, SX, SO};

// -- -> game ------------------------------------------------------------------

// -- -> board -----------------------------------------------------------------

// -- -> sub-board -------------------------------------------------------------

// -- -> rows ------------------------------------------------------------------

impl SBoard {
    pub fn rows(self) -> [Row; 3] {
        unimplemented!(); // TODO
    }
}

// -- -> row -------------------------------------------------------------------

impl SBoard {
    pub fn row(self, ri: SRI) -> Row {
        let s = match ri { SRI::R0 => 0, SRI::R1 => 5, SRI::R2 => 10 };
        Row::from_u8((self.0 >> s & 0b11111) as u8)
    }
}

// -- -> board play ------------------------------------------------------------

// -- -> sub-board play --------------------------------------------------------

// -- -> board location --------------------------------------------------------

// -- -> sub-board location ----------------------------------------------------

// -- -> slots -----------------------------------------------------------------

impl Row {
    /// Returns an array of three slots for a given row.
    pub fn as_slots(row: Row) -> [Slot; 3] {
        match row {
            Row::EEE => [SE, SE, SE],
            Row::EEO => [SE, SE, SO],
            Row::EEX => [SE, SE, SX],
            Row::EOE => [SE, SO, SE],
            Row::EOO => [SE, SO, SO],
            Row::EOX => [SE, SO, SX],
            Row::EXE => [SE, SX, SE],
            Row::EXO => [SE, SX, SO],
            Row::EXX => [SE, SX, SX],
            Row::OEE => [SO, SE, SE],
            Row::OEO => [SO, SE, SO],
            Row::OEX => [SO, SE, SX],
            Row::OOE => [SO, SO, SE],
            Row::OOO => [SO, SO, SO],
            Row::OOX => [SO, SO, SX],
            Row::OXE => [SO, SX, SE],
            Row::OXO => [SO, SX, SO],
            Row::OXX => [SO, SX, SX],
            Row::XEE => [SX, SE, SE],
            Row::XEO => [SX, SE, SO],
            Row::XEX => [SX, SE, SX],
            Row::XOE => [SX, SO, SE],
            Row::XOO => [SX, SO, SO],
            Row::XOX => [SX, SO, SX],
            Row::XXE => [SX, SX, SE],
            Row::XXO => [SX, SX, SO],
            Row::XXX => [SX, SX, SX]
        }
    }
}

// -- -> slot ------------------------------------------------------------------

impl Board {
    /// Returns the Slot at a particular row and column in a board.
    #[allow(unused_variables)]
    pub fn slot(self, row: RI, col: CI) -> Slot {
        unimplemented!(); // TODO
    }
}

impl SBoard {
    /// Returns the Slot at a particular row and column in a sub-board.
    #[allow(unused_variables)]
    pub fn slot(self, row: SRI, col: SCI) -> Slot {
        unimplemented!(); // TODO
    }
}

impl Row {
    /// Returns the Slot for a particular column index for a given Row.
    #[allow(unused_variables)]
    pub fn slot(self, col: SCI) -> Slot {
        unimplemented!(); // TODO
    }
}

// -- -> board indexes ---------------------------------------------------------

// -- -> sub-board indexes -----------------------------------------------------

// -- -> player ----------------------------------------------------------------

// -- -> u8 --------------------------------------------------------------------

impl Row {
    /// Convert a Row into a u8 value.
    pub fn as_u8(row: Row) -> u8 {
        match row {
            Row::EEE => 0x00,
            Row::EEX => 0x01,
            Row::EEO => 0x02,
            Row::EXE => 0x03,
            Row::EXX => 0x04,
            Row::EXO => 0x05,
            Row::EOE => 0x06,
            Row::EOX => 0x07,
            Row::EOO => 0x08,
            Row::XEE => 0x09,
            Row::XEX => 0x0A,
            Row::XEO => 0x0B,
            Row::XXE => 0x0C,
            Row::XXX => 0x0D,
            Row::XXO => 0x0E,
            Row::XOE => 0x0F,
            Row::XOX => 0x10,
            Row::XOO => 0x11,
            Row::OEE => 0x12,
            Row::OEX => 0x13,
            Row::OEO => 0x14,
            Row::OXE => 0x15,
            Row::OXX => 0x16,
            Row::OXO => 0x17,
            Row::OOE => 0x18,
            Row::OOX => 0x19,
            Row::OOO => 0x1A
        }
    }
}
