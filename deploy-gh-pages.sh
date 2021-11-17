#!/usr/bin/env bash -x

# deploy demo to gh-pages
git checkout gh-pages
rm -rf doc/ monotone_crescendo.wasm index.html
cp -r demo/* .
git add -A doc
git commit -am 'update demo'
git push
git checkout main
