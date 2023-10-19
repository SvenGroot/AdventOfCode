# Day 22

[Puzzle description.](https://adventofcode.com/2021/day/22)

Another tough one. At first it seemed simple: just create a set of every cube that's on, right? But
the steps that were not in the "initialization area" defined very large cubes, and since those were
excluded from part 1, it was clear that this naive approach wasn't going to scale for part 2.

Still, I wrote part 1 using the naive approach, and I'm kind of glad I did. It gave me some info I
needed to debug part 2.

For part 2, I came up with the following approach

- Keep a list of "on" cuboids.
- When processing a step, for each cuboid in the list.
  - Compute the difference between the cuboid, and the new cuboid from the step. This breaks the
    existing cuboid into multiple cuboids that cover only the areas where the cuboid didn't overlap.
  - Create a new list of cuboids using the split pieces.
- If the step turns on the cubes, add the step's cuboid to the list.

The hardest part here was writing an algorithm that computes the difference between cuboids, and
getting that to work right for all edge cases. This is where knowing what number of cubes should be
on after each step from the naive approach in part 1 helped; by comparing that, I could tell at
which step the new algorithm was failing.

After a bunch of iterations of finding and fixing bugs, it was working. I have on idea if there's
a better way to do this, maybe one that's easier... but it doesn't matter. I got the answer anyway.

This is definitely not the kind of thing I have to solve in my day job, so it was an interesting
challenge.
