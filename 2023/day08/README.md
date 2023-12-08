# Day 8

[Puzzle description.](https://adventofcode.com/2023/day/8)

Part 2 was actually easier than I anticipated. I expected that just following multiple paths until
they all reached an end would be too slow, and it was, by far. However, I also expected that to do
this faster, I would have to go from every start to every end it could reach, then find a cycle in
that path, with a different number of steps at every part of the cycle. Then with such a path for
every start node, I would have to find a place where they all matched up.

That turned out to be unnecessary, as at least in my input every start node could reach only one
end node, and the number of steps to reach that end from the start was always the same as the
number of steps to reach it again continuing from that end node. That meant all I needed was the
length of start->end for each start node, and then to find the LCM of those values. And, I already
had a helper for finding the LCM of more than two values from an earlier year.
