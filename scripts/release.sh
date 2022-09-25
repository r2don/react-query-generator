#!/usr/bin/env sh

error() {
  echo "Error: $1" >&2
  exit 1
}

git switch main || error "Failed to switch to main branch"
git pull || error "Failed to pull latest changes"

yarn version --no-git-tag-version "$@" || error "Failed to bump version"

version=$(node -p "require('./package.json').version")
sed -i '' "s/^version = \".*\"/version = \"$version\"/" ./generator/Cargo.toml
sed -i '' "s/^version = \".*\"/version = \"$version\"/" ./generator/Cargo.lock

git add package.json generator/Cargo.toml generator/Cargo.lock
git commit -m "v$version"
git tag "v$version"

git push --atomic origin main "v$version" || error "Failed to push changes"
