#!/usr/bin/env bash

set -o errexit
set -o pipefail
set -o nounset

echo "Running as: RUST_LOG=debug cargo run"

RUST_LOG=debug cargo run