# Data Structure Sizes

From [the Rust Reference on machine-dependent integer types][mdit]:

> The usize type is an unsigned integer type with the same number of bits as the
> platform's pointer type. It can represent every memory address in the process.

So, for a 64-bit machine, a `usize` must be 64 bits wide.

These data structures have memory layouts **bigger** than [usize][usize]:

* `Game`
* `Board`
* `[Slot; 9]`: `u8` * 9 = 72 bits

These data structures have memory layouts **less** than `usize`:

* `[Slot; 3]`: `u8` * 3 = 24 bits
* `[Row; 3]` : `u8` * 3 : 24 bits
* `Solution`
* `Outcome`
* `GameState` : `u8`
* `SBoard` : `u16`
* `Row` : `u8`
* `Play`
* `SPlay`
* `Loc`
* `SLoc`
* `Slot` : `u8`
* `RI`, `CI`
* `SRI`, `SCI`
* `SCI`, `SBI`
* `Player`
* `Count`

[mdit]: https://doc.rust-lang.org/reference.html#machine-dependent-integer-types
[usize]: https://doc.rust-lang.org/std/primitive.usize.html
