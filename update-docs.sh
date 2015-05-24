#!/bin/sh
cargo clean
cargo doc --no-deps
cd target/doc
git init
git add . -A
git commit -m "Commiting docs to github pages"
git remote add origin https://github.com/ivanceras/rustorm
git checkout -b gh-pages
git push --force origin gh-pages
