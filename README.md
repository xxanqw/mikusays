<div align=center>

<img width="500px" src="https://github.com/user-attachments/assets/f8f42b9f-43c9-4c0b-914c-fc2bce5a438d" />


# MikuSays 🎤

A `cowsay` clone with Hatsune Miku ASCII art and speech bubbles.  

**NOW WITH**  
<img height=24 src="https://cdn.xserv.pp.ua/files/css/COLORS.svg" alt="COLORS">


[![GitHub release (latest by date)](https://img.shields.io/github/v/release/xxanqw/mikusays?style=for-the-badge)](https://github.com/xxanqw/mikusays/releases/latest)
[![AUR Package Version](https://img.shields.io/aur/version/mikusays?style=for-the-badge)](https://aur.archlinux.org/packages/mikusays)

</div>

## Usage

```bash
Usage: mikusays [OPTIONS] [TEXT]

Arguments:
  [TEXT]  Text to display in the speech bubble

Options:
  -s, --style <STYLE>            Style of the Miku art. A random one is chosen if not specified
  -l, --list                     List all available art styles with their indices
      --rainbow                  Apply a smooth rainbow gradient across the ASCII art
      --saturation <SATURATION>  Saturation level for rainbow gradient (0-100) [default: 100]
      --brightness <BRIGHTNESS>  Brightness level for rainbow gradient (0-100) [default: 50]
      --color <COLOR>            Override with a single solid color
      --gradient <START:END>     Define a custom two-color gradient (e.g., --gradient red:blue)
      --no-color                 Disable all coloring (respects NO_COLOR environment variable)
  -h, --help                     Print help
  -V, --version                  Print version
```

## Installation

### Arch Linux
```bash
yay -S mikusays
paru -S mikusays
```

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
      (installing with scoop and installer resolve this dependency automatically)
- **Linux**: `mikusays-linux-*.tar.gz`
- **macOS**: `mikusays-macos-*.tar.gz`

Linux and macOS archives preserve the executable permission. Extract and run one with:

```bash
archive="mikusays-linux-x64-VERSION.tar.gz"
tar -xzf "$archive"
"./${archive%.tar.gz}" "Hello, World!"
```

The macOS binaries are currently unsigned. If Gatekeeper quarantines an extracted binary,
remove the quarantine attribute before running it:

```bash
xattr -d com.apple.quarantine ./mikusays-macos-arm64-VERSION
```

Or build from source:
```bash
cargo build --release
```

## Development

### Release automation

Tagged releases build the platform binaries first, then trigger the Scoop bucket.
After all DEB and RPM assets have uploaded, the package workflow triggers the APT/RPM
repository with the exact tag. The `RELEASE_TOKEN` Actions secret must be able to create
repository dispatch events in `xxanqw/scoop-bucket`, `xxanqw/homebrew-mikusays`, and
`xxanqw/apt-repository`. If that token expires, the release remains successful and the
downstream repositories recover through their scheduled update workflows. Uploads to the
MikuSays release itself use the built-in workflow token and do not depend on this secret.

The distribution repositories keep scheduled and manual workflows as recovery paths if
an event dispatch fails.

1. Fork repository
2. Clone your fork
3. Implement your changes
4. If needed, write tests (it would be great)
5. Run your code with your changes
   ```bash
   cargo run -- [your implemented methods, or just text if it's ASCII only].
   ```
7. Run the tests
   ```bash
   cargo test
   ```
8. Check your code quality with:
   ```bash
   cargo clippy -- -D warnings
   ```
   Fix any errors that appear
   
10. Finally, run the formatter:
    ```bash
    cargo fmt
    ```
11. Commit and push your changes
12. Open a pull request

Now you're a contributor!

<div align=center>

<img height="124" src="https://github.com/user-attachments/assets/f3d4e4e8-0fa4-40a1-bc89-aafbbd896a73" />

</div>
