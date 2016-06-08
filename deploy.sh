#!/usr/bin/env bash

rev=$(git rev-parse --short HEAD)

cd stage/_book
rm -rf rust-by-example
mkdir rust-by-example
mv ./* rust-by-example

# dirty hack for enable syntax highlight in local
sed -i  -e 's!/gitbook/plugins/gitbook-plugin-rust-playpen/mode-rust.js!/rust-by-example/gitbook/plugins/gitbook-plugin-rust-playpen/mode-rust.js!' rust-by-example/gitbook/plugins/gitbook-plugin-rust-playpen/editor.js

git init
git config user.name "joemphilips"
git config user.email "joemphilips@gmail.com"
git remote add upstream "https://$GH_TOKEN@github.com/rust-lang-ja/rust-by-example-ja.git"
git fetch upstream && git reset upstream/gh-pages

# echo "rustbyexample.com" > CNAME
cp -r ../../vendor/gitbook/* gitbook/
echo "rust-lang-ja.org" > CNAME

touch .

git add -A .

git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
