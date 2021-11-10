#!/usr/bin/env bash -x

# deploy demo to gh-pages
git checkout gh-pages
git checkout main -- demo
cp demo/* .
git reset HEAD demo
rm -rf target/ demo/
git commit -am 'update demo'
git push
git checkout main