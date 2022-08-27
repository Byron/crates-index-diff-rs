#!/bin/bash

set -eu -o pipefail

parts="tests/fixtures/index-parts"

commit_list=$parts/commit.list
tar

mkdir -p "$out"
(
  cd "${1:?first argument is the clone of https://github.com/arlosi/crates.io-index}"
  path=gi/
  revlist="$root/.tmp.revs"
  { git log --format=format:%H $path; echo; } | tail -r > "$revlist"

  first_commit="$(head -1 "$revlist")"
  git archive --format tar "$first_commit" $path > "$out/init.$first_commit.tar"

  tail +2 "$revlist" > "$commit_list"
  while read -r commit; do
    git diff "$commit"~1.."$commit" -- $path > "$out/$commit".diff
  done < "$commit_list"
)