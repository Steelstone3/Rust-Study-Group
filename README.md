# Rusty-Mars-Rover

## Rules
The rules for mars rover are as follows

> - The following interface is provided within a class (struct) of MarsRover String execute(String commands)
> - With this interface there are three commands that can be passed in 'M', 'L' and 'R'
> - M moves the mars rover in the current direction
> - L turns the mars rover to the next leftward cardinal point
> - R turns the mars rover to the next rightward cardinal point
> - Once each of the commands have been executed a final position and direction is returned
> - The starting state of the mars rover is 0:0:N

### Examples
The mars rover moves forward 3 times ("MMM") with the starting coordinates of 0:0:N

> The result returned would be 0:3:N as the Y coordinate will have increased by 3

The mars rover turns left 3 times ("LLL") with the starting coordinates of 0:0:N

> The result returned would be 0:0:E as the cardinal direction would have changed 3 times from North to West then South then East

The mars rover travels west 5 times ("LMMMMM") with the starting coordinate of 0:0:N

> The result returned would be -5:0:W as the X coordinate will have decreased by 5 and the cardinal direction changed from North to West

#### Acceptance Test

> given an input MMRMMLM then the output should be 2:3:N

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
