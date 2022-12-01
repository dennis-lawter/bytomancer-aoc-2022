# Bytomancer's Advent Of Code 2022 Solutions

## SPOILERS AHEAD

By reading further, you will face spoilers for the Advent of Code 2022.
If you wish to try the event for yourself,
please visit https://www.adventofcode.com/

## About

I've decided to tackle the Advent of Code for 2022 as a means of learning Rust.
I don't claim these to be particularly elegant or efficient solutions,
I'm simply trying to expose myself to a new language.

I'm open to any feedback or criticism.
Rust has been a difficult language to learn,
as I've found it to be quite different from my more comfortable languages
(Primarily C++, PHP, and Python).
As I am still learning Rust,
I'm certain I've made several beginner mistakes I'd be happy to learn from.

## AOC Framework

This project began in November 2022,
as I worked on solving the AOC 2021 problems.
From my time tinkering with these problems,
I decided to add on a few features to ease development
(and for the simple fun of it).

### Features

1. I've implemented an input downloader which retrieves input files via the web.
   - A `.env` file is required with `SESSION=<Session ID from your cookie>`.
   - Files are downloaded to a `_cache/` folder created in the project root.
   - If an input file is already found locally, that file is loaded instead.
2. Arguments dictate the solution to be run.
   - After discovering significant re-use between the days,
     I decided to package my code together in a single project.
   - Execution is performed with `cargo run -- dXsY`,
     representing day X solution Y.
3. Colorization is used heavily.
   - Tracking outputs and debugging is much simpler,
     thanks to the `colored` crate.

## Personal Leaderboard

```
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
  1   00:15:44  7176      0   00:23:06  7379      0
```
