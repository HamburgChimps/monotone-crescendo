#!/usr/bin/env bash -x

# deploy demo to gh-pages
git checkout gh-pages
cp -r demo/* .
git commit -am 'update demo'
git push
git checkout main
