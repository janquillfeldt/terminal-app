# TermiX - Modern Terminal Application

**TermiX** is a next-generation portable terminal application that runs on both Linux and Windows platforms. It combines the power of traditional terminals with modern UI features.

## ✨ Features

- **Multi-Tab Support** - Run multiple terminals and markdown editors simultaneously
- **Smart Command Suggestions** - Auto-completion for 50+ common shell commands
- **Cross-Platform** - Runs seamlessly on Linux and Windows
- **Dual Interface** - Terminal UI (TUI) and Graphical UI (GUI)
- **Embedded PTY Terminal** - Full shell integration with live output
- **SSH Connection Manager** - Save and manage SSH connections
- **Markdown Editor** - Built-in editor with live preview
- **Customizable** - Dark/Light themes, font scaling
- **Green Cursor** - Visual indicator for active input position

## 🚀 Quick Start

### Installation

1. Install dependencies (Linux):
   ```bash
   sudo apt-get install pkg-config libssl-dev
   ```

2. Clone and build:
   ```bash
   git clone <repository-url>
   cd termix
   cargo build --release
   ```

### Running TermiX

Start with the convenient launcher script:

- **GUI Mode** (Recommended):
  ```bash
  ./start.sh --gui
  ```

- **TUI Mode** (Terminal UI):
  ```bash
  ./start.sh --tui
  ```

- **Release Build**:
  ```bash
  ./start.sh --gui --release
  ```
    - Debug/Release:
       ```bash
       ./start.sh --tui          # Debug
       ./start.sh --tui --release  # Release
       ```

3. Direkt per Cargo:
    - TUI:
       ```bash
       cargo run --
       ```
    - GUI (Feature aktivieren):
       ```bash
       cargo run --features gui --
       ```

## Usage Guidelines

- TUI: Pfeiltasten ↑/↓, Enter zum Auswählen, ESC/Q zum Beenden
- GUI:
   - Terminal-Panel: Eingabefeld unten, Enter oder "Ausführen"
   - Einstellungen: Dark Mode, Schrift-Skalierung
   - Über: Systeminfos (OS, Rust-Version)
   - Beenden: schließt das Fenster

## Contributing

Contributions are welcome! Please submit a pull request or open an issue for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.