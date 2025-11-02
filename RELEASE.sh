#!/bin/bash
# Quick release script for pq-jwt
# Usage: ./RELEASE.sh 0.2.0

set -e

VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.2.0"
    exit 1
fi

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 Starting release process for version ${VERSION}${NC}\n"

# Check if on trunk branch
BRANCH=$(git branch --show-current)
if [ "$BRANCH" != "trunk" ]; then
    echo -e "${RED}❌ Error: Not on trunk branch (currently on: $BRANCH)${NC}"
    echo "Switch to trunk: git checkout trunk"
    exit 1
fi

# Check if working directory is clean
if [ -n "$(git status --porcelain)" ]; then
    echo -e "${RED}❌ Error: Working directory not clean${NC}"
    echo "Commit or stash your changes first"
    exit 1
fi

# Pull latest changes
echo -e "${YELLOW}📥 Pulling latest changes...${NC}"
git pull origin trunk

# Run tests
echo -e "${YELLOW}🧪 Running tests...${NC}"
cargo test --all-features || {
    echo -e "${RED}❌ Tests failed${NC}"
    exit 1
}

# Check formatting
echo -e "${YELLOW}🎨 Checking formatting...${NC}"
cargo fmt -- --check || {
    echo -e "${RED}❌ Code not formatted. Run: cargo fmt${NC}"
    exit 1
}

# Run clippy
echo -e "${YELLOW}📎 Running clippy...${NC}"
cargo clippy -- -D warnings || {
    echo -e "${RED}❌ Clippy warnings found${NC}"
    exit 1
}

# Update version in Cargo.toml
echo -e "${YELLOW}📝 Updating Cargo.toml version...${NC}"
CURRENT_VERSION=$(grep "^version = " Cargo.toml | head -1 | cut -d'"' -f2)
echo "Current version: $CURRENT_VERSION"
echo "New version: $VERSION"

if [ "$CURRENT_VERSION" == "$VERSION" ]; then
    echo -e "${RED}❌ Error: Version $VERSION is the same as current version${NC}"
    exit 1
fi

# Update version in Cargo.toml
sed -i.bak "s/^version = \"$CURRENT_VERSION\"/version = \"$VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Update Cargo.lock
cargo update -p pq-jwt

# Dry run publish
echo -e "${YELLOW}🏗️  Dry run publish...${NC}"
cargo publish --dry-run || {
    echo -e "${RED}❌ Dry run publish failed${NC}"
    # Revert changes
    sed -i.bak "s/^version = \"$VERSION\"/version = \"$CURRENT_VERSION\"/" Cargo.toml
    rm Cargo.toml.bak
    cargo update -p pq-jwt
    exit 1
}

# Show diff
echo -e "${YELLOW}📋 Changes to be committed:${NC}"
git diff Cargo.toml Cargo.lock

# Confirm
echo ""
read -p "$(echo -e ${YELLOW}Continue with release? [y/N]: ${NC})" -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${RED}❌ Release cancelled${NC}"
    # Revert changes
    sed -i.bak "s/^version = \"$VERSION\"/version = \"$CURRENT_VERSION\"/" Cargo.toml
    rm Cargo.toml.bak
    cargo update -p pq-jwt
    exit 1
fi

# Commit changes
echo -e "${YELLOW}💾 Committing changes...${NC}"
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to $VERSION"

# Create tag
echo -e "${YELLOW}🏷️  Creating tag v$VERSION...${NC}"
git tag -a "v$VERSION" -m "Release version $VERSION"

# Push changes
echo -e "${YELLOW}📤 Pushing to GitHub...${NC}"
git push origin trunk
git push origin "v$VERSION"

echo ""
echo -e "${GREEN}✅ Release process completed!${NC}"
echo ""
echo "Next steps:"
echo "1. Monitor GitHub Actions: https://github.com/MKSinghDev/pq-jwt-rust/actions"
echo "2. Check crates.io: https://crates.io/crates/pq-jwt"
echo "3. Verify GitHub Release: https://github.com/MKSinghDev/pq-jwt-rust/releases"
echo ""
echo -e "${GREEN}🎉 Version $VERSION will be published automatically!${NC}"
