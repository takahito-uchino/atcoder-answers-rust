#!/bin/bash
# update_caret_to_ge.sh
# このスクリプトは、ルート以下のすべての Cargo.toml ファイルに対して、
# 依存関係のバージョン指定で、"^" を ">=" に置換します。
# ※ 実行前に必ずバックアップを取ってください。

find . -type f -name Cargo.toml | while read file; do
  echo "Processing $file ..."
  # = "^(...)" を = ">=(...)"
  perl -pi.bak -e 's/=\s*"\^/= ">=/g' "$file"
  echo "Finished processing $file. Backup saved as $file.bak"
done

echo "All Cargo.toml files updated."
