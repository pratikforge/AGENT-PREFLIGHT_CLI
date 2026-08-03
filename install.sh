#!/usr/bin/env bash
set -e

REPO="pratikforge/agent-preflight"
INSTALL_DIR="$HOME/.local/bin"
TMP_DIR=$(mktemp -d)

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     TARGET="x86_64-unknown-linux-gnu";;
    Darwin*)    TARGET="x86_64-apple-darwin";;
    *)          echo "Unsupported operating system: ${OS}"; exit 1;;
esac

# Detect Architecture
ARCH="$(uname -m)"
if [ "${ARCH}" != "x86_64" ] && [ "${ARCH}" != "amd64" ]; then
    echo "Unsupported architecture: ${ARCH}. Currently only x86_64 is supported."
    exit 1
fi

echo "Fetching latest release of Agent Preflight for ${OS}..."

# Get latest release from GitHub API
RELEASE_API_URL="https://api.github.com/repos/${REPO}/releases/latest"
if command -v curl >/dev/null 2>&1; then
    RELEASE_DATA=$(curl -fsSL "$RELEASE_API_URL")
elif command -v wget >/dev/null 2>&1; then
    RELEASE_DATA=$(wget -qO- "$RELEASE_API_URL")
else
    echo "Error: curl or wget is required to download the release."
    exit 1
fi

# Parse browser_download_url
# We're using grep/sed to avoid a hard dependency on jq
ASSET_URL=$(echo "$RELEASE_DATA" | grep -oP "(?<=\"browser_download_url\": \")[^\"]*" | grep "${TARGET}.tar.gz" | head -n 1)

if [ -z "$ASSET_URL" ]; then
    # Fallback to standard grep/sed if grep -P is not available (e.g. macOS)
    ASSET_URL=$(echo "$RELEASE_DATA" | grep "\"browser_download_url\": " | grep "${TARGET}.tar.gz" | sed -E 's/.*"([^"]+)".*/\1/' | head -n 1)
fi

if [ -z "$ASSET_URL" ]; then
    echo "Error: Could not find an artifact for ${TARGET} in the latest release."
    exit 1
fi

echo "Downloading ${ASSET_URL}..."
TAR_PATH="${TMP_DIR}/agent-preflight.tar.gz"

if command -v curl >/dev/null 2>&1; then
    curl -fsSL -o "$TAR_PATH" "$ASSET_URL"
else
    wget -qO "$TAR_PATH" "$ASSET_URL"
fi

echo "Installing to ${INSTALL_DIR}..."
mkdir -p "${INSTALL_DIR}"
tar -xzf "$TAR_PATH" -C "${INSTALL_DIR}" agent-preflight

# Clean up
rm -rf "${TMP_DIR}"

echo ""
echo "Agent Preflight installed successfully to ${INSTALL_DIR}/agent-preflight!"

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo -e "\033[33mWarning: ${INSTALL_DIR} is not in your PATH.\033[0m"
    echo "Please add the following line to your ~/.bashrc, ~/.zshrc, or ~/.profile:"
    echo ""
    echo "    export PATH=\"\$PATH:${INSTALL_DIR}\""
    echo ""
fi

echo "Run 'agent-preflight scan .' to get started."
