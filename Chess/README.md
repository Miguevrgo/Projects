# Oxide Chess 

A chess game for the terminal, written in pure Rust. This project provides a simple yet efficient implementation of chess. It is designed to be played in the terminal for ease of installation and simplicity, with the goal of offering clean and accessible code as a reference for others.

The game can be played on a single computer, with two movement control options (arrow keys and Vim-like motions), against a computer, or even over a LAN between two different computers. A TCP-based client-server design has been chosen for network play.

## Showcase
![Menu](https://github.com/Miguevrgo/Projects/main/Chess/menu.jpg?raw=true)

![Game](https://github.com/Miguevrgo/Projects/main/Chess/game.jpg?raw=true)
## Board
I spent a long time considering whether to use a bitboard design. Ultimately, I decided against it in order to develop my own alternative. While it may not be as efficient, it has been space-efficient enough while allowing me to learn. However, I do not rule out implementing both approaches in the future to explore bitboard representation.

## Features
- Select pieces using `Enter`
- Move with `h`, `j`, `k`, `l` (Vim-style navigation)
- Validate moves on both client and server
- Basic chess rules implemented

## To-Do
- Castling logic
- Refactor code for better legibility and performance
- Chess engine to play against an AI
- LAN server for online multiplayer

## Installation
Make sure you have Rust installed, then clone the repository and build the project:

```sh
git clone https://github.com/miguevrgo/Projects.git
cd projects/Chess/chess 
cargo build --release

