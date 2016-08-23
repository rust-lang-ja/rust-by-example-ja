#!/bin/sh

# This script publishes the contents of `./stage/_book` directory of
# the `master` branch to `origin/gh-pages` branch.
#
# Requirements:
# - CI must be configured with write access to the git remote `origin`.
# - The current directory must be the top directory of the CircleCI
#   project.
# - The current git branch must be `master` branch. (See circle.yml's
#   deployment -> branch)

set -ex

# Get the revision of this branch (master branch)
REVISION=$(git rev-parse --short HEAD)

# Recreate the worktree for gh-pages
rm -rf ./gh-pages || true
git worktree prune || true
git branch -D gh-pages 2> /dev/null || true
mkdir -p ./gh-pages
git branch gh-pages origin/gh-pages
git worktree add ./gh-pages gh-pages

make clean && make && make book

# Copy the contents of stage/_book to gh-pages/rust-by-example
cp -rp ./stage/_book/ ./gh-pages/rust-by-example

cd ./gh-pages

# Dirty hack to enable syntax highlight in local
sed -i -e 's!/gitbook/plugins/gitbook-plugin-rust-playpen/mode-rust.js!/rust-by-example/gitbook/plugins/gitbook-plugin-rust-playpen/mode-rust.js!' ./rust-by-example/gitbook/plugins/gitbook-plugin-rust-playpen/editor.js

cp -r ../gitbook/* rust-by-example/gitbook/

# Set the custom domain
echo "rust-lang-ja.org" > CNAME

# Create circle.yml to disable CI on gh-pages branch.
cat > circle.yml <<EOF
general:
  branches:
    ignore:
      - gh-pages
EOF

# If there are anything to commit, do `git commit` and `git push`
git add .
set +e
ret=$(git status | grep -q 'nothing to commit'; echo $?)
set -e
if [ $ret -eq 0 ] ; then
    echo "Nothing to push to gh-pages."
else
    git commit -m "ci: publish pages at ${REVISION}"
    echo "Pushing to gh-pages..."
    git push -f origin gh-pages
fi
