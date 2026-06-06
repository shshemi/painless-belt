#!/usr/bin/env bash
set -euo pipefail

OWNER=shshemi
REPO=painless-belt
BIN=pb

PREFIX="${PREFIX:-$HOME/.local}"
BIN_DIR="$PREFIX/bin"
BASH_COMP_DIR="$PREFIX/share/bash-completion/completions"
ZSH_COMP_DIR="$PREFIX/share/zsh/site-functions"
FISH_COMP_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/fish/completions"

URL="https://github.com/$OWNER/$REPO/releases/latest/download/$BIN"

echo "==> downloading $BIN from $URL"
mkdir -p "$BIN_DIR"
curl -fsSL --retry 3 -o "$BIN_DIR/$BIN" "$URL"
chmod +x "$BIN_DIR/$BIN"
xattr -d com.apple.quarantine "$BIN_DIR/$BIN" 2>/dev/null || true

echo "==> generating completions"
mkdir -p "$BASH_COMP_DIR" "$ZSH_COMP_DIR" "$FISH_COMP_DIR"
"$BIN_DIR/$BIN" --generate-completion bash > "$BASH_COMP_DIR/pb"
"$BIN_DIR/$BIN" --generate-completion zsh  > "$ZSH_COMP_DIR/_pb"
"$BIN_DIR/$BIN" --generate-completion fish > "$FISH_COMP_DIR/pb.fish"

cat <<EOF

Installed:
  binary       -> $BIN_DIR/$BIN
  bash         -> $BASH_COMP_DIR/pb
  zsh          -> $ZSH_COMP_DIR/_pb
  fish         -> $FISH_COMP_DIR/pb.fish
EOF

case ":$PATH:" in
    *":$BIN_DIR:"*) ;;
    *)
        cat <<EOF

If '$BIN_DIR' is not on your PATH, add to your shell rc:
  export PATH="$BIN_DIR:\$PATH"
EOF
        ;;
esac

if command -v zsh >/dev/null 2>&1 &&
   ! zsh -ic 'print -l -- $fpath' 2>/dev/null | grep -Fxq "$ZSH_COMP_DIR"; then
    cat <<EOF

For zsh completions to be picked up, ensure your .zshrc includes:
  fpath=($ZSH_COMP_DIR \$fpath)
  autoload -U compinit && compinit
EOF
fi
