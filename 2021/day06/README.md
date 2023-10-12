# Day 6

[Puzzle description.](https://adventofcode.com/2021/day/6)

This was the first one that required a little bit of thought.

I was well aware that just keeping an actual list of fish wasn't going to scale, and that part 2
was probably going to ask me to increase the number of days, but I wrote the "naive" version anyway,
just because I wanted to.

And, I was right. Even using `u8` for each fish, the sample alone would require 26GB to store the
list, and the real input would've required around 1.6TB, since the answer was 1,687,617,803,407
fish.

Fortunately, the better solution was immediately obvious: just keep count of how many fish are at
each timer value, and adjust the counts each round. This only needs 9 `usize` values (well, 10,
including the temp value used while simulating), so it's *slightly* more efficient.
