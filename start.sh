#!/bin/bash
set -euo pipefail

# TermiX Launcher Script
# Modern Terminal Application with Multi-Tab support

# Lade Rust-Umgebung falls vorhanden
if [ -f "$HOME/.cargo/env" ]; then
    # shellcheck disable=SC1091
    source "$HOME/.cargo/env"
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

MODE="tui" # default mode
FEATURES=""  # no extra features by default

while [[ $# -gt 0 ]]; do
    case "$1" in
        --gui)
            MODE="gui"
            FEATURES="--features gui"
            shift
            ;;
        --tui)
            MODE="tui"
            FEATURES=""  # ensure no gui feature
            shift
            ;;
        --release)
            RELEASE=1
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [--gui|--tui] [--release]"
            exit 0
            ;;
        *)
            echo "Unbekannte Option: $1"
            exit 1
            ;;
    esac
done

PROFILE_DIR="target/release"
BUILD_CMD=(cargo build --release)
BIN_PATH="$PROFILE_DIR/termix"

if [ -z "${RELEASE:-}" ]; then
    PROFILE_DIR="target/debug"
    BUILD_CMD=(cargo build)
    BIN_PATH="$PROFILE_DIR/termix"
fi

echo "[INFO] Baue TermiX (${MODE})..."
"${BUILD_CMD[@]}" $FEATURES

echo "[INFO] Starte TermiX..."
if [ "$MODE" = "gui" ]; then
    "$BIN_PATH"
else
    "$BIN_PATH" --tui
fi
