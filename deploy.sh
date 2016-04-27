#!/bin/bash

rev=$(git rev-parse --short HEAD)

cd stage/_book

git init
git remote add upstream "https://github.com/rust-lang-ja/rust-by-example-ja.git"
git fetch upstream && git reset upstream/gh-pages

# echo "rustbyexample.com" > CNAME
cp -r ../../vendor/gitbook/* gitbook/

touch .

git add -A .

git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
