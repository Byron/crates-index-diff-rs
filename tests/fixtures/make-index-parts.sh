#!/bin/bash

set -eu -o pipefail

# due to the prerequisite, this file has to be run by hand to generate the file from which
# a repository can be re-generated later.

root="$(cd "${0%/*}" && pwd)"
out="$root/index-parts"
mkdir -p "$out"
(
  cd "${1:?first argument is the clone of https://github.com/arlosi/crates.io-index}"
  paths="gi/ .github"
  revlist="$root/.tmp.revs"
  { git log --format=format:%H $paths; echo; } | tail -r > "$revlist"

  first_commit="$(head -1 "$revlist")"
  git archive --format tar "$first_commit" $paths > "$out/init.$first_commit.tar"

  commit_list=$out/commit.list
  tail +2 "$revlist" > "$commit_list"
  while read -r commit; do
    git diff "$commit"~1.."$commit" -- $paths > "$out/$commit".diff
    git log --format=%B -n1 "$commit" > "$out/$commit.msg"
  done < "$commit_list"

base_name='reproduce-#19'
base="$out/$base_name"
cat <<EOF > "$base.diff"
diff --git a/al/lo/allowed b/al/lo/allowed
new file mode 100644
index 0000000..b30662b
--- /dev/null
+++ b/al/lo/allowed
@@ -0,0 +1 @@
+{"name":"allowed","vers":"1.0.0","deps":[],"features":{},"cksum":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855","yanked":true}
EOF
  echo "$base_name" >> "$commit_list"
  echo 'reproduce issue #19' >> "$base.msg"

)