# Chess Terminal Game

A chess game in the terminal, written in Rust. This project provides a simple interface for playing chess using keyboard controls. It validates moves on both client and server sides, ensuring legality.

## Features
- Select pieces using `Enter`
- Move with `h`, `j`, `k`, `l` (Vim-style navigation)
- Validate moves on both client and server
- Basic chess rules implemented

## To-Do
- Implement "en passant" capture
- Castling logic
- Checkmate detection
- Chess engine to play against an AI
- LAN server for online multiplayer

## Installation
Make sure you have Rust installed, then clone the repository and build the project:

```sh
git clone https://github.com/miguevrgo/Projects.git
cd projects/Chess/chess 
cargo build --release

