# Satra
- A lightweight terminal-based process launcher and tracker.

## Installation

```sh
curl -sSL https://raw.githubusercontent.com/TattvaOrg/Satra/main/install.sh | bash
```

## Updating

Update Satra to the latest version at any time:

```sh
satra update
```

Or re-run the installer:

```sh
curl -sSL https://raw.githubusercontent.com/TattvaOrg/Satra/main/install.sh | bash
```

## Features

- Detached process launching
- Simple TUI with minimal dark theme
- Process status and uptime tracking
- Process management (Kill, Restart, Delete)
- Auto-clears exited processes

## Usage

Run `satra` in your terminal:

```sh
satra
```

### Keybindings

| Key | Mode | Action |
| --- | --- | --- |
| `Tab` | Any | Toggle focus between input bar and session list |
| `Enter` | Input | Launch command as detached background process |
| `↑` / `↓` | List | Navigate through active sessions |
| `Enter` | List | Open Session Detail panel |
| `ESC` | List | Return focus to input bar |
| `k` | Detail Panel | Send SIGKILL to the process |
| `r` | Detail Panel | Restart the process |
| `d` | Detail Panel | Terminate and remove from list |
| `q` / `ESC` | Detail Panel | Close detail panel |
| `Ctrl+C` | Any | Quit Satra (launched processes continue running) |

> **Note**: Launched processes run detached via `setsid`. Quitting Satra will not terminate your running applications. Exited sessions are kept in the list with exit codes and automatically clean up after 5 minutes.

## Build from Source

```sh
git clone https://github.com/TattvaOrg/Satra.git
cd Satra
cargo build --release
```
