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
  echo 'reproduce issue #19' > "$base.msg"

  base_name='reproduce-#20'
  base="$out/$base_name"
  cat <<EOF > "$base.diff"
diff --git a/al/lo/allowed b/al/lo/allowed
deleted file mode 100644
index b30662b..0000000
--- a/al/lo/allowed
+++ /dev/null
@@ -1 +0,0 @@
-{"name":"allowed","vers":"1.0.0","deps":[],"features":{},"cksum":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855","yanked":true}
EOF
  echo "$base_name" >> "$commit_list"
  echo 'reproduce issue #20' > "$base.msg"


  base_name='reproduce-#26-1'
  base="$out/$base_name"
  cat <<EOF > "$base.diff"
diff --git b/an/si/ansi-color-codec a/an/si/ansi-color-codec
new file mode 100644
index 0000000000..f3400fa0eb
--- /dev/null
+++ a/an/si/ansi-color-codec
@@ -0,0 +1,6 @@
+{"name":"ansi-color-codec","vers":"0.2.9","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"a18b40315b9f13d98ae2ee8df35cdb810d696d197f859c15365fb6d34ecbba11","features":{},"yanked":true,"links":null}
+{"name":"ansi-color-codec","vers":"0.3.1","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"b14431739f0b027eede3789fc83f1c13deae067f369e8456ef5d183ddbdf82c4","features":{},"yanked":true,"links":null}
+{"name":"ansi-color-codec","vers":"0.3.2","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"fed46d46cba320856b6a9186d60129dfad6c142a5a6c00c922cfa6990140c26f","features":{},"yanked":true,"links":null}
+{"name":"ansi-color-codec","vers":"0.3.3","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"ad5a24fc4e47f61b52b8b4c6544bb47db648666f5a742b8776cc90a98a4c2459","features":{},"yanked":true,"links":null}
+{"name":"ansi-color-codec","vers":"0.3.4","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"1048f2e4c18e0a4ca910a8f78da2b31c8257f5ffe4922432c85ac1677933722b","features":{},"yanked":false,"links":null}
+{"name":"ansi-color-codec","vers":"0.3.5","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"9b56ec379f35520cdb52f98d3269cd26d59088414fef86817d4f933c06b9374a","features":{},"yanked":false,"links":null}
EOF
  echo "$base_name" >> "$commit_list"
  echo 'reproduce issue #26: create ansi-color-codec file at b49672ff6a2' > "$base.msg"

  base_name='reproduce-#26-2'
  base="$out/$base_name"
  cat <<EOF > "$base.diff"
diff --git a/an/si/ansi-color-codec b/an/si/ansi-color-codec
index f3400fa0eb..ca9114c44d 100644
--- a/an/si/ansi-color-codec
+++ b/an/si/ansi-color-codec
@@ -4,3 +4,4 @@
 {"name":"ansi-color-codec","vers":"0.3.3","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"ad5a24fc4e47f61b52b8b4c6544bb47db648666f5a742b8776cc90a98a4c2459","features":{},"yanked":true,"links":null}
 {"name":"ansi-color-codec","vers":"0.3.4","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"1048f2e4c18e0a4ca910a8f78da2b31c8257f5ffe4922432c85ac1677933722b","features":{},"yanked":false,"links":null}
 {"name":"ansi-color-codec","vers":"0.3.5","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"9b56ec379f35520cdb52f98d3269cd26d59088414fef86817d4f933c06b9374a","features":{},"yanked":false,"links":null}
+{"name":"ansi-color-codec","vers":"0.3.11","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"92d898f94a19fefba0f0b9906376ef7e1be7e542cd958fd0fd4de1c7c2c9818a","features":{},"yanked":false,"links":null}
EOF
  echo "$base_name" >> "$commit_list"
  echo 'reproduce issue #26: updating ansi-color-codec 0.3.11 da97cd0243' > "$base.msg"

  base_name='reproduce-#26-3'
  base="$out/$base_name"
  cat <<EOF > "$base.diff"
diff --git a/an/si/ansi-color-codec b/an/si/ansi-color-codec
index ca9114c44d..36a32366eb 100644
--- a/an/si/ansi-color-codec
+++ b/an/si/ansi-color-codec
@@ -2,6 +2,6 @@
 {"name":"ansi-color-codec","vers":"0.3.1","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"b14431739f0b027eede3789fc83f1c13deae067f369e8456ef5d183ddbdf82c4","features":{},"yanked":true,"links":null}
 {"name":"ansi-color-codec","vers":"0.3.2","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"fed46d46cba320856b6a9186d60129dfad6c142a5a6c00c922cfa6990140c26f","features":{},"yanked":true,"links":null}
 {"name":"ansi-color-codec","vers":"0.3.3","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"ad5a24fc4e47f61b52b8b4c6544bb47db648666f5a742b8776cc90a98a4c2459","features":{},"yanked":true,"links":null}
-{"name":"ansi-color-codec","vers":"0.3.4","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"1048f2e4c18e0a4ca910a8f78da2b31c8257f5ffe4922432c85ac1677933722b","features":{},"yanked":false,"links":null}
+{"name":"ansi-color-codec","vers":"0.3.4","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"1048f2e4c18e0a4ca910a8f78da2b31c8257f5ffe4922432c85ac1677933722b","features":{},"yanked":true,"links":null}
 {"name":"ansi-color-codec","vers":"0.3.5","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"9b56ec379f35520cdb52f98d3269cd26d59088414fef86817d4f933c06b9374a","features":{},"yanked":false,"links":null}
 {"name":"ansi-color-codec","vers":"0.3.11","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"92d898f94a19fefba0f0b9906376ef7e1be7e542cd958fd0fd4de1c7c2c9818a","features":{},"yanked":false,"links":null}
EOF
  echo "$base_name" >> "$commit_list"
  echo 'reproduce issue #26: yanking ansi-color-codec 0.3.4 1533f8e863' > "$base.msg"

  base_name='reproduce-#26-4'
  base="$out/$base_name"
  cat <<EOF > "$base.diff"
diff --git a/an/si/ansi-color-codec b/an/si/ansi-color-codec
index 36a32366eb..f24049269d 100644
--- a/an/si/ansi-color-codec
+++ b/an/si/ansi-color-codec
@@ -3,5 +3,5 @@
 {"name":"ansi-color-codec","vers":"0.3.2","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"fed46d46cba320856b6a9186d60129dfad6c142a5a6c00c922cfa6990140c26f","features":{},"yanked":true,"links":null}
 {"name":"ansi-color-codec","vers":"0.3.3","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"ad5a24fc4e47f61b52b8b4c6544bb47db648666f5a742b8776cc90a98a4c2459","features":{},"yanked":true,"links":null}
 {"name":"ansi-color-codec","vers":"0.3.4","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"1048f2e4c18e0a4ca910a8f78da2b31c8257f5ffe4922432c85ac1677933722b","features":{},"yanked":true,"links":null}
-{"name":"ansi-color-codec","vers":"0.3.5","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"9b56ec379f35520cdb52f98d3269cd26d59088414fef86817d4f933c06b9374a","features":{},"yanked":false,"links":null}
+{"name":"ansi-color-codec","vers":"0.3.5","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"9b56ec379f35520cdb52f98d3269cd26d59088414fef86817d4f933c06b9374a","features":{},"yanked":true,"links":null}
 {"name":"ansi-color-codec","vers":"0.3.11","deps":[{"name":"clap","req":"^4.0.23","features":["derive"],"optional":false,"default_features":true,"target":null,"kind":"normal"},{"name":"ctrlc","req":"^3.2.3","features":[],"optional":false,"default_features":true,"target":null,"kind":"normal"}],"cksum":"92d898f94a19fefba0f0b9906376ef7e1be7e542cd958fd0fd4de1c7c2c9818a","features":{},"yanked":false,"links":null}
EOF
  echo "$base_name" >> "$commit_list"
  echo 'reproduce issue #26: yanking ansi-color-codec 0.3.5 92c18bdf30' > "$base.msg"
)