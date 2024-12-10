# 2024-advent-of-code

My solutions to [Advent of Code](https://adventofcode.com/) 2024 in Rust

## Input

My personal inputs are not included in this repo per https://adventofcode.com/2024/about.

Currently, the expected location of the inputs are: `./input/dayNN.txt` and `./input/dayNN_test.txt`
e.g. `./input/day02.txt` and `./input/day02_test.txt`

Place your input(s) in that directory, with the same naming convention to have this repo work as-is for your problem inputs.

## Usage

Run the program from the command line without any arguments to solve every problem so far, e.g.

```
cargo run
```

```
===================
Advent of Code 2024

=== Day 01 ===
 -- Part 1: 1834060 (took: 0.002s)
 -- Part 2: 21607792 (took: 0.017s)

=== Day 02 ===
 -- Part 1: 591 (took: 0.003s)
 -- Part 2: 621 (took: 0.005s)

=== Day 03 ===
 -- Part 1: 155955228 (took: 0.025s)
 -- Part 2: 100189366 (took: 0.118s)

=== Day 04 ===
 -- Part 1: 2567 (took: 0.004s)
 -- Part 2: 2029 (took: 0.002s)

=== Day 05 ===
 -- Part 1: 4905 (took: 0.023s)
 -- Part 2: 6204 (took: 0.206s)

=== Day 06 ===
 -- Part 1: 5199 (took: 0.001s)
 -- Part 2: 1915 (took: 1.946s)

=== Day 07 ===
 -- Part 1: 5540634308362 (took: 0.052s)
 -- Part 2: 472290821152397 (took: 1.162s)

=== Day 08 ===
 -- Part 1: 413 (took: 0.002s)
 -- Part 2: 1417 (took: 0.002s)

=== Day 09 ===
 -- Part 1: 6241633730082 (took: 0.035s)
 -- Part 2: 6265268809555 (took: 0.421s)

=== Day 10 ===
 -- Part 1: 459 (took: 0.003s)
 -- Part 2: 1034 (took: 0.001s)

          ★
         ★★★
          ★
         *✦*
        ❅***●
       ✦******
      ✦****◆***
     **◆◆o*****o
    ***o*●******o
   ****❅**❅**o❅***
  ***✦**o*****◆❅***
 *****❅●**o*✦*****✦*
          |
          |
         ===
```

### Arguments

Pass arguments when running the program to change the output:

- `--day <n>` - specify a single day to run
- `--tree-size <n>` - specify the size of the tree, defaults to sizing based off of the day
- `--no-festive` - do not print the ASCII tree

e.g.

Run a single day

```
cargo run -- --day 5
```

```
===================
Advent of Code 2024

=== Day 05 ===
 -- Part 1: 4905 (took: 0.031s)
 -- Part 2: 6204 (took: 0.209s)

     ★
     *
    *◆*
   ***◆*
  **❅***●
 *✦**o****
     |
    ===
```

Run a single day, specifying tree size

```
cargo run -- --day 5 --tree-size 10
```

```
===================
Advent of Code 2024

=== Day 05 ===
 -- Part 1: 4905 (took: 0.027s)
 -- Part 2: 6204 (took: 0.208s)

          ★
         ★★★
          ★
         ◆**
        *◆***
       o*****✦
      *◆***◆o**
     ◆**●*❅**●**
    ******●*●o●**
   *********✦o**●*
  ***◆****❅*o●*◆**✦
 **●◆*******✦*******
          |
          |
         ===
```

Run a single day, without printing the ASCII tree

```
cargo run -- --day 5 --no-festive
```

```
===================
Advent of Code 2024

=== Day 05 ===
 -- Part 1: 4905 (took: 0.033s)
 -- Part 2: 6204 (took: 0.209s)
```
