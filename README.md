# rustshot

Capture terminal command output as beautiful PNG images with macOS-style window chrome. A Rust implementation of [termshot](https://github.com/homeport/termshot).

## Features

- **ANSI color support** - Faithfully renders colored terminal output (256 colors + RGB)
- **Multiple commands** - Render several commands as stacked panels in one image
- **Dynamic font sizing** - Automatically scales font size based on output dimensions
- **Stdin piping** - Pipe output from any command directly into rustshot
- **Terminal chrome** - macOS-style title bar with traffic light buttons, rounded corners, drop shadow
- **Cross-platform** - Works on Windows and Linux
- **Bundled font** - Ships with JetBrains Mono for consistent rendering everywhere

## Installation

### From source

```
cargo install --path .
```

### From releases

Download the latest binary from [Releases](https://github.com/Msfv3n0m/rustshot/releases).

## Usage

```
rustshot [OPTIONS] [-- <COMMANDS>...]
```

### Examples

Capture a single command:
```
rustshot -- "ls -la --color=always"
```

Capture multiple commands with headers:
```
rustshot -c -f screenshot.png -- "git status" "git log --oneline -5"
```

Pipe output from stdin:
```
nmap -sV 10.0.0.1 | rustshot -f scan.png
```

Capture with custom styling:
```
rustshot --font-size 20 -p 24 -m 30 -f output.png -- "echo hello"
```

### Options

| Flag | Description | Default |
|---|---|---|
| `-f, --filename <FILE>` | Output file path | `out.png` |
| `-c, --show-cmd` | Show the command above its output | off |
| `-C, --columns <N>` | PTY column width | `80` |
| `--rows <N>` | PTY row count | `24` |
| `-p, --padding <PX>` | Inner padding in pixels | `16` |
| `-m, --margin <PX>` | Outer margin in pixels | `20` |
| `--no-decoration` | Disable window chrome | off |
| `--no-shadow` | Disable drop shadow | off |
| `--font-size <PX>` | Force font size (disables dynamic sizing) | auto |

## How it works

1. Commands are executed inside a pseudo-terminal (PTY) to preserve ANSI color codes
2. Raw output is parsed through an in-memory terminal emulator (`vt100`) to extract styled cells
3. Font size is dynamically computed to fit content within reasonable image dimensions
4. The styled grid is rendered onto a PNG with terminal window chrome using `imageproc`

## License

MIT
