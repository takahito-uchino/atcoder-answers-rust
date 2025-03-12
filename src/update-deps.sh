#!/bin/bash
# update_versions.sh
# ルート以下の各 Cargo.toml ファイルについて、以下を行います：
# ・ "nalgebra = "=0.20.0"" を "nalgebra = "^0.27.1"" に更新
# ・ "smallvec = "=1.2.0"" を "smallvec = "^1.6.1"" に更新
# ・ 依存関係のその他の "=X.Y.Z" 指定を "^X.Y.Z" に置換（ただし、[package] セクションの version は除外）

find . -type f -name Cargo.toml | while read file; do
    echo "Updating $file ..."
    
    # nalgebra と smallvec の個別更新
    perl -pi -e 's/(nalgebra\s*=\s*")=0\.20\.0(")/$1^0.27.1$2/g' "$file"
    perl -pi -e 's/(smallvec\s*=\s*")=1\.2\.0(")/$1^1.6.1$2/g' "$file"
    
    # その他の依存関係について、行の先頭が "version ="（パッケージのversion指定）ではない行で置換
    perl -pi -e 'unless (/^\s*version\s*=/) { s/=\s*"(=)?([0-9]+\.[0-9]+\.[0-9]+)"/= "^\2"/g }' "$file"
done

echo "All Cargo.toml files have been updated."
