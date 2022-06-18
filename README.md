# Conways-Game-Of-Life

## Rules
The rules for conways game of life are as follows

> - Any live cell with fewer than two live neighbours dies, as if caused by under-population.
> - Any live cell with two or three live neighbours lives on to the next generation.
> - Any live cell with more than three live neighbours dies, as if by overcrowding.
> - Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

## Build
To build this project use the following command

> cargo build

To test this project use the following command

> cargo test

### Running Tests on Save
This is a useful command when test driving the development of this kata.

Firstly install cargo-watch

> cargo install cargo-watch

Next set cargo watch to "test" and leave the terminal executing this command open

> cargo watch -cx test 

On each save the tests will be run

This tool can be set up to run other things such as build, clippy and so on e.g

> cargo watch -cx build
> cargo watch -cx clippy
