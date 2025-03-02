#!/usr/bin/env bash

nix flake check
nix run .#refined-example-quickstart
nix run .#refined-example-axum
