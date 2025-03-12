#!/bin/bash
# remove_sized_chunks.sh
# このスクリプトは、ルート以下のすべての Cargo.toml ファイルから
# [dependencies] セクション内にある "sized-chunks" の記述を削除します。
# macOS 用のsedを使用しています。

find . -type f -name Cargo.toml | while read file; do
  echo "Processing $file..."
  # "sized-chunks" で始まる行を削除
  sed -i.bak '/^[[:space:]]*sized-chunks[[:space:]]*=/d' "$file"
  echo "Finished processing $file. Backup saved as $file.bak"
done

echo "All Cargo.toml files have been updated."
