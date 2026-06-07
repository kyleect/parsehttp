#!/usr/bin/env bash
set -euo pipefail

cargo build
cargo doc
cd book && mdbook build