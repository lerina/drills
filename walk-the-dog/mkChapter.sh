#!/bin/sh

# USAGE: ./mkChapter chap<number>

TARGET=$1
mkdir code-along/$TARGET
cp -a src/ www/ README.md Cargo.toml run.sh code-along/$TARGET
tree code-along/$TARGET
git tag $TARGET
git push origin $TARGET

