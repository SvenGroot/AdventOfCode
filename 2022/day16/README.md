# Day 16

Definitely the hardest one so far.

For part 1 I went the wrong way for a bit, while this is clearly a version of the traveling salesman
problem, I thought maybe I could cheese it by just always going to the next valve that would be give
me the best flow rate for the time it would take to get there, at that particular step. As it turns
out, that's a reasonable approximation for the correct answer (it was off by 2 for the sample), but
not good enough, as I might've expected.

So, I realized an exhaustive search was needed. I thought it might not be too bad, with a relatively
small graph and a maximum recursion depth of 30, so I started with the naive approach of recursively
walking the graph, minute by minute, treating "opening" and "moving" as separate steps for the
recursion. This worked sort of okay for the sample (took about a minute), but for my actual input
I gave up after five minutes.

I realized that there were a lot of valves with flow rate zero in my input, so I eliminated them,
creating a weighted graph between only the valves with non-zero rates (and the start, which is
zero). That improved things: I now got the correct answer for part one in about 2.5 minutes.

Thinking that part 2 would be more computationally expensive, I wasn't satisfied. I decided to use
my Dijkstra implementation from part 12 to pre-compute the distances between each pair of nodes, so
when choosing a next step I could directly go to one of the closed valves rather than randomly
walking until you find one. That worked great: down to 0.2s.

My mind didn't really want to wrap itself around part 2, which was the same problem but with two
players. I figured I could use a recursive approach, going again minute-by-minute, keeping track of
which nodes each player was going to. This would probably have worked, but I felt it was
complicated, and after spending so much time on part 1 I didn't want to write it.

I'll admit, I looked for some hints on a better approach, and soon found that since the person and
the elephant both visit distinct sets of valves, I could just compute the best possible flow for
each path with any number of valves, then find the best combination of two paths whose valves were
entirely distinct. This works, and gets the the answer in under a second.

To do this, I converted my valve naming scheme to a bitmap, to make it easy to keep track of which
valves were opened in each path. I could probably update the whole algorithm to utilize that bitmap,
rather than the maps I am using, but that's beyond what I'm willing to do at this point.
