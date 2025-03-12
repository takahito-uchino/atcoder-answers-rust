for d in abc* tessoku-book; do
  if [ -d "$d" ]; then
    echo "Updating $d..."
    (cd "$d" && cargo update)
  fi
done
