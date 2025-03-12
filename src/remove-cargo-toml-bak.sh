#!/bin/bash
# remove_cargo_toml_bak.sh
# このスクリプトは、ルート以下のすべての Cargo.toml.bak ファイルを削除します。

echo "Removing all Cargo.toml.bak files..."
find . -type f -name "Cargo.toml.bak" -print -exec rm {} \;
echo "All Cargo.toml.bak files have been removed."
