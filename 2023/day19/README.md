# Day 19

[Puzzle description.](https://adventofcode.com/2023/day/19)

This one was interesting, for two reasons.

First, the input was slightly non-trivial to parse, so I decided to give the `nom` crate a try.
Definitely a nice way to write a parser.

Second, part 2 was essentially a completely different problem, just using the same input. I went a
bit wrong here at first, trying to come up with a unified set of accepted ranges and get the count
from that, but I don't know if that's actually possible. Once I realized I would have to limit each
rule to ranges that hadn't already been filtered out by the previous rule, this was fairly easy.
