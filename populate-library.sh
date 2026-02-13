#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./populate-library.sh
# Optional:
#   DRY_RUN=1 ./populate-library.sh
#   DEPTH=1 ./populate-library.sh

DRY_RUN="${DRY_RUN:-0}"
DEPTH="${DEPTH:-1}"

run() {
  if [[ "$DRY_RUN" == "1" ]]; then
    echo "+ $*"
  else
    eval "$@"
  fi
}

clone_or_update() {
  local url="$1"
  local dir="$2"

  if [[ -d "$dir/.git" ]]; then
    echo "Updating $dir"
    run "git -C \"$dir\" fetch --all --prune"
    run "git -C \"$dir\" pull --ff-only"
  else
    echo "Cloning $dir"
    run "git clone --depth \"$DEPTH\" \"$url\" \"$dir\""
  fi
}

# Use HTTPS so contributors don't need SSH keys configured.
clone_or_update "https://github.com/rust-embedded/awesome-embedded-rust.git" "awesome-embedded-rust"
clone_or_update "https://github.com/esp-rs/awesome-esp-rust.git" "awesome-esp-rust"

# NOTE:
# If you want to vendor rppal as a full repo, keep it here.
# Otherwise, consider switching this to a submodule or removing the vendored copy.
clone_or_update "https://github.com/golemparts/rppal.git" "rppal"

echo "Done."
