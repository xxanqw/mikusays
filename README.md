<div align=center>

<img width="500px" src="https://github.com/user-attachments/assets/f8f42b9f-43c9-4c0b-914c-fc2bce5a438d" />


# MikuSays 🎤

A `cowsay` clone with Hatsune Miku ASCII art and speech bubbles, plus an optional Kasane Teto collection.

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
  -s, --style <STYLE>            Style of the selected character. Otherwise, randomly choose one that fits the terminal
  -l, --list                     List all available art styles with their indices
      --teto                     Use the separate Kasane Teto art collection instead of Miku
      --rainbow                  Apply a smooth rainbow gradient across the ASCII art
      --saturation <SATURATION>  Saturation level for rainbow gradient (0-100) [default: 100]
      --brightness <BRIGHTNESS>  Brightness level for rainbow gradient (0-100) [default: 50]
      --color <COLOR>            Override with a single solid color
      --gradient <START:END>     Define a custom two-color gradient (e.g., --gradient red:blue)
      --no-color                 Disable all coloring (respects NO_COLOR environment variable)
  -h, --help                     Print help
  -V, --version                  Print version
```

### Terminal-aware selection

Without `--style`, mikusays randomly chooses only artwork that fits the terminal's
width and height together with the speech bubble. The bubble wraps to the available
width. If no artwork fits, only the wrapped message is printed. An explicit
`--style <number>` always displays that artwork, even if it exceeds the terminal size.
`--list` still shows every style.

### Kasane Teto

Teto has a separate collection and style indices; Miku remains the default.
The collection contains 20 Teto artworks, matching Miku’s 20 styles. Styles 0–1 are the examples supplied in [issue #25](https://github.com/xxanqw/mikusays/issues/25); styles 2–19 are additional reviewed selections from [Emoji Combos](https://emojicombos.com/teto-ascii-art). See [art sources](src/teto/SOURCES.md) for the preview-to-style mapping.

```bash
mikusays --teto "Hello from Teto!"          # Random Teto style
mikusays --teto --list                     # List only Teto styles (0–19)
mikusays --teto --style 0 "Hello!"         # Select a Teto style
mikusays --teto --style 1 --color red "Hi!" # Color options also work for Teto
mikusays --list                            # Miku styles, unchanged
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
