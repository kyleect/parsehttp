#!/usr/bin/env bash
set -euo pipefail

./scripts/tests-requests-lex.sh --update
./scripts/tests-requests-parse.sh --update
