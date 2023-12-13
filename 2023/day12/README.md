# Day 12

[Puzzle description.](https://adventofcode.com/2023/day/12)

I did part 1 in the a very naive way, not knowing for sure where part 2 would be going. My solution
was kind of clever, I think. I treated each unknown spring state as a bit in a number, so I could
just increment that number to iterate through all the possible arrangements, and then check which
were valid.

This was obviously not an efficient approach, since it would check many states that were obviously
not going to be correct, but for part 1, it sufficed.

Part 2's gimmick was just to extend the search space by duplicating the values a bunch, so yeah,
my original solution wasn't going to work anymore, if only because plenty of rows in the real input
now had more than 64 unknown springs. Maybe 128 bits would've been enough, but it was obviously
going to be far too slow.

So I first rewrote my part 1 solution to be recursive, only considering states that could lead to
a valid solution (no dumb things like putting too many damaged springs next to each other, more than
were required for the next group). That worked, and was fine for the sample for part 2, but still
too slow for the input data.

That meant it was time for ye olde memoization. Adding that solved part 2 in a few milliseconds,
even with a debug build.
