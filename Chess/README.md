# Oxide Chess 

A chess game for the terminal, written in pure Rust. This project provides a simple yet efficient implementation of chess. It is designed to be played in the terminal for ease of installation and simplicity, with the goal of offering clean and accessible code as a reference for others.


## Board
I spent a long time considering whether to use a bitboard design. Ultimately, I decided against it in order to develop my own alternative. While it may not be as efficient, it has been space-efficient enough while allowing me to learn. Later on, the project became a challenge from my teacher and I decided to use bitboard.

## Features
- Select pieces using `Enter`
- Move with `h`, `j`, `k`, `l` (Vim-style navigation)
- Validate moves
- Perft tests passed (Tested with depth 8)

## To-Do
- Chess engine to play against an AI
- LAN server for online multiplayer

## Installation
Make sure you have Rust installed, then clone the repository and build the project:

```sh
git clone https://github.com/miguevrgo/Projects.git
cd projects/Chess/chess 
cargo build --release
```

## Using
It can be played simply by running
```
cargo run
```
To check perft, you can navigate into src/chess/perft.rs and you will find the tests on the bottom, feel free to increase depth as you wish, current depth level is used in order to quickly check correctness.
```
cargo test --release -- --nocapture
```
