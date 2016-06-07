# Solving the Game

## Background

Lindsay and I were working on a Clojure version of Ultimate Tic-Tac-Toe
(UTTT). I noticed that it was not particularly efficient with memory. That
prompted me to discuss the Java memory layout with her. This led to discussions
about garbage collection and so on. For some reason, I remembered that Rust had
a different model for handling memory allocation. So I started to learn Rust.

## Goal

The goal is to solve the game of Ultimate Tic-Tac-Toe (UTTT).

To solve the game, one could use an algorithm that returns the set of best moves
for every board state.

However, for my purposes, I will be content with an algorithm that returns one
of the best moves. Why?

* Such an algorithm is sufficient to ensure that no other algorithm could play
  better than it.
* A single-valued function is more amenable to my approach, which I will explain
  below.

I can see one caveat: although an algorithm that returns only one of a set of
best possible moves may be "optimal" in terms of the ultimate result, it means
that an opponent could take advantage of the algorithm's "bias"
(i.e. simplication of the solution space). In particular, an opponent might
learn time which of the "best moves" the algorithm picks. Even though this may
not give the opponent an advantage in terms of winning or tieing (given
unlimited time), it may allow the opponent to compute more efficiently. As a
result, this could reduce the opponents power demands or allow it to compute an
optimal result more quickly.

## Other Possible Goals

For each board position, it could be interesting to annotate each valid move
with additional information:

* is the move optimal or not?
* if the move is optimal, and the opponent is optimal, how many turns until the
  game is ended and what will be the conclusion?
* if the move is non-optimal, and the opponent is optimal, how many turns until
  the game is ended and what will be the conclusion?

The last bit of information could be useful in constructing an AI player that
has calibrated levels of skill. Such an AI could intentionally avoid the best
moves and prefer sub-optimal moves.

## Meaning of "A Best Move"

What does it mean for a move to be "a best move"? It means that one has to prove
that there are no other moves that are better.

What do we mean by "better"? Let's assume it is X's turn and he is choosing
between moves A and B. Move A is better than B if one of the following is true:

* A leads to a certain victory while B does not.
* A leads to a certain victory in N turns while B leads to a victory in more
  than N turns.
* A leads to a certain tie while B does not lead to a tie (or better).
* A leads to a certain tie in N turns while B leads to a tie in more than N
  turns.
* A leads to a loss in N turns while B leads to a loss in fewer than N turns.

It may be useful to make some comments about the above definitions:

* X chooses moves on the assumption that O is optimal.
* X is most interested in an "inevitable win", even if such a win takes more
  moves than might be possible with a "sneaky" or "quick" win.
* Put another way, X is "uninterested" in "risky" moves that have some
  probability of a quicker win against weaker or careless opponents.
* Even if X is in an "unwinnable" position, X will continue playing
  optimally. This is useful in case O makes a mistake.

Note: it could be useful to compute if a position is unwinnable against an
optimal opponent. This could allow an AI player to resign, perhaps, or allocate
more computation to games where it has a chance of winning.

## Number of Board States

As one upper bound, there a maximum of 3 ^ 81 = 4.434e38 board states in
UTTT, since there are 9 x 9 = 81 slots, each with 3 possible values (empty, X,
or O).

However, not all of the 4.434e38 combinations are valid, because not all can be
reached by playing the game according to the rules.

At this time, I have not worked out a more accurate estimate of the valid board
states. In the future, I may use empirical methods to better estimate this
number.

## Storage

Having 4.434e38 board states is large enough to make it impractical to store a
best move for each board state, even with modern distributed storage mechanisms,
such as Amazon S3. (This assumes that one finds a suitable mechanism to
precompute each result.)

To sketch out the details, one needs at least 6 bits to represent a move, since
there are 81 possibilities. In practice, on modern systems, a move would
conveniently be stored as one byte. So, how big is 4.434e38 bytes?

* 4.330e35 KB
* 4.229e32 MB
* 4.130e29 GB
* 4.033e26 TB
* 3.938e23 PB
* 3.846e20 EB
* 3.756+17 ZB

If one qualifies for Amazon's S3 pricing of $0.0280 per GB per month, this would
cost $1.156e28 per month!

## A Space-Bounded Algorithm

The impracticality of storing such a large state space means that one cannot
simply precompute and later look up the answer.

Instead, one needs an algorithm that operates with a reasonable amount of
memory. (For example, my personal laptop has 16 GB of RAM. Personally, I would
mind a program using 8 GB of it for a long period of time.)

## Higher Order Rules

For the game of UTTT in particular, one could, logically, deduce higher order
principles that arise from the stated rules. For example, using logic, one can
reason through the rule set and realize that certain board positions are
unwinnable unless certain sub-boards can be won.

Such *emergent* rules (to use the term from complexity theory) could provide
useful information to an algorithm that wants to compute the best possible
moves.

## A Black Box

However, for the purposes of this discussion, I don't want to rely on
higher-order understandings of the game rules. Instead, I'm interested in
finding an approach for solving the game even if one only has a black box. The
"black box" assumption applies to these kinds of situations:

* Situations where the rules are not known in advance.

* Situations where the rules are known, in some sense, but complicated enough
  such writing them down isn't worth it.

(Note: my expectation is that the black box is deterministic; I do not know if a
"black box" game with stochastic behavior is solvable with this approach -- or
any other approach.)

## A Dynamic Programming Algorithm

For now, without proof, I will claim that the game-solving algorithm falls is a
dynamic programming algorithms. This means that it has overlapping
sub-problems. A good way to solve dynamic programming algorithms involves the
combination of memoization and building up from base cases. In the case of
solving UTTT, this would mean starting from board states that are "close" to
ending states; e.g. wins or ties.

## A Problem?

For the point of argument, let us say that such an algorithm, combined with
memoization and base-case inducation, performs reasonably well in computing
individual solutions. However, to solve the entire game, it must run over the
the entire state space in order to provably solve the entire game. But we said
that it is impractical to keep such a record of correct moves for each board
position!

Does this mean that this approach unworkable in practice? No! There is a solution!

## A Solution

My recommended solution involves a hybrid of artificial intelligence (i.e. the
game-solving algorithm) and machine learning (i.e. building a model to compress
the knowledge learned in the game-solving algorithm).

I have not read about such an approach previously.

Here are some high-level details about my approach:

* Train a machine learning model (probably an artificial neural network)
  incrementally with results from the game-solving algorithm.
* Keep a small fraction of the solved game states, since it is impractical to
  store all of it.
* Carefully split the solved game data so that the machine learning model
  has to both learn information without forgetting previous results.

One key advantage of this approach is that it takes advantage of the machine
learning model to compress the game-solver.

TODO: Write a bit more here?

### Machine Learning Model

I will start with an artificial neural network (ANN) for the machine learning
model. An ANN may be a good choice because:

* An ANN can learn general bit-level patterns, which correspond to the game-solving
  data
* Solving a board position arguably will benefit from "spatial" reasoning; this
  intuitively feels like a good match for an ANN.
* An ANN can be tuned (i.e. the number of layers) so that it strikes a balance
  between performance and memory requirements.
* An ANN can be trained incrementally.

### Game-Solving Algorithm

Like I mentioned above, to overcome the challenges of working with an expensive
dynamic programming algorithm, it is beneficial to memoize previous results and
start from base cases.

Although the system cannot memoize all previous cases, it can start with board
positions that are "close" to base cases; i.e. board positions close to
game-ending states.

How does one find the set of all ended games? Doing this would be a
combinatorically hard problem! I don't have an estimate for the number of ways
to win.

Perhaps, with considerable thinking about the game rulles, one could find a
computationally feasible way of generating valid end states. However, my goal
here is to not have to reason about the rules. I want to keep the rules as a
black box.

So, I needed to find a different way. (This wasn't hard to figure out, actually;
when all else fails, use randomization!) Let me start by explaining a simple way
to do this. This is not the "final" result, but it is a useful way to explain it
for now.

Start with an empty board and proceed forward with random moves, tracking the
progression of game states. When a game end is reached, go back two moves and
send that board position to the game solver.

TODO: Write a bit more here?
