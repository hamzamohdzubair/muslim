#!/bin/bash
# Helper script to bump version and publish

set -e

CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

echo "Current version: $CURRENT_VERSION"
echo ""
echo "What kind of release?"
echo "  1) Alpha (e.g., 0.1.0-alpha.2)"
echo "  2) Beta (e.g., 0.1.0-beta.1)"
echo "  3) Stable (e.g., 0.1.0)"
echo "  4) Custom version"
read -p "Choice [1-4]: " choice

case $choice in
    1)
        read -p "Enter alpha number: " num
        NEW_VERSION="${CURRENT_VERSION%-*}-alpha.$num"
        ;;
    2)
        read -p "Enter beta number: " num
        NEW_VERSION="${CURRENT_VERSION%-*}-beta.$num"
        ;;
    3)
        NEW_VERSION="${CURRENT_VERSION%-*}"
        ;;
    4)
        read -p "Enter version: " NEW_VERSION
        ;;
    *)
        echo "Invalid choice"
        exit 1
        ;;
esac

echo ""
echo "Updating version from $CURRENT_VERSION to $NEW_VERSION"
sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml

echo "Running tests..."
cargo test

echo "Building release..."
cargo build --release

echo ""
echo "Ready to publish version $NEW_VERSION"
read -p "Publish to crates.io? [y/N]: " confirm

if [[ $confirm == [yY] ]]; then
    echo "Publishing..."
    cargo publish

    echo ""
    echo "Creating git tag..."
    git tag -a "v$NEW_VERSION" -m "Release version $NEW_VERSION"

    echo ""
    echo "✓ Published $NEW_VERSION to crates.io"
    echo "✓ Created git tag v$NEW_VERSION"
    echo ""
    echo "Next steps:"
    echo "  git push origin v$NEW_VERSION"
    echo "  cargo install muslim --version $NEW_VERSION"
else
    echo "Dry run only. To publish manually:"
    echo "  cargo publish"
fi
