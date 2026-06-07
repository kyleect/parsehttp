#!/usr/bin/env bash
set -euo pipefail

cargo clean
cd book && mdbook clean
