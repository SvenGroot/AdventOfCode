# Day 18

[Puzzle description.](https://adventofcode.com/2023/day/18)

I have to give this puzzle props for obscuring what part 2 was going to be. I originally went with a
naive approach for part 1, drawing the shape on a grid and counting the inside tiles. It was slow,
but okay for part 1.

I thought this might be okay because I was expecting part 2 to be color related, but it turns out
the "color" was just a red herring, and it actually just hid instructions for a much larger shape.
With the proper values decoded from the "color" field, the trench was now far too large to measure
it using the naive approach.

So, I looked online for an algorithm to calculate the area of an irregular polygon. I found one, but
it didn't quite work; the result it gave was too small. I tried adding the circumference, but that
was too big. On a hunch, I tried adding half the circumference, which was off-by-one. After fixing
that, the algorithm worked for both part 1 and 2.

I'm not sure why I needed to make those changes; I just did it by trial and error until it worked.
But, I got the right answer, so we're moving on!
