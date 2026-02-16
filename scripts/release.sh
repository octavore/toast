#!/usr/bin/env bash
set -euo pipefail

# Usage: ./scripts/release.sh v0.1.0

VERSION="${1:?Usage: $0 <version tag, e.g. v0.1.0>}"
NAME="toast"
TARGET="aarch64-apple-darwin"
DIST="dist"

# Check platform
if [[ "$(uname -m)" != "arm64" ]] || [[ "$(uname -s)" != "Darwin" ]]; then
  echo "error: only Apple Silicon (aarch64-apple-darwin) is supported."
  exit 1
fi

# Check if tag already exists
if git rev-parse "$VERSION" >/dev/null 2>&1; then
  echo "error: tag $VERSION already exists."
  exit 1
fi

# Ensure we have a clean state
if ! git diff --quiet; then
  echo "error: working tree is dirty. Commit or stash changes first."
  exit 1
fi

# Build release binary

# Clean old dist directory
rm -rf "$DIST"
mkdir -p "$DIST"

echo "Building $TARGET..."
cargo build --release --target "$TARGET"

echo "Packaging $TARGET to $DIST/${NAME}-${VERSION}.tar.gz..."
tar -czf "$DIST/${NAME}-${VERSION}.tar.gz" -C "target/${TARGET}/release" "$NAME"

DIST_CHECKSUM=$(shasum -a 256 "$DIST"/*.tar.gz | awk '{print $1}')

# Tag and push
if git rev-parse "$VERSION" >/dev/null 2>&1; then
  echo ""
  echo "Tag $VERSION already exists, skipping tag creation."
else
  git tag "$VERSION"
  # git push origin "$VERSION"
fi

# Create GitHub release with assets
# echo ""
# echo "Creating GitHub release $VERSION..."
# gh release create "$VERSION" "$DIST"/*.tar.gz --title "$VERSION" --generate-notes

echo ""
echo "Release $VERSION created successfully!"
echo "Upload the following artifacts to the GitHub release page and push the tag with 'git push origin $VERSION':"
echo ""
echo "$DIST/${NAME}-${VERSION}.tar.gz"
echo "  version: $VERSION"
echo "  checksum: $DIST_CHECKSUM"
