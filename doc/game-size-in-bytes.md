# Bits Required to Store a Game

## Summary / Conclusion

If one wants to compactly store a game, 151 bits (i.e. 19 bytes) are needed.

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
  ^ 7 = 128 possibilities.
* How many bits to store a game? 144 + 7 = 151 bits (18.875 bytes)
