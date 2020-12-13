# snake

This is a snake game I made to explore Rust a bit.

## Installation and playing

![screenshot](snake.gif)

* It looks better in real life than on this asciicast, I promise.

```
cargo install --git https://github.com/adder46/snake.git
```

Then run:

```
snake
```

## Development

You will need curses libraries:

```
sudo apt install libncurses5 libncurses5-dev
```

Fork the repository, and make sure to run `clippy` and `rustfmt` before submitting a PR. There are no tests at this moment as this is mostly a toy project.

## Licensing

Licensed under the [MIT License](https://opensource.org/licenses/MIT). For details, see [LICENSE](https://github.com/adder46/snake/blob/master/LICENSE).
