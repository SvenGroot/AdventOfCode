# Day 5

[Puzzle description.](https://adventofcode.com/2023/day/5)

Ah, the first "the straight-forward approach is too slow" puzzle of the year. Still, not slow enough
to warrant further optimization. Even without threading part 2 took only about a minute, with the
threads it's about 20 seconds (it's only 3x due to the imbalance between the threads).

The optimal way to do this, I would guess, would be to first fold all the maps into a single map
that goes straight from seed to location, and then you only need to consider the minimum value in
each map range that a seed range overlaps with. However, with the correct answer already found, it's
not worth attempting at this point.
