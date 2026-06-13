#!/usr/bin/env bash
set -euo pipefail

cargo test
./scripts/tests-requests-lex.sh
./scripts/tests-requests-parse.sh