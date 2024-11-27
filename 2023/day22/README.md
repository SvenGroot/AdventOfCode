# Day 22

[Puzzle description.](https://adventofcode.com/2023/day/22)

I tried to be clever with part 2, but didn't quite figure out how to handle some edge cases that
weren't in the sample input, such as a stack like this (2D sample):

```text
DDD  JJJ
B C  H I
A F  GGG
```

Removing A will not move D, as it's still supported by C and F. But removing G will move J, as both
H and I are freed by removing G. This made clever solutions that precompute the number of bricks
supported by each brick tricky.

I could probably figure out a way around that, but in the end the brute force solution (remove each
brick, then let the remaining bricks fall, count how many were moved) wasn't slow enough for me to
care (it took a minute or two).
