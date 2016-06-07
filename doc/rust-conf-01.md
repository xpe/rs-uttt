# Rust Conf Proposal

## Title

Solving Ultimate Tic Tac Toe with Rust, AI, and Machine Learning

## Abstract

Rust is a great language for (attempting to) solve the game of Ultimate Tic Tac
Toe (UTTT). I used Rust because I wanted to reduce the memory layout and
increase performance while retaining high-level language features. I'll talk
about what it means to solve a game and why solving UTTT is practically
impossible using traditional techniques due to the large state space. I'll
present what I hope is a novel approach to "solve" the game incrementally using
a combination of artificial intelligence and machine learning techniques.


## Details

Solving (or trying to solve) the game of Ultimate Tic-Tac-Toe (UTTT) is a
fascinating challenge.

One could devise an algorithm that returns the set of best moves for every board
state. However, for my purposes, I will be content with an algorithm that
returns one of the best moves. Why?

* Such an algorithm is sufficient to ensure that no other algorithm could play
  better than it.
* A single-valued function is more amenable to my approach, which I will explain
  below.

What does it mean for a move to be "a best move"? It means that one has to prove
that there are no other moves that are better.

As one upper bound, there a maximum of 3 ^ 81 = 4.434e38 board states in UTTT,
since there are 9 x 9 = 81 slots, each with 3 possible values (empty, X, or
O). (However, not all of the 4.434e38 combinations are valid, because not all
can be reached by playing the game according to the rules. In the future, I hope
to generate a more accurate estimate of the number of board positions.)

Having 4.434e38 board states is large enough to make it impractical to store a
best move for each board state, even with modern distributed storage
mechanisms. The impracticality of storing such a large state space means that
one cannot simply precompute and later look up the answer. Instead, one needs an
algorithm that operates with a reasonable amount of memory.

For the game of UTTT in particular, one could, logically, deduce higher order
principles that arise from the stated rules. Two examples:

1. The game has symmetry. This could be taken advantage of to simplify the board
   state space.

2. Using logic, one can about the the rule set and realize that certain board
   positions are unwinnable.

Such emergent rules (to use the term from complexity theory) could provide
useful information to an algorithm that wants to compute the best possible
moves. However, for the purposes of this discussion, I don't want to rely on
higher-order understandings of the game rules.

Instead, I'm interested in finding an approach for solving the game even if one
only has a black box. The "black box" assumption is useful in that it helps
generalize this approach. For example, it includes:

* Situations where the rules are not known in advance.

* Situations where the rules may be known (more or less), but complicated enough
  such writing them down isn't worth it.

Solving the game for all board states has overlapping subproblems, so it is not
surprising that [dynamic programming][cs97si] is a useful approach. Using
memoization and starting with reasonable base cases,

[cs97si]: https://web.stanford.edu/class/cs97si/04-dynamic-programming.pdf

sub-problems. A good way to solve dynamic programming algorithms involves the
combination of memoization and building up from base cases. In the case of
solving UTTT, this would mean starting from board states that are "close" to
ending states; e.g. wins or ties.

For the point of argument, let us say that such an algorithm, combined with
memoization and base-case inducation, performs reasonably well in computing
individual solutions. However, to solve the entire game, it must run over the
the entire state space in order to provably solve the entire game. But we said
that it is impractical to keep such a record of correct moves for each board
position!

Does this mean that this approach unworkable in practice? No! There is a solution!

My recommended solution involves a hybrid of artificial intelligence (i.e. the
game-solving algorithm) and machine learning (i.e. building a model to compress
the knowledge learned in the game-solving algorithm).

Here are some high-level details about my approach:

1. Train a machine learning model (probably an artificial neural network)
   incrementally with results from the game-solving algorithm.

2. Keep a small fraction of the solved game states, since it is impractical to
   store all of them.

3. Carefully split the solved game data so that the machine learning model has
   to both learn information without forgetting previous results.

4. If the machine learning model performs well, over time, that builds
   confidence that it has "solved" the game!

I have not heard about nor read about such an approach previously.

## Pitch

## Bio

David is a part software developer and entrepreneur. He is driven to find
creative solutions to advance technologies, processes, and cultures to support
compelling products and missions. He often uses diverse quantitative models to
frame decisions, clarify thinking, and ground discussion on key decisions.
