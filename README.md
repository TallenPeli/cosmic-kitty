# Cosmic Kitty

This program converts an exported `.ron` cosmic(tm) terminal theme into one that is useable in Kitty.

## Dependencies

The only dependency that you need is `cargo` which you can install with these instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

## Build Instructions

```
git clone https://github.com/TallenPeli/cosmic-kitty && cd cosmic-kitty
cargo build --release
```

This will place the binary in `/target/release/`.

## Usage

1. Get a cosmic(tm) terminal theme by going to:

`View->Color Schemes...->{three dots next to theme you want to export}->Export`

Take the exported `.ron` file and place it somewhere memerable.

2. Run the command:

```
cosmic-kitty <path to cosmic-term theme>.ron
```

This should make a file with the format `<Theme Name>-kitty.conf`.

3. Move the theme to `~/.config/kitty/`

4. Make sure your kitty config uses your theme by adding this line to your `~/.config/kitty/kitty.conf`

```
include <theme name>-kitty.conf
```

5. Restart Kitty and see your theme applied!

## Features

| Status | Feature                                 |
| ------ | --------------------------------------- |
| ✅     | Text coloring                           |
| ❌     | Background color following cosmic theme |
| ❌     | Automatic theme syncing                 |
| ✅     | Theme parsing and generation            |

## Contributing

Please feel free to fork this repository and/or make a pull request with any changes you made. Any help will be greatly appreciated!
