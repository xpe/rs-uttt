I will demonstrate what I believe is a novel approach to solving the game of
[Ultimate Tic Tae Toe](https://joeyrobert.org/projects/ultimatetictactoe/) using
Rust.





1. Each turn, you mark one of the small squares.
2. When you get three in a row on a small board, you’ve won that board.
3. To win the game, you need to win three small boards in a row.
3. You don’t get to pick which of the nine boards to play on. That’s determined by your opponent’s previous move. Whichever square he picks, that’s the board you must play in next.
If your opponent sends you to a board that's already been won, you must play in that board in there is a free square.
If your opponent sends you to a board that's full, you can play anywhere on the board.



combinatorial game theory
https://en.wikipedia.org/wiki/Combinatorial_game_theory


A solved game is a game whose outcome (win, lose, or draw) can be correctly
predicted from any position, assuming that both players play perfectly.

https://en.wikipedia.org/wiki/Solved_game
