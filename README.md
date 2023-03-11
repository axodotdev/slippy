# axoslides

> 🖼 the easiest way to make slideshows

## Usage

```sh
Usage: axoslides [OPTIONS] <FILE>

Arguments:
  <FILE>

Options:
  -t, --theme <THEME>  [possible values: light, dark, cupcake]
  -h, --help           Print help
  -V, --version        Print version
```

### Basic usage

```sh
> axoslides deck.md
```

### Changing the theme

```sh
> axoslides --theme=dark deck.md
```

Current themes are:

- light (default)
- dark
- cupcake

### Using local assets

Any images or assets that you want to be on your published slideshow must be in the `static` folder, this folder will be copied over to the public directory, ready to be uploaded to any static file hoster.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))

at your option.
