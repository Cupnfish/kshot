# kshot - Windows Screenshot CLI

A fast Windows screenshot CLI tool using Windows Graphics Capture, built with Rust.

## Features

- Capture primary or specific screen by index
- List all connected displays
- High-DPI aware via Windows Graphics Capture
- HDR to SDR color space conversion support

## Installation

### Pre-built Binary

Download from [Releases](https://github.com/Cupnfish/kshot/releases) or use with Alma skill:

```
~/.config/alma/skills/windows-screenshot/bin/kshot.exe
```

### Build from Source

```bash
git clone https://github.com/Cupnfish/kshot.git
cd kshot
cargo build --release
```

The binary will be at `target/release/kshot.exe`.

## Usage

```bash
# Capture primary screen
kshot -o screenshot.png

# List all screens
kshot --list

# Capture second screen
kshot -o screenshot.png -s 1
```

### Options

| Option | Description |
|--------|-------------|
| `-o, --output <PATH>` | Output file path (default: screenshot.png) |
| `-s, --screen <INDEX>` | Screen index to capture (0-based) |
| `-l, --list` | List all available screens |

## Requirements

- Windows 10 or Windows 11
- Windows Graphics Capture support

## License

MIT

## Credits

Powered by [kscreenshot](https://github.com/Cupnfish/kscreenshot) library.
