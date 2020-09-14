#! /bin/bash

if ! which montage 2>/dev/null > /dev/null; then
  echo "Needs montage (image magick)"
  exit 1
fi
files=(
  "${@}"
)

montage "${files[@]}" -tile "${#files[@]}x1" -geometry +0+0 out.png
