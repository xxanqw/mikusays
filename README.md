<div align=center>

# MikuSays 🎤

A `cowsay` clone with Hatsune Miku ASCII art and speech bubbles.  

[![GitHub release (latest by date)](https://img.shields.io/github/v/release/xxanqw/mikusays?style=for-the-badge)](https://github.com/xxanqw/mikusays/releases/latest)
[![AUR Package Version](https://img.shields.io/aur/version/mikusays?style=for-the-badge)](https://aur.archlinux.org/packages/mikusays)

</div>

## Usage

```bash
mikusays "Hello, World!"
```

## Installation

### Arch Linux
```bash
yay -S mikusays
paru -S mikusays
```

### macOS (or Linux if using Homebrew)
```bash
brew tap xxanqw/mikusays
brew install mikusays
```

### Windows (using Scoop)
```bash
scoop bucket add xxanqw-bucket https://github.com/xxanqw/scoop-bucket
scoop update
scoop install mikusays
```

Download from [Releases](https://github.com/xxanqw/mikusays/releases):

- **Windows**: `mikusays-windows-*.exe`
- **Linux**: `mikusays-linux-*` 
- **macOS**: `mikusays-macos-*`

Or build from source:
```bash
cargo build --release
```

## Development

```bash
cargo build
cargo run -- "message"
```
