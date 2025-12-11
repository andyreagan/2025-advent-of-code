# 2025 Advent of Code

My solutions in Rust.

## Solutions

| Problem | Part A | Part B |
|---------|--------|--------|
| [Day 1](https://adventofcode.com/2025/day/1) | [164.42 μs](day01a/src/main.rs) | [172.62 μs](day01b/src/main.rs) |
| [Day 2](https://adventofcode.com/2025/day/2) | [41.17 ms](day02a/src/main.rs) | [200.58 ms](day02b/src/main.rs) |
| [Day 3](https://adventofcode.com/2025/day/3) | [102.50 ms](day03a/src/main.rs) | [604.00 μs](day03b/src/main.rs) |
| [Day 4](https://adventofcode.com/2025/day/4) | [73.54 μs](day04a/src/main.rs) | [1.17 ms](day04b/src/main.rs) |
| [Day 5](https://adventofcode.com/2025/day/5) | [132.25 μs](day05a/src/main.rs) | [64.25 μs](day05b/src/main.rs) |
| [Day 6](https://adventofcode.com/2025/day/6) | [82.00 μs](day06a/src/main.rs) | [468.58 μs](day06b/src/main.rs) |
| [Day 7](https://adventofcode.com/2025/day/7) | [78.00 μs](day07a/src/main.rs) | [80.25 μs](day07b/src/main.rs) |
| [Day 8](https://adventofcode.com/2025/day/8) | [41.24 ms](day08a/src/main.rs) | [54.17 ms](day08b/src/main.rs) |
| [Day 9](https://adventofcode.com/2025/day/9) | [119.12 μs](day09a/src/main.rs) | [316.48 ms](day09b/src/main.rs) |

**Total runtime: 707.49 ms**

## Inputs

For a full run (benchmarking) you need the inputs.
I have them as a private submodule,
so just

```
git clone --recurse-submodules git@github.com:andyreagan/advent-of-code.git
```

Puzzle inputs go in `inputs/day01.txt`, `inputs/day02.txt`, etc.

## Testing

Run the tests for each individual day & part by changing to that project first,
and running tests.

```
cd day01a
cargo test
```

If you want to feel how fast we are on the full data, build the release version

```
cargo run --release
```

## Benchmarking

Run the benchmark with

```
cd runner && cargo run --release --bin bench
```

## All years

- [2025](#) (Current)
- [2022](https://github.com/andyreagan/2022-advent-of-code)
- [2018](https://github.com/andyreagan/2018-advent-of-code) 
