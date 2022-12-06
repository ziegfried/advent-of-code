# AOC CLI

Simple CLI to create new Rust projects for AOC problems and to download the input file.

## Install

Make sure `~/.cargo/bin` is in your `$PATH`, then

```sh-session
$ cd tools/aoc-cli
$ cargo install --path .
```

## Usage

### Setup

The CLI needs a session key for adventofcode.com to download a puzzle input.
First you can use `aoc auth` to store the session key in a config file.

```sh-session
$ aoc auth
```

### Create a new project

To create a new project, run `aoc new`. It will prompt for year and day
and will create the directory structure. Make sure you execute this command
from the working directory where you want to store the project.

```sh-session
$ aoc new
```

### Download puzzle input

Run `aoc input` in a project directory to download the puzzle input.
It will store it in `src/input.txt` by default.

```sh-session
$ aoc input
```
