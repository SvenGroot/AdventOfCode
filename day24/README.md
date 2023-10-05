# Day 23

[Puzzle description.](https://adventofcode.com/2022/day/24)

## Part 1

Boy this was a doozy again...

At first, I believed this was another exhaustive search, like day 16 and 19, and I tried to model it
as such. This was slow (predictably), so I had to find shortcuts.

First, I realized that the blizzard positions would repeat, which meant you could walk in circles
(get back to the same position, with all the blizzards in the same positions), which meant the
search was endless. Trying to "guess" a reasonable longest path and capping the search like that
worked for the sample, but not for the real input; it was still to slow.

I then realized that because the valley states repeated, and there weren't that many (on 599 in the
real input) I could pre-compute all of them, saving work during the search. That also meant I could
represent a state as `(valley_index, position)`, which made memoization easy. Yet, this was still
too slow, so I used to memoization to detect cycles too.

This worked, both for the sample and part 1.

## Part 2

This looked simple. Just do the same thing twice more, with different starting states. My search was
fast, so no problem, right?

For the sample, sure. For the real input, I first ran into a stack overflow with the return path.
Trying to cap the search depth to a reasonable length worked, but got the wrong answer. And the
website told me my answer was too high. How could capping the search depth lead to a path that was
too long?

It took me some time trying to debug this, before I realized my method was fundamentally flawed.
The cycle detection would hide possible paths. Consider a path `A->B->C->A`, which is obviously
a cycle (here each letter represents a combined valley state and position). The search from `C` will
therefore not consider any paths that go through `A`, and then save the result for that state.

Later, we may see state `C` again, but where state `A` was *not* a predecessor. In this case, the
memoized state is used, not considering that going `C->A` is no longer a cycle and may lead to a
shorter result for this path where we didn't come from `A`. You therefore can't memoize anything
until all the possible paths from that state are resolved as non-cycles, which (if possible at all)
would probably take too long again.

My solution was therefore broken, and only got the correct answer for part 1 by accident.

So, back to the drawing board. I realized that since I could pre-compute all the valley states with
their blizzard positions, all the possible states that can be visited consist of all the empty tiles
in each state, which is also trivial to compute. Every possible state transition is from one of
those states, to the next valley state where either the current position (if you wait) or a neighbor
is free.

That way of looking at it turns the problem into a graph, where vertices are all the possible
`(valley_state, position)` combinations, and the edges are the state transitions. Now, we could use
Dijkstra (which I already had an implementation of from an earlier day) to find the shortest path
through those states. No more recursive search, no more dealing with walking in circles; it sounded
ideal.

I got this to work for the sample pretty easily, but for the real input it was terminally slow.
While there were 653916 states (and therefore vertices in the "graph"), I hadn't been expecting
that. Using a profiler I discovered that finding the current lowest distance was taking up almost
all the time.

That was good news, because my Dijkstra implementation was pretty dumb, and just used a linear
search to find the lowest value every round. Optimizing that with a priority queue would be
straightforward. Fortunately, I not only found a good priority queue crate for Rust, it even had
a fast "change priority" operation, which was exactly what I would here!

This was still slightly slower for part 1 than the original search, but had the advantage of being
actually correct, rather than working by accident. And, it got the correct answer for part 2, in
just under 3 seconds: 798 minutes.

I'm pretty proud that I figured all this out by myself, without looking for hints or existing
solutions.

All that work, plus over 13 hours slogging back and forth through a valley full of blizzards, just
because some damn elf forgot his snacks...
