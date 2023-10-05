# Day 19

This was another optimization problem, like day 16, but not quite as hard. I was able to make it
work using a straight-forward DFS, with only minimal pruning.

I originally made this way too complicated because I didn't realize you can only build one robot per
minute, and I tried to make the "don't build a robot" path recursion-free, which just complicated
the code. Once I realized my mistake with the robot factory, I rewrote a simpler version which
actually terminated in reasonable time.

The only optimization I did was to only build robots if you couldn't have built them earlier. There
is, after all, never a reason to wait a turn and then build a robot if you could've already built
that same robot on the first turn.

This worked well for part 1, which finishes quickly. Part 2 is still somewhat slow, but by running
each blueprint in its own thread it finished in 48 seconds, which is certainly good enough. There
are a bunch of other optimizations possible, including further pruning of "always bad" moves (like
building more ore robots than the most expensive robot's ore cost), or memoization, but it wasn't
needed.
