# Bits Required to Store a Game

## Summary / Conclusion

Here, I list three ways to store a game:

1. As compactly as possible: 9 * 15 + 7 = 142 bits
2. Less compactly: 9 * 16 + 7 = 151 bits
3. Conveniently for a database: 9 * 16 + 8 = 152 bits

For database persistence, I choose option #2. The following shows the
calculation:

## Calculation

A game consists of a board and the last location.

* How many bits to store a sub-board? The SBoard data structure is stored with
  16 bits. (Yes, there is 1 wasted bit; 15 bits would be enough. However, to
  make byte manipulations easier and faster, we tolerate 1 wasted bit per
  sub-board.)
* How many bits to store a board? Since there are 9 nine sub-boards, we need 9
  x 16 = 144 bits (18 bytes) to store an entire board.
* How many bits to store the last location? While the game is underway, there
  are 81 board positions. At the start of the game, there is no 'last location'
  but it still must be representable in binary form. This means there need to be
  81 + 1 possibilities. 6 bits is not enough, since it can represent 2 ^ 6 =
  64 possibilities. 7 bits is sufficient and optimal, since it can represent 2
  ^ 7 = 128 possibilities. In this case, the program uses 8 bits. This is
  1 bit more than needed but makes byte alignment easier.
* How many bits to store a game? 144 + 8 = 152 bits (19 bytes)
