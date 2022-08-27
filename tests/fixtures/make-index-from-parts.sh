#!/bin/bash

set -eu -o pipefail

parts="tests/fixtures/index-parts"
commit_list=$parts/commit.list

git init
tar -x $parts/init.*.tar
git add . && git commit -m "initial commit"


while read -r commit; do
    patch -p1 < "$parts/$commit.diff"
    git commit -F "$parts/$commit.msg"
done < "$commit_list"