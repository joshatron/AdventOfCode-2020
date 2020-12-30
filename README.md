Advent of Code 2020
===================

As I did last year, I used Advent of Code to learn a new language.
This year I chose Rust.
Because I was learning Rust, I chose to try and write good, clean, and fast code rather than find the solution as fast as possible.
I created benchmarks to give me some idea of algorithm speed, and worked to improve times of slow solutions.
I managed to find a solution on the day each problem came out, but sometimes went back to previous days to optimize on time or readability.
I also tried mostly programming using TDD so the code is pretty well covered.

One thing I enjoyed about the challenges this year was is picking the best data structure for each problem.
There were several that I began with a Vec, which had to be searched through and made the algorithm too slow, so I would change to a HashMap or HashSet.
For some solutions though, I went back to the Vec, using it like a HashMap where the index was the key and the value stored was the value.
A good example of this was day 15, which required optimizing a loop that ran 30,000,000 times.

Overall, I really enjoyed programming in Rust.
The ownership and lifetime rules are sometimes difficult to wrap my head around, but one I started understanding them, I could see how powerful they are.
I also really enjoyed how helpful the compiler and online documentation was.
Finally, I learned about halfway through the puzzles how much faster a release build can be.

My final time to run all the puzzles is 1.5 seconds on a mid range desktop computer.