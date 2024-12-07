# Day 3

[Puzzle description.](https://adventofcode.com/2024/day/3)

Yeah, I'm already behind. I'm just too busy this month. We'll see how far we get this time.

I wanted to use nom for this puzzle, but I couldn't find a good solution to skipping an arbitrary
number of characters. Maybe `take_until` would work but it seems like that would evaluate the
pattern twice? I'm not sure.

Oh well, doing it with regex was easy.
