extern crate uttt;

use uttt::data::{SBoard};
use uttt::constants::{SE, SX, SO};

fn main() {
    let slots = [
        SE, SE, SE,
        SE, SE, SX,
        SO, SX, SE,
    ];
    println!("{:b}", SBoard::from_slots(slots).0);
}
