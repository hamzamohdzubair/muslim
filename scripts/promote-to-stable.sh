#!/bin/bash
# Promote current pre-release to stable

set -e

CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

if [[ ! $CURRENT_VERSION =~ -alpha|-beta|-rc ]]; then
    echo "Current version ($CURRENT_VERSION) is already stable!"
    exit 1
fi

# Remove pre-release suffix
STABLE_VERSION="${CURRENT_VERSION%%-*}"

echo "Current version: $CURRENT_VERSION"
echo "Stable version:  $STABLE_VERSION"
echo ""
read -p "Promote to stable? [y/N]: " confirm

if [[ $confirm != [yY] ]]; then
    echo "Cancelled"
    exit 0
fi

# Update version
sed -i "s/^version = \".*\"/version = \"$STABLE_VERSION\"/" Cargo.toml

echo ""
echo "Running tests..."
cargo test

echo "Building release..."
cargo build --release

echo ""
echo "Ready to publish version $STABLE_VERSION (same code as $CURRENT_VERSION)"
read -p "Publish to crates.io? [y/N]: " publish

if [[ $publish == [yY] ]]; then
    echo "Publishing..."
    cargo publish

    echo ""
    echo "Creating git tag..."
    git tag -a "v$STABLE_VERSION" -m "Stable release $STABLE_VERSION"

    echo ""
    echo "✓ Published $STABLE_VERSION to crates.io"
    echo "✓ Created git tag v$STABLE_VERSION"
    echo ""
    echo "Old alpha: $CURRENT_VERSION (still exists on crates.io)"
    echo "New stable: $STABLE_VERSION (same code!)"
    echo ""
    echo "Next steps:"
    echo "  git push origin v$STABLE_VERSION"
    echo "  cargo install muslim  # Users get stable $STABLE_VERSION"
    echo ""
    read -p "Yank the old pre-release version? [y/N]: " yank
    if [[ $yank == [yY] ]]; then
        cargo yank --version "$CURRENT_VERSION"
        echo "✓ Yanked $CURRENT_VERSION"
    fi
else
    echo "Dry run. To publish manually:"
    echo "  cargo publish"
    echo ""
    echo "To revert version change:"
    echo "  git checkout Cargo.toml"
fi
