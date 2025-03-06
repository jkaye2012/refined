#!/usr/bin/env bash
set -e

workdir="$(pwd)"
dirname="$(basename "$workdir")"
if [ "$dirname" != "refined" ]; then
  echo "update-examples.sh must be run from the repository root"
  exit 255
fi

function update_single() {
  cd "$1"
  cargo update
  cd "$workdir"
}

for ex in examples/*; do
  update_single "$ex"
done
