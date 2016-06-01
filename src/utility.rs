/// Utility functions.

use show::Show;

/// Prints a heading with given level and title.
pub fn h(level: u8, title: &str) {
    let prefix = match level {
        0 => "# ",
        1 => "## ",
        2 => "### ",
        _ => "",
    };
    println!("\n{}{}\n", prefix, title);
}

/// Prints a reference that has the Show trait.
pub fn p<T: Show>(x: &T) {
    println!("{}", x.show());
}