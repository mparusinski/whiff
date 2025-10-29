#!/usr/bin/env bash

set -eu

# This script automates the "Version bump" section

version="$1"

if [[ -z $version ]]; then
  echo "Usage: must supply version as first argument" >&2
  exit 1
fi

git switch -C "release-$version"

sed -i -e "s/^# Upcoming release/# $version/" CHANGELOG.md
