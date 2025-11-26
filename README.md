<div align=center>

<img width=500px src="https://github.com/user-attachments/assets/d08929ed-85c6-42c5-8858-b46753d34205">

# MikuSays 🎤

A `cowsay` clone with Hatsune Miku ASCII art and speech bubbles.  

[![GitHub release (latest by date)](https://img.shields.io/github/v/release/xxanqw/mikusays?style=for-the-badge)](https://github.com/xxanqw/mikusays/releases/latest)
[![AUR Package Version](https://img.shields.io/aur/version/mikusays?style=for-the-badge)](https://aur.archlinux.org/packages/mikusays)

</div>

## Usage

```bash
mikusays "Hello, World!" # Chooses random style

mikusays "World is Mine!" --style 4

mikusays --list # Display all available styles
```

## Installation

### Arch Linux
```bash
yay -S mikusays
paru -S mikusays
```
> [!NOTE]
> On update 'cause of same linux file naming, you need to clear AUR cache before installing or hash will missmatch
> ```bash
> yay -Scc
> paru -Scc
> ```

### Debian / Ubuntu / Fedora / RHEL / CentOS
The repository and instructions available here: https://apt.xxanqw.pp.ua

### macOS (or Linux if using Homebrew)
```bash
brew tap xxanqw/mikusays
brew install mikusays
```

### Windows  
- **Using Scoop**:
  
  ```bash
  scoop bucket add extras
  scoop bucket add xxanqw-bucket https://github.com/xxanqw/scoop-bucket
  scoop update
  scoop install mikusays
  ```
- **Using Installer**:
  
  Download `mikusays-windows-inst-any-x.x.x.exe` from [Latest release page](https://github.com/xxanqw/mikusays/releases/latest)

---

### Download from [Releases](https://github.com/xxanqw/mikusays/releases):

- **Windows**: `mikusays-windows-*.exe`
    - Small note for Windows that application depends on `vcredist2022`  
      (installing with scoop will resolve this dependency automatically)
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
