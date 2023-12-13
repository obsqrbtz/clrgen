# clrgen

**Description**:  color scheme config generator for popular linux applications written in Rust.

**Status**: work in progress.

**OS**: Linux.

## Installation

### Manual

- download the latest tarball from "Release" section
- extract it to any directory on your pc
- run `chmod +x install.sh && ./install.sh`

## Usage

`clrgen [OPTIONS] --theme <THEME>`

```
Options:
  -t, --theme <THEME>      Path to the color theme
  -a, --appname <APPNAME>  Target app [default: use_config].
  -c, --config <CONFIG>    Path to config file [default: ~/.config/clrgen/clrgen.conf]
  -h, --help               Print help
  -V, --version            Print version
```

**Available apps options**: `kitty`, `alacritty`, `waybar`, `i3`, `sway`, `eww`, `all`.

Themes will be created in `~/.clrgen` directory.

**Examples:**

`clrgen -t ~/.config/.clrgen/colors/dracula` - create `dracula` colorschemes for all available apps.
`clrgen -t ~/.config/.clrgen/colors/dracula -a kitty` - create `dracula` colorscheme for kitty.

## Configuration

**Default configuration file location**: ~/.config/clrgen/clrgen.toml.

This file is used to define the app list for clrgen. If you don't need colorscheme for the app `x` simply comment or remove the corresponding line.

```
[config]
apps = ["kitty", 
        "alacritty", 
        #"waybar", 
        #"eww", 
        "i3", 
        "sway"]
```

If `--config` flag is not specified, `clrgen` uses default configuration file.

## Building

```
git clone https://github.com/obsqrbtz/clrgen.git
cd clrgen
mkdir ~/.config/.clrgen
cp -r templates ~/.config/.clrgen/
cp clrgen.conf ~/.config/.clrgen/
cargo build
```
