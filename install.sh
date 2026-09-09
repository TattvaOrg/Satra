#!/bin/bash
set -e

ARCH=$(uname -m)
case $ARCH in
    x86_64)
        TARGET_ARCH="x86_64"
        ;;
    aarch64|arm64)
        TARGET_ARCH="aarch64"
        ;;
    *)
        echo "Error: Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

BIN_DIR="${CARGO_HOME:-$HOME/.local}/bin"
[ -d "$HOME/.local/bin" ] && BIN_DIR="$HOME/.local/bin"
mkdir -p "$BIN_DIR"

IS_UPDATE=false
if [ -f "$BIN_DIR/satra" ]; then
    IS_UPDATE=true
    CURRENT_VERSION=$("$BIN_DIR/satra" --version 2>/dev/null || echo "existing")
    echo "==> Existing Satra installation detected ($CURRENT_VERSION)."
    echo "==> Updating Satra at $BIN_DIR/satra..."
else
    echo "==> Installing Satra to $BIN_DIR/satra..."
fi

URL="https://github.com/TattvaOrg/Satra/releases/latest/download/satra-$TARGET_ARCH-linux"
TMP_BIN=$(mktemp)
trap 'rm -f "$TMP_BIN"' EXIT

echo "==> Fetching prebuilt release binary..."
if curl -sSfL "$URL" -o "$TMP_BIN" 2>/dev/null; then
    # Verify file is an executable binary and not an error page
    if file "$TMP_BIN" | grep -qE "ELF|executable"; then
        # Unlink existing binary first to avoid ETXTBSY if currently running
        rm -f "$BIN_DIR/satra"
        mv -f "$TMP_BIN" "$BIN_DIR/satra"
        chmod +x "$BIN_DIR/satra"
        NEW_VER=$("$BIN_DIR/satra" --version 2>/dev/null || echo "")
        if [ "$IS_UPDATE" = true ]; then
            echo "✅ Satra updated successfully! ($NEW_VER)"
        else
            echo "✅ Satra installed successfully! ($NEW_VER)"
        fi
    else
        echo "Error: Downloaded file is not a valid binary."
        exit 1
    fi
else
    echo "Notice: Prebuilt release binary not found at $URL"
    if command -v cargo >/dev/null 2>&1; then
        echo "==> Compiling and updating from source using cargo..."
        cargo install --force --git https://github.com/TattvaOrg/Satra.git --root "${BIN_DIR%/bin}"
        NEW_VER=$("$BIN_DIR/satra" --version 2>/dev/null || echo "")
        if [ "$IS_UPDATE" = true ]; then
            echo "✅ Satra updated successfully via cargo! ($NEW_VER)"
        else
            echo "✅ Satra installed successfully via cargo! ($NEW_VER)"
        fi
    else
        echo "Error: Could not download prebuilt binary and 'cargo' is not installed."
        echo "Please install Rust (https://rustup.rs) or check https://github.com/TattvaOrg/Satra/releases"
        exit 1
    fi
fi

# PATH guidance
case ":$PATH:" in
    *":$BIN_DIR:"*) ;;
    *)
        echo ""
        echo "⚠️  Note: $BIN_DIR is not in your PATH."
        echo "Add it to your shell configuration (e.g. ~/.bashrc or ~/.zshrc):"
        echo "    export PATH=\"\$PATH:$BIN_DIR\""
        ;;
esac

echo ""
echo "Run 'satra' to start the application."
