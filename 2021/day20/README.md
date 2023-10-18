# Day 20

[Puzzle description.](https://adventofcode.com/2021/day/20)

Not too bad. The only thing that made this interesting was the fact that the image was "infinite",
and that was only interesting because in my actual input, the value at index 0 of the enhancement
algorithm was `#`.

That meant that on every odd iteration, all the infinite pixels would get turned on. And since the
last value in the enhancement algorithm was `.`, on even iterations they all got turned off again.

So it wasn't enough to just extend the grid by some guessed amount, I had to either manually reset
the edges, or keep track of the value for pixels beyond the grid, which is what I went with.

Unfortunately, nothing interesting gets drawn here, so there's no point in sharing the resulting
grid here.
