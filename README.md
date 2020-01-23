# Advent of Code 2019

This is my implementation of [Advent of Code 2019](https://adventofcode.com/2019), in [Rust](https://www.rust-lang.org/).

You can run it with

```
cargo run --release [ day [ star ] ]
```

where `day` is between 1 and 25 and `star` can be 1 or 2 for first or second star of the day. Without parameters, all stars between day 1 and day 24 are executed.
Day 25 is an interactive adventure and must be run with
 
```
cargo run --release 25
```

Unit tests are available, based on examples from the descriptions. You can run them with

```
cargo run --release [ dayXX ]
```

where `XX` is between `01` and `25`. Without parameters, all tests are executed.

All solutions are generic, that is you can replace inputs of my session with yours and you should get the correct answers.