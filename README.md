# clrgen

**Description**:  color scheme config generator for popular linux applications written in Rust.

**Status**: work in progress.

**OS**: Linux.

## Installation

### Manual

- download the latest tarball from "Release" section
- extract it to any directory on your pc
- run `install.sh`

## Configuration

Work in progress, there are no config file yet.

## Usage

`clrgen [OPTIONS] --theme <THEME>`

```
Options:
  -t, --theme <THEME>      Path to the color theme
  -a, --appname <APPNAME>  Target app.
  -c, --config <CONFIG>    Path to config file [default: ~/.config/clrgen/clrgen.conf]
  -h, --help               Print help
  -V, --version            Print version
```

**Available apps options**: `kitty`, `alacritty`, `waybar`, `i3`, `sway`, `eww`, `all`.

**Examples:**

`clrgen -t ~/.clrgen/colors/dracula` - create `dracula` colorschemes for all available apps.
`clrgen -t ~/.clrgen/colors/dracula -a kitty` - create `dracula` colorscheme for kitty.

## Building

```
git clone https://github.com/obsqrbtz/clrgen.git
cd clrgen
mkdir ~/.clrgen
cp -r templates ~/.clrgen/
cargo build
```
