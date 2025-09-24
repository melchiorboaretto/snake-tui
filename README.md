# Snake TUI #

A Rust based snake game to run in the terminal

## How to install ##

Be sure to have:
- Rust and Cargo installed;
- ~/.cargo/bin in your PATH; (or set cargo to install wherever you prefer)


```
git clone https://github.com/melchiorboaretto/snake-tui.git
cd snake-tui
cargo install --path .
```

Or simply
```
cargo install --git https://github.com/melchiorboaretto/snake-tui.git
```


## How to play ##
To start the game type 'snake' in the terminal.

To move the snake use the arrow keys.

To increase/decrease speed use ']' and '[' keys, respectively.

The game uses these chars to represent things:
- @: Snake head
- #: Snake body
- $: Apple
