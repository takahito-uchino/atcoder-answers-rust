#!/bin/bash
# update_caret_versions.sh
# このスクリプトは、ルート以下のすべての Cargo.toml ファイルを検索し、
# [dependencies] セクション内の依存関係のバージョン指定で、
# "数字" から始まるものを "^数字" に変更します。
#
# ※ 実行前に必ずバックアップを取ってください

find . -type f -name Cargo.toml | while read file; do
  echo "Processing $file ..."
  perl -0777 -pi.bak -e '
    # [dependencies] セクションを一括で処理
    s{
      (\[dependencies\].*?)(?=\n\[|\z)
    }{
      my $block = $1;
      # 各行で、"= "の後に " 数字" がある場合、"^" を追加する
      $block =~ s/(=\s*")([0-9])/$1^$2/gm;
      $block;
    }egs;
  ' "$file"
  echo "Finished processing $file"
done

echo "All Cargo.toml files updated."
