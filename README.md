# ![NullTek Documentation](https://github.com/CreatingNull/NullTek-Assets/blob/main/img/logo/NullTekDocumentationLogo.png?raw=true) Advent of Code 2024

[![Language](https://img.shields.io/badge/rust-1.88-red.svg?style=flat-square&logo=rust&logoColor=white)](https://blog.rust-lang.org/2025/06/26/Rust-1.88.0/)
[![License](https://img.shields.io/:license-mit-blue.svg?style=flat-square&color=orange)](LICENSE.md)
[![Stars](https://img.shields.io/badge/progress-0%20stars-000000.svg?logo=star&style=flat-square&color=yellow)](https://adventofcode.com/2021)

Very late post-release attempt at advent of code 2024 in Rust.

I am teaching myself Rust, so these are **not** ideal or reference solutions by any stretch of the imagination.

## Progress

| Day | Challenge                         | Status                             |
| --- | --------------------------------- | ---------------------------------- |
| 1   | --- Day 1: Historian Hysteria --- | :star: :star: [Complete](src/day1) |

## Personal Rules

This is a very casual attempt, I'll just be trying random tasks as I get enthusiastic.

1. This is about having fun, challenging myself and learning new things.
2. I will almost definitely skip days, and am encouraging myself to do it when it's impacting my sanity (See Rule #1).
3. I can use what ever tools I want within rust. This is a learning experience around the whole ecosystem for me.
4. I will not look at any reference solutions to the tasks.
5. I can research ways of efficiently solving general problems required by the task provided it doesn't conflict with Rule #4.

## Structure

### Source Code and Test Data

The project is structured with each days solutions being placed in modules in the root crate, `src\dayxx`.
Test data is included in the same directory:

- \`example.txt" - Example data from the question for verifying solutions with (usually just during development).
- `data.txt` - The data I was provided for the task.

There is also a helper crate `aoc-lib`, which just contains some general functionality for loading data, and provides macros for pretty displays during testing.

**Bundled unit-tests**

Each module contains `rstest` inline parametrized unit tests for verifying the results of the solutions against my verified results.
These can be called with `cargo test --all --`, you can add `--nocapture --test-threads=1` if you to display results for the solutions.

### CLI Runner

The crate also builds to a CLI interface for calling the solutions on arbitrary data input (note the files must be named and formatted the same as ones in my source.).

```shell
Advent of Code Runner

Usage: aoc-2024.exe --day <DAY> --part <PART> --data <DATA>

Options:
  -d, --day <DAY>    Which day to run
  -p, --part <PART>  Which part to run
  -d, --data <DATA>  Path to the input file
  -h, --help         Print help
  -V, --version      Print version
```

Example:

```shell
aoc-2024.exe --day=1 --part=1 --data=src/day01/data.txt
🧩 Output: 1319616
```

## License

The source of this repo uses the MIT open-source license, for details on the current licensing see LICENSE.md or click the badge above.

- Copyright 2025 © <a href="https://nulltek.xyz" target="_blank">NullTek</a>.
