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
A "cowsay" clone with Hatsune Miku and Kasane Teto ASCII art and speech bubbles.

Usage: mikusays [OPTIONS] [TEXT]

Arguments:
  [TEXT]  Text to display; read piped stdin when omitted

Options:
      --teto
          Use the separate Kasane Teto art collection instead of Miku
      --character <CHARACTER>
          Character collection, or random selection across both collections [possible values: miku, teto, random]
      --small
          Randomly choose a small artwork (at most 40 columns and 20 lines)
      --medium
          Randomly choose a medium artwork (at most 80 columns and 40 lines, not small)
      --large
          Randomly choose a large artwork (over 80 columns or 40 lines)
      --think
          Use a thought bubble with circles instead of a speech tail
      --align <ALIGN>
          Horizontal alignment of the entire drawing [default: center] [possible values: left, center, right]
      --art <ART>
          Display a custom UTF-8 artwork, even if it does not fit
      --exclude-styles <EXCLUDE_STYLES>
          Exclude indices from random selection (comma-separated, applies to each collection)
      --config <CONFIG>
          Load defaults from a file (one option per line)
      --no-config
          Ignore the automatic configuration file
      --save-config <SAVE_CONFIG>
          Save effective options as a configuration file and exit
      --configure
          Open the interactive configuration editor (arrow keys, preview, save/load)
  -s, --style <STYLE>
          Style of the selected character. Otherwise, randomly choose one that fits the terminal
  -l, --list
          List all available art styles with their indices
      --rainbow
          Apply a smooth rainbow gradient across the ASCII art
      --saturation <SATURATION>
          Saturation level for rainbow gradient (0-100) [default: 100]
      --brightness <BRIGHTNESS>
          Brightness level for rainbow gradient (0-100) [default: 50]
      --color <COLOR>
          Override with a single solid color
      --gradient <START:END>
          Define a custom two-color gradient (e.g., --gradient red:blue)
      --no-color
          Disable all coloring (respects NO_COLOR environment variable)
      --color-debug
          Print color detection/debug info and exit
  -h, --help
          Print help
  -V, --version
          Print version
```

### Terminal-aware selection

Without `--style`, mikusays randomly chooses only artwork that fits the terminal's
width and height together with the speech bubble. The bubble wraps to the available
width. If no artwork fits, only the wrapped message is printed. An explicit
`--style <number>` always displays that artwork, even if it exceeds the terminal size.
`--list` still shows every style. If terminal size detection is unavailable (for
example, in a pipe or CI), selection uses 80 columns by 24 rows.

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

### Pipes, thoughts, alignment, and custom art

```bash
fortune | mikusays --character random --small
printf 'Hello!\nAnother line\n' | mikusays --teto --think
mikusays "Thinking…" --think --align right
mikusays "Hi!" --art ./my-art.txt --gradient cyan:blue
mikusays "Hi!" --character random --exclude-styles 3,7,12
```

If `TEXT` is omitted, piped stdin is read until EOF; explicit `TEXT` takes priority.
Newlines and blank paragraphs are preserved, and long lines wrap by display width.
An empty/whitespace-only pipe reports a missing-message error rather than waiting
for interactive input. `--list`, `--color-debug`, and `--save-config` need no text.

`--align left|center|right` aligns the entire drawing (center is the default).
The speech/thought tail stays centered beneath its bubble in every alignment.
`--art FILE` reads a UTF-8 text file, preserving artwork whitespace. Like an explicit
`--style`, it deliberately bypasses terminal-fit and random-size filters. An empty
art file is rejected. Custom art cannot be combined with `--style`, `--teto`,
`--character`, or `--list`.

### Random-selection filters

Choose at most one of:

- `--small`: artwork at most 40 display columns **and** 20 lines.
- `--medium`: artwork at most 80 columns **and** 40 lines, excluding small artwork.
- `--large`: artwork exceeding 80 columns **or** 40 lines.

Sizes refer to the artwork, including its existing whitespace; terminal-fit checks
also include the bubble and its tail. Large artwork still requires a large enough
terminal. If the filters leave no fitting artwork, only the message is printed.

`--character miku|teto|random` selects a collection; `--teto` remains a shorthand.
`random` chooses uniformly from the eligible artworks across both collections.
`--exclude-styles 3,7,12` excludes those indices in each selected collection.
An explicit `--style N` overrides size/exclusion/terminal-fit filters; with
`--character random`, it chooses style N from one of the two collections.
`--list` is unfiltered; `--character random --list` lists both collections.

### Interactive configuration editor

```bash
mikusays --configure                    # Edit the automatic config
mikusays --configure --config ./my.conf # Open/create another config
mikusays --configure --no-config        # Start from built-in defaults
```

The full-screen terminal interface shows the current character, style, size filter,
bubble, alignment, colors, saturation, brightness, exclusions, custom artwork, and
save/load path. No message is required. Both stdin and stdout must be terminals.

- **Up/Down** (or Tab): select a setting; the list scrolls on smaller terminals.
- **Enter/Right/Space**: edit a value or cycle a choice; Enter on Save writes the file.
- **P**: preview with the current settings; any key returns to the editor.
- **S**: save to the displayed config path (including replacing an existing file).
- **L**: reload that file; **R**: reset settings to built-in defaults without saving.
- **Q/Esc/Ctrl+C**: exit; unsaved edits require a discard confirmation.
- In an input dialog, **Ctrl+U** clears, **Backspace** deletes, **Enter** applies,
  and **Esc** cancels. Confirm discard by typing `y` and pressing Enter.

Use `random` in the Style dialog, or leave it empty, to remove an explicit style.
An empty exclusions field clears the list; an empty custom-art path switches back
to built-in artwork. Editing Solid color or Gradient activates that color mode;
Color mode also cycles through auto, solid, gradient, rainbow, and no color.
Changing the Config file field selects a save-as/load target; it does not load it
until **L** is pressed. The preview message is never saved as a default message.
Invalid entries leave the previous value intact. A malformed config opens with
an error notice and default settings, allowing repair via Save. No file is changed
until Save is selected. Terminal state is restored on exit.

### Configuration

Save your preferred options (not the message itself):

```bash
mikusays --character random --small --think --gradient cyan:blue \
  --save-config ~/.config/mikusays/config
mikusays "Hello!"                    # Load those defaults automatically
mikusays "Hello!" --color red        # Override the saved gradient
mikusays "Hello!" --no-config        # Ignore automatic defaults
mikusays "Hello!" --config ./my.conf # Load a particular file instead
```

The automatic path is `$XDG_CONFIG_HOME/mikusays/config` when set, otherwise
`~/.config/mikusays/config` (Windows: `%APPDATA%/mikusays/config`). A missing
automatic file is fine; a missing explicit `--config` file is an error.
`--save-config FILE` creates parent directories and replaces that file.

The file contains **one long option per line**, optionally followed by its value.
Blank lines and lines starting with `#` are ignored. Values are literal: no shell
execution, variable expansion, or quoting is performed. For example:

```text
# My terminal greeting
--character random
--small
--think
--align left
--gradient cyan:blue
--exclude-styles 3,7,12
```

CLI options take priority over matching config options, including the mutually
exclusive character, size, and color groups. A CLI `--art` replaces configured
character/style choices, and a CLI character/style choice replaces configured
custom art. Boolean defaults can be removed by editing the file or using
`--no-config`. Relative `--art` paths in a config resolve relative to that file;
saved artwork paths are absolute. Config files accept only the documented default
options, not messages, `--list`, or configuration-loading/saving commands.

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
