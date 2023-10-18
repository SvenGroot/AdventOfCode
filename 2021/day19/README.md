# Day 19

[Puzzle description.](https://adventofcode.com/2021/day/19)

This was definitely the hardest one so far of this year, because it didn't really fall into any
puzzle type I'd done before, and it wasn't immediately obvious what approach to take.

At first I thought I might be able to shift two scanners to the same origin and then check if any
of their beacons overlap. However, I realized that only works if they see the exact same scanners,
otherwise determining that common origin is not possible.

So, I settled on having to look for matching relative scanner positions, which would then need to
be done for every possible orientation for the scanners.

It's not very fast, but it only takes a few seconds, so it's plenty fast enough for this purpose.

At least part 2 was easy after that.
