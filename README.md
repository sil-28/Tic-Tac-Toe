# Tic Tac Toe (Rust CLI)

A simple two-player Tic Tac Toe game written in Rust, played from the terminal. Built as a learning project.

## Features

- Two human players, playing locally on the same terminal
- Players are asked for their names at the start of the game
- 3x3 board displayed after every move
- Detects a win (row, column, or diagonal) and announces the winner
- Detects a draw when the board is full with no winner
- Input validation: invalid or already-occupied cells don't crash the program

## How it works

The board is represented internally as 9 positions (1-9), read left to right, top to bottom:

```
 1 | 2 | 3
---+---+---
 4 | 5 | 6
---+---+---
 7 | 8 | 9
```

Players take turns entering a position to place their mark (X or O). The game checks all 8 possible winning combinations (3 rows, 3 columns, 2 diagonals) after each move.

## Requirements

- [Rust and Cargo](https://www.rust-lang.org/tools/install)

## Running the project

Clone the repository and run it with Cargo:

```bash
git clone https://github.com/sil-28/TicTacToe.git
cd TicTacToe
cargo run
```

You'll be asked for both players' names, then the game starts with Player 1 as X and Player 2 as O.

## What I learned

This project was built while learning Rust, focusing on:

- Enums to represent a fixed set of states (`Cell::Empty`, `Cell::X`, `Cell::O`)
- Deriving traits (`Clone`, `Copy`, `PartialEq`) to make custom types easier to work with
- Arrays and constants (`const`) for fixed data known at compile time
- Pattern matching (`match`) and `Option` for representing "no winner yet"
- Input validation and parsing user input safely

## Possible improvements

- Add a single-player mode against a computer opponent (starting with random moves, then a minimax algorithm for an unbeatable AI)
- Keep score across multiple rounds
- Support different board sizes