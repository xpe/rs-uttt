# Example Overflow

This example shows how Rust catches overflow:

```
const LOW_MARK: u32 = 9999999999; // 100;
const MULTIPLIER: u32 = 99999999999999; // 10000;

fn threshold(min: u32) -> u32 {
    if min < LOW_MARK {
        LOW_MARK * MULTIPLIER
    } else {
        min * MULTIPLIER
    }
```

Here are the compiler warnings from `cargo build`:

```
   Compiling byteorder v0.5.3
   Compiling hex v0.2.0
   Compiling libc v0.2.17
   Compiling linked-hash-map v0.2.1
   Compiling bufstream v0.1.2
   Compiling md5 v0.2.0
   Compiling fallible-iterator v0.1.3
   Compiling phf_shared v0.7.20
   Compiling log v0.3.6
   Compiling lru-cache v0.1.0
   Compiling phf v0.7.20
   Compiling postgres-protocol v0.1.0
   Compiling rand v0.3.14
   Compiling postgres v0.13.3
   Compiling uttt v0.1.0 (file:///Users/xpe/dev/rs-uttt)
warning: attempt to multiply with overflow, #[warn(const_err)] on by default
   --> src/solver/devices/ssd.rs:210:9
    |
210 |         LOW_MARK * MULTIPLIER
    |         ^^^^^^^^^^^^^^^^^^^^^

warning: this expression will panic at run-time
   --> src/solver/devices/ssd.rs:210:9
    |
210 |         LOW_MARK * MULTIPLIER
    |         ^^^^^^^^^^^^^^^^^^^^^ attempt to multiply with overflow

    Finished debug [unoptimized + debuginfo] target(s) in 6.84 secs
```

For background, see [RFC #560: Integer Overflow][RFC560].

[RFC560]: https://github.com/nikomatsakis/rfcs/blob/integer-overflow/text/0000-integer-overflow.md

Note: My editor, Atom, automatically builds the project, so it will display the
above warnings in the GUI.

However, if Atom does the first build, running the `cargo build` command from
the CLI is treated as a subsequent build and warning are suppressed.

Many compilers have a 'treat warnings as errors' flag, including rustc. However,
it would seem that cargo does not allow passing of such flags to rustc. More
discussion at:

* [a 23 Apr 2015 post on Reddit][33jpru]
* [rust-lang issue 921](https://github.com/rust-lang/cargo/issues/921)
* [rust-lang issue 544](https://github.com/rust-lang/cargo/issues/544)
* [rust-lang issue 60](https://github.com/rust-lang/cargo/issues/60)

[33jpru]: https://www.reddit.com/r/rust/comments/33jpru/promote_warnings_to_errors/
