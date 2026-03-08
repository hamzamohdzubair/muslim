#!/bin/bash
# Quick script to reinstall local version

set -e

echo "Uninstalling old version..."
cargo uninstall muslim 2>/dev/null || echo "Not currently installed"

echo "Building and installing from local path..."
cargo install --path .

echo ""
echo "✓ Installed successfully!"
echo ""
echo "Test it:"
echo "  muslim"
echo "  muslim --help"
