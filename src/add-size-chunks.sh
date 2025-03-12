#!/bin/bash
# add_sized_chunks.sh
# このスクリプトは、各Cargo.tomlファイルに対して、
# [dependencies] セクション内に sized-chunks = "0.6.5" がなければ追記します。
# ※実行前にバックアップを取ってください。

find . -type f -name Cargo.toml | while read file; do
  echo "Processing $file ..."
  if ! grep -q '^[[:space:]]*sized-chunks[[:space:]]*=' "$file"; then
    # [dependencies] セクションの直後に追記する
    awk '{
      print;
      if ($0 ~ /^\[dependencies\]/) {
        print "sized-chunks = \"0.6.5\"";
      }
    }' "$file" > "$file.tmp" && mv "$file.tmp" "$file"
    echo "Added sized-chunks dependency in $file"
  else
    echo "sized-chunks dependency already exists in $file"
  fi
done

echo "All Cargo.toml files have been updated."
