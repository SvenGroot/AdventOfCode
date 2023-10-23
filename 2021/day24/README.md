# Day 24

[Puzzle description.](https://adventofcode.com/2021/day/24)

I didn't like this one. I don't think I've seen a puzzle before where you need to manually study the
input to look for a pattern to exploit in the solution. Usually, you just need to look at the puzzle
description. The lack of example input also meant I had no known solution to compare against.

Because of that, I had no clear idea how to approach this. I built an interpreter for the ALU under
the assumption we'd need it, which was pretty easy. But then what?

Trying all codes wasn't going to work, that's far too slow.

I tried looking for a pattern of input -> z that I could exploit, but I couldn't find one.

Finally I started looking at the program. I realized it had blocks of "read a digit, do something."
I then tried to work backwards, doing an exhaustive search of z + digit values that would lead to
z being zero in the end. Again, way too slow, even with memoization.

Defeated, I looked for help online. There, someone pointed out that the blocks in the program would
pair up: one that multiplies by 26, and one that divides. I'd noticed that pattern, but hadn't made
the connection that they'd pair up where the second had to undo the first.

Once I got that, it was trivial. But man, did I waste a lot of time before I got there.
