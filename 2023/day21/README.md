# Day 21

[Puzzle description.](https://adventofcode.com/2023/day/21)

The actual solution to part 2 here was beyond me, as it was too much math. I adapted my algorithm
to work with the "infinite" map, but as expected, it was much too slow to do the required number
of steps.

I figured there had to be some kind of pattern to the result, but looking for simple patterns like
"how often does it double" didn't work. The solution was actually to fit the result to a polynomial,
which I don't really know how to do. An algorithm called LaGrange Interpolation can do it, but I
couldn't find a good Rust implementation, and didn't want to write one myself.

Finding the solution depends on realizing that the middle row and column (the one with the start
position) is empty, and as a result it takes 65 steps (half the map size) before you reach the first
repeats of the map, and after that it takes 131 steps every time before you reach the next set of
repeats. Then you have to realize that the number of steps 26501365 == 202300 * 131 + 65. This fact
can be used to find a sequence which can be interpolated to find the final value.

What's extra annoying here is that none of this is true for the sample input, so it can't be used
to solve the puzzle, unless you go with a more complicated and more generic approach.

I found someone who mentioned that the algorithm from  day 9 could interpolate values in a sequence,
and that it can in fact be used here. That sounded at least interesting to implement, so I adapted
the algorithm from day 9 for use here. Mainly, I had to change it so it could be used to repeatedly
extend the same sequence without recomputing the diffs each time.

So no, I didn't solve this, really. But I'm hoping to do the remaining days before the next AoC
starts, so I can't spend too much time on them.
