# Day 14

[Puzzle description.](https://adventofcode.com/2021/day/14)

When I saw part 1, I just knew that part 2 was going to be the same thing with more steps. With the
speed at which the polymer grows, I knew straight away that it wasn't going to fit in memory if I
tried to build it.

My approach was to take each insertion rule and indicate which element it would add, and which two
rules should be followed next, based on the two new pairs created. We could then recursive apply
the rules, keeping a cumulative count of elements, without needing to store the polymer string at
all. For part 1, this worked just fine.

When I saw part 2, I realized I'd been right, and while my existing approach would make sure the
memory requirements were low, it would still take far too long. The solution, as always, was to
store visited states.

In this case, I indexed the state by current pair and remaining steps. Every time we visit a pair,
if it wasn't in the state already, I would add how many elements it added with that number of
remaining steps.

Technically, this still duplicates some work, as we could actually compute and store the value for
the current pair, and 1 up to the remaining steps, if we wanted to. However, that turned out to be
unnecessary since the naive memoization is already pretty much instant.
