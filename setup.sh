#!/usr/bin/env bash
set -e

echo "🔧 Zahra Invoice Desktop — Setup"
echo "================================"
echo ""

# Check Node.js
if ! command -v node &>/dev/null; then
    echo "❌ Node.js 18+ is required. Install it from https://nodejs.org"
    exit 1
fi
echo "✅ Node.js $(node -v)"

# Check Rust
if ! command -v rustc &>/dev/null; then
    echo "❌ Rust is required. Install it from https://rustup.rs"
    exit 1
fi
echo "✅ Rust $(rustc --version | cut -d' ' -f2)"

# Check system libs (Linux)
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    MISSING=""
    dpkg -s libwebkit2gtk-4.1-dev &>/dev/null || MISSING="$MISSING libwebkit2gtk-4.1-dev"
    dpkg -s libgtk-3-dev &>/dev/null || MISSING="$MISSING libgtk-3-dev"
    dpkg -s libsoup-3.0-dev &>/dev/null || MISSING="$MISSING libsoup-3.0-dev"
    dpkg -s libjavascriptcoregtk-4.1-dev &>/dev/null || MISSING="$MISSING libjavascriptcoregtk-4.1-dev"
    if [ -n "$MISSING" ]; then
        echo ""
        echo "⚠️  Missing system libraries:"
        echo "   sudo apt install$MISSING"
    fi
fi

echo ""
echo "📦 Installing dependencies..."
npm install

echo ""
echo "======================================"
echo "✅ Setup complete!"
echo ""
echo "▶️  Start the app:"
echo "   npm run tauri dev"
echo ""
echo "📦 Build for distribution:"
echo "   npm run tauri build"
