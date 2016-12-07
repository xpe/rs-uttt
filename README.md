# rs-uttt

This is a minimax solver (with pruning) of Ultimate Tic Tac Toe (UTTT) written
in Rust.

## About UTTT

Ultimate Tic Tac Toe is a modified version of Tic Tac Toe with a 9x9 board.
There are various online versions available, including:

* [UTTT by Lindsay Anchors and David James](http://ultimate-ttt.herokuapp.com)
* [UTTT by Yannick Rietz](http://bejofo.net/ttt)

Both links have instructions on how to play.

## Setup

### Install Rust

I have used [Nightly Rust][nightly-rust] for this project. I recommend
installing Rust using [rustup][rustup]. The list below shows the steps. If you
want additional context, I recommend reading the [rustup README][1].

[1]: https://github.com/rust-lang-nursery/rustup.rs

1. [Download rustup][rustup].
2. Run `rustup install nightly`.
3. To update rustup, run `rustup update`.

[rustup]: https://www.rustup.rs/

[nightly-rust]: https://doc.rust-lang.org/book/nightly-rust.html

### Database Setup

TODO

## Running

```
cargo build --release
```

The "run" script does the same thing.

### Example Run

Here is a snippet from an example run.

```
### Trial #8 Game N-5

SSD RAM cache_1 size : 1000
SSD RAM cache_2 size : 6509731

     0   1   2    3   4   5    6   7   8

0    X │   │      X │   │      O │ O │      0
    ───┼───┼───  ───┼───┼───  ───┼───┼───
1    O │ O │ O      │ X │        │   │ O    1
    ───┼───┼───  ───┼───┼───  ───┼───┼───
2      │ X │      X │   │        │ O │ O    2

3      │   │ X      │   │ X      │   │ X    3
    ───┼───┼───  ───┼───┼───  ───┼───┼───
4      │   │        │ X │ O    O │ X │      4
    ───┼───┼───  ───┼───┼───  ───┼───┼───
5      │ X │      X │ O │        │ X │      5

6    O │ O │      X │   │ X      │   │ X    6
    ───┼───┼───  ───┼───┼───  ───┼───┼───
7      │ O │      O │ O │        │ O │      7
    ───┼───┼───  ───┼───┼───  ───┼───┼───
8    X │   │ O    X │ O │        │   │ X    8

     0   1   2    3   4   5    6   7   8
    n=38      last=O:❨R4,C6❩      ongoing

- Trial #8 Game N-5 depth=11
    ❨Play:❨X:❨R3,C1❩❩ Outcome:❨Win:X 11❩❩
    ❨Play:❨X:❨R5,C0❩❩ Outcome:❨Win:X 11❩❩
    ❨Play:❨X:❨R5,C2❩❩ Outcome:❨Win:X 11❩❩
    ❨Play:❨X:❨R4,C2❩❩ Outcome:❨Win:X 11❩❩
    ❨Play:❨X:❨R4,C1❩❩ Outcome:❨Win:X 11❩❩
    ❨Play:❨X:❨R3,C0❩❩ Outcome:❨Win:X 11❩❩
    ❨Play:❨X:❨R4,C0❩❩ Outcome:❨Win:X 11❩❩
```

The board shown at the top is board position that the program is solving via
minimax. (Each board position is chosen at random.) The rows and columns are
marked along their respective axes. The "n=38" indicates that 38 moves have
been made. The last move was made by O at (row=4, col=6). The label "ongoing"
means that the game has not been won by either player or tied.

In this case, the solver found 7 "correct" plays (listed at the bottom) that
ensures a win for X in 11 moves.

The top two lines reflect the two RAM caches. In reverse order:

* The second cache has 6,509,731 filled slots out of `CACHE_2_CAP` (50,000,000)
total. This is a general purpose RAM cache, used to speed read access over
previous computed board positions.

* The first cache has 1000 filled slots of out `CACHE_1_CAP` (1000) total. It
is used to delay writes to the PostgreSQL database. This effectively reduces
"write-thrashing" since the minimax algorithm explores many of the same board
positions with varying depths.
