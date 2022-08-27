#!/bin/bash

set -eu -o pipefail

parts="${1:?first argument is the repoitory root}/tests/fixtures/index-parts"
commit_list=$parts/commit.list

git init
tar -x < $parts/init.*.tar
git add . && git commit -m "initial commit"


while read -r commit; do
    patch -p1 < "$parts/$commit.diff"
    git add .
    git commit -F "$parts/$commit.msg"
done < "$commit_list"
git gc