#!/bin/bash
set -e

echo "Installing Satra..."

ARCH=$(uname -m)
case $ARCH in
    x86_64)
        TARGET_ARCH="x86_64"
        ;;
    aarch64|arm64)
        TARGET_ARCH="aarch64"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

BIN_DIR="$HOME/.local/bin"
mkdir -p "$BIN_DIR"

URL="https://github.com/TattvaOrg/Satra/releases/latest/download/satra-$TARGET_ARCH-linux"

echo "Downloading from $URL"
curl -sSL "$URL" -o "$BIN_DIR/satra"
chmod +x "$BIN_DIR/satra"

echo "✅ Installed successfully to $BIN_DIR/satra"
echo "Make sure $BIN_DIR is in your PATH."
echo "Run 'satra' to start the application."
