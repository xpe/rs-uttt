## Question

Should I use `pub flags` instead of `flags` with `bitflags!`? Why or why not?

From [the bitflags documentation][bf]:

> Visibility: The generated struct and its associated flag constants are not
> exported out of the current module by default. A definition can be exported
> out of the current module by adding pub before flags:

[bf]: https://doc.rust-lang.org/bitflags/bitflags/macro.bitflags!.html
