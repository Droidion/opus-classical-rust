#!/usr/bin/env bash
# exit on error
set -o errexit

# Build and run app
pnpm i
pnpm i -g sass
pnpm run sass
pnpm run build
cargo build --release