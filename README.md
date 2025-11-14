# TermiX - Modern Terminal Application

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![Rust](https://img.shields.io/badge/rust-1.82%2B-orange)
![License](https://img.shields.io/badge/license-MIT-green)

**TermiX** is a next-generation portable terminal application built with Rust. It combines the power of traditional terminals with modern UI features and runs on both Linux and Windows platforms.

## ✨ Features

### Core Functionality
- 🖥️ **Multi-Tab Support** - Run multiple terminals and markdown editors simultaneously
- 💡 **Smart Command Suggestions** - Auto-completion for 50+ common shell commands (Tab to complete)
- 🌈 **ANSI Color Support** - Full 16/256/Truecolor terminal rendering
- 📜 **Configurable Scrollback** - 100-10,000 lines of terminal history
- ⌨️ **Keyboard Shortcuts** - Ctrl+T (new tab), Ctrl+W (close), Ctrl+Tab (navigate), and more

### Advanced Features
- 🔗 **SSH Connection Manager** - Save, manage, and connect to remote servers
- ✂️ **Split-View Terminals** - Horizontal/vertical splits with Ctrl+H / Ctrl+Shift+V
- 📝 **Markdown Editor** - Built-in editor with live preview
- 💾 **Settings Persistence** - All configurations auto-saved to ~/.config/termix/
- 📤 **Import/Export** - Backup and restore settings and SSH connections

### Customization
- 🎨 **7 Built-in Themes** - Dark, Light, Dracula, Monokai, Solarized, Nord, Gruvbox
- 🔠 **Font Scaling** - Adjust text size with Ctrl+Plus/Minus/0
- 🎯 **Customizable Cursor** - 6 cursor styles with blinking support
- 🌍 **Cross-Platform** - Runs seamlessly on Linux and Windows
- 🖼️ **Dual Interface** - Terminal UI (TUI) and Graphical UI (GUI)

## 🚀 Quick Start

### Prerequisites

**Rust 1.82 or higher** is required. Install from [rustup.rs](https://rustup.rs/)

**Linux dependencies:**
```bash
sudo apt-get install pkg-config libssl-dev
```

### Installation from Source

1. **Clone the repository:**
   ```bash
   git clone https://github.com/janquillfeldt/terminal-app.git
   cd terminal-app
   ```

2. **Build the project:**
   ```bash
   cargo build --release --features gui
   ```

3. **Run TermiX:**
   ```bash
   ./target/release/termix
   ```

### Using the Launcher Script

The included `start.sh` script provides convenient build and run options:

**GUI Mode** (Recommended):
```bash
./start.sh --gui
```

**TUI Mode** (Terminal UI):
```bash
./start.sh --tui
```

**Release Builds** (optimized, slower compile):
```bash
./start.sh --gui --release
./start.sh --tui --release
```

**Direct Cargo Commands:**
```bash
# TUI (default)
cargo run --release

# GUI
cargo run --release --features gui
```

### Binary Releases

Pre-built binaries for Linux and Windows are available on the [Releases page](https://github.com/janquillfeldt/terminal-app/releases).

Simply download, extract, and run!

## 📖 Usage Guide

### GUI Mode

**Navigation:**
- 🖥️ **Terminal** - Multi-tab terminal with command suggestions
- 🔗 **SSH Connections** - Manage and connect to remote servers
- 📝 **Markdown** - Write and preview markdown documents
- ⚙️ **Settings** - Customize themes, colors, fonts, and behavior
- ℹ️ **About** - View system information and version details

**Keyboard Shortcuts:**
- `Ctrl+T` - New terminal tab
- `Ctrl+W` - Close active tab
- `Ctrl+Tab` / `Ctrl+Shift+Tab` - Navigate tabs
- `Ctrl+H` - Horizontal split
- `Ctrl+Shift+V` - Vertical split
- `Ctrl+1-9` - Switch between split panes
- `Ctrl+Plus/Minus/0` - Zoom in/out/reset
- `PageUp/PageDown` - Scroll terminal

**Terminal Features:**
- Type commands and press `Tab` for auto-completion
- Use arrow keys `↑↓` to select suggestions
- Command history available with arrow keys in terminal
- Drag tabs to reorder, Ctrl+Click to close

### TUI Mode

**Navigation:**
- `↑↓` - Navigate menu items
- `Enter` - Select option
- `ESC` or `Q` - Exit application

### Settings Persistence

All settings are automatically saved to:
- **Linux/macOS:** `~/.config/termix/settings.toml`
- **Windows:** `%APPDATA%\termix\settings.toml`

SSH connections are stored in `ssh_connections.toml` in the working directory.

## 🖼️ Screenshots

Below is an overview of the main application views. (Place your captured screenshots into `assets/` with the filenames shown.)

| View | Description | Filename |
|------|-------------|----------|
| Terminal (DE, Dark Theme) | Multi-tab terminal with split panes | `assets/terminal-de-dark.png` |
| Terminal (EN, Light Theme) | Alternate language + theme | `assets/terminal-en-light.png` |
| SSH Manager | Saved connections and connect dialog | `assets/ssh-manager.png` |
| Markdown Editor | Editor with live preview and multiple tabs | `assets/markdown-editor.png` |
| Settings | Themes, fonts, cursor, language switch | `assets/settings-panel.png` |
| About | Feature & technology overview | `assets/about-view.png` |
| Exit / Dialog Example | Exit panel or rename dialog | `assets/exit-dialog.png` |

### How to Capture
Use your OS screenshot tools while the GUI is running:

Linux (Wayland):
```bash
grim -g "$(slurp)" assets/terminal-de-dark.png
```
Linux (X11):
```bash
import -window root assets/terminal-de-dark.png  # from ImageMagick
```
Generic (desktop keybindings):
- Print Screen → Full screen (then save/crop)
- Shift+Print → Area selection (GNOME/KDE)

Inside the app:
1. Switch language: Settings → Language (🇩🇪 / 🇬🇧)
2. Change theme: Settings → Theme list (pick Dark/Light/Dracula/etc.)
3. Open required view from sidebar (Terminal/SSH/Markdown/Settings/About/Exit)
4. Arrange splits (Ctrl+H / Ctrl+Shift+V) for terminal screenshot.
5. For dialog screenshot: rename a tab (✏) or open Exit view.

Once files are added, the above table will render them. Example embedding:
```markdown
![Terminal Dark DE](assets/terminal-de-dark.png)
```

> Optional: add more variants (e.g. Dracula theme, Nord theme) using similar filenames.

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Commit your changes** (`git commit -m 'Add amazing feature'`)
4. **Push to the branch** (`git push origin feature/amazing-feature`)
5. **Open a Pull Request**

Please ensure your code:
- Builds without warnings (`cargo clippy`)
- Follows Rust formatting (`cargo fmt`)
- Includes appropriate tests where applicable

## 💖 Support

If you find TermiX useful, consider supporting the development:

<a href="https://www.buymeacoffee.com/janquillfeldt" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" style="height: 60px !important;width: 217px !important;" ></a>

Your support helps maintain and improve TermiX!

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.