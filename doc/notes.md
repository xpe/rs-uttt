# Notes

## Bitwise Operators

From:
https://doc.rust-lang.org/reference.html#bitwise-operators

> Like the arithmetic operators, bitwise operators are syntactic sugar for calls
> to methods of built-in traits. This means that bitwise operators can be
> overridden for user-defined types. The default meaning of the operators on
> standard types is given here. Bitwise &, | and ^ applied to boolean arguments
> are equivalent to logical &&, || and != evaluated in non-lazy fashion.

* `&` : Bitwise AND. Calls the bitand method of the std::ops::BitAnd trait.
* `|` : Bitwise inclusive OR. Calls the bitor method of the std::ops::BitOr
  trait.
* `^` : Bitwise exclusive OR. Calls the bitxor method of the std::ops::BitXor
  trait.
* `<<` : Left shift. Calls the shl method of the std::ops::Shl trait.
* `>>` : Right shift (arithmetic). Calls the shr method of the std::ops::Shr
  trait.

## Option Type

According to [Wikipedia: Option Type][wot], Rust has an option type:

```
enum Option<T> { None, Some(T) }
```

[wot]: https://en.wikipedia.org/wiki/Option_type

## Conversion Traits

Using the traits from [std::convert] may be preferable to creating functions
named `x_to_y`:

[std::convert]: https://doc.rust-lang.org/std/convert/

## Naming Conventions

Conversion Functions:

* `to_` : ?
* `as_` : inexpensive conversion
* `into_` : ?
