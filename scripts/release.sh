#!/usr/bin/env sh

error() {
  echo "Error: $1" >&2
  exit 1
}

git switch main || error "Failed to switch to main branch"
git pull || error "Failed to pull latest changes"

yarn version "$1" || error "Failed to bump version"

version=$(node -p "require('./package.json').version")
sed -i '' "s/^version = \".*\"/version = \"$version\"/" ./generator/Cargo.toml
sed -i '' "s/^version = \".*\"/version = \"$version\"/" ./generator/Cargo.lock

git add .
git commit --amend --no-edit

git push
