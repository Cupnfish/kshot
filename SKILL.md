# Windows Screenshot Skill

Take screenshots on Windows using `kshot` CLI - a Rust-based screenshot tool using Windows Graphics Capture.

## Binary Location

```
~/.config/alma/skills/windows-screenshot/bin/kshot.exe
```

## Installation

The binary is pre-built. To rebuild from source:

```bash
cd ~/.config/alma/skills/windows-screenshot
cargo build --release
cp target/release/kshot.exe bin/
```

## Usage

```bash
# Capture primary screen
kshot -o C:\Users\cupnfish\AppData\Local\Temp\screenshot.png

# List all screens
kshot --list

# Capture specific screen by index
kshot -o screenshot.png -s 1
```

## Take Screenshot for Alma

**Important:** Always resize screenshots before reading into context to avoid exceeding token limits.

```bash
# 1. Capture screen
kshot -o C:\Users\cupnfish\AppData\Local\Temp\alma-screenshot.png

# 2. Resize to 1280px wide (PowerShell)
powershell -Command "Add-Type -AssemblyName System.Drawing; $img = [System.Drawing.Image]::FromFile('C:\Users\cupnfish\AppData\Local\Temp\alma-screenshot.png'); $ratio = 1280 / $img.Width; $newHeight = [int]($img.Height * $ratio); $thumb = New-Object System.Drawing.Bitmap(1280, $newHeight); $g = [System.Drawing.Graphics]::FromImage($thumb); $g.DrawImage($img, 0, 0, 1280, $newHeight); $thumb.Save('C:\Users\cupnfish\AppData\Local\Temp\alma-screenshot-thumb.jpg', [System.Drawing.Imaging.ImageFormat]::Jpeg); $g.Dispose(); $img.Dispose(); $thumb.Dispose()"

# 3. Read the resized version with Read tool
# Path: C:\Users\cupnfish\AppData\Local\Temp\alma-screenshot-thumb.jpg
```

## Options

| Option | Description |
|--------|-------------|
| `-o, --output <PATH>` | Output file path (default: screenshot.png) |
| `-s, --screen <INDEX>` | Screen index to capture (0-based) |
| `-l, --list` | List all available screens |

## Tips

- Always resize before reading into context
- Primary screen is captured by default
- Use `--list` to see all connected displays
- Supports high-DPI displays via Windows Graphics Capture
