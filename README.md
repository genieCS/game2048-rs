# 2048 Game using Cursive

![2048](https://user-images.githubusercontent.com/35099832/234502901-e1721cd9-eb64-4c52-84ac-ba0c5d5cd3e3.gif)


This is a Rust implementation of the popular 2048 game using the [Cursive](https://github.com/gyscos/cursive) library for building terminal user interfaces. The game is played on a 4x4 grid where the player combines tiles with the same number to create a tile with a larger number. The goal is to create a tile with the number 2048.

## Installation

To install the game, if you don't have Rust and Cargo installed on your system, you can try it using GitHub Codespace. 
If you have Rust and Cargo installed, you can clone the repository and run the following command:

`cargo run --release
`

This will build and run the game in release mode.

Or you can download the crate from crates.io with following command:

`cargo install game2048-rs`


## How to Play

The game is played using the arrow keys on your keyboard. You can move the tiles left, right, up, or down. When two tiles with the same number touch, they merge into one tile with the sum of their numbers. The game is won when a tile with the number 2048 is created. The game is over when the board is full and no more moves can be made.

## Features

- Terminal user interface using Cursive
- Random tile generation
- Score tracking
- Game over detection
- Game won detection

## Code Structure

The main game logic is implemented in the `board.rs` file. The `Board` struct represents the game board and contains methods for moving tiles, generating new tiles, and detecting game over and game won conditions. The `board` in `game.rs` is responsible for rendering the game board in the terminal using Cursive.

## Contributing

Contributions are welcome! If you find a bug or have an idea for a new feature, please open an issue on the [project](https://github.com/users/genieCS/projects/1/views/1) or submit a pull request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
