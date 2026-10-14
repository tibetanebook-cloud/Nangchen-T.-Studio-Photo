#!/usr/bin/env bash
# Source this file to export the same verified font used by the Windows package.
set -euo pipefail
font_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
font_dir="$font_root/.build-tools/tibetan-font"
mkdir -p "$font_dir"
font_base='https://raw.githubusercontent.com/google/fonts/main/ofl/notoseriftibetan'
fetch_font_input() {
  local url="$1" dest="$2" expected="$3" actual
  if [ ! -f "$dest" ]; then
    curl --fail --location --retry 3 "$url" --output "$dest"
  fi
  if command -v sha256sum >/dev/null; then
    actual="$(sha256sum "$dest" | awk '{print $1}')"
  else
    actual="$(shasum -a 256 "$dest" | awk '{print $1}')"
  fi
  if [ "$actual" != "$expected" ]; then
    echo "Tibetan font input checksum mismatch: $dest" >&2
    return 1
  fi
}
fetch_font_input "$font_base/NotoSerifTibetan%5Bwght%5D.ttf" "$font_dir/NotoSerifTibetan.ttf" \
  060ec022b04c306de3f58d051fb0e1cf81a5b610c5910fbfc43bba154c057cda
fetch_font_input "$font_base/OFL.txt" "$font_dir/OFL.txt" \
  0077789fc3cb20f358278ba4c6437eb14ade97d4d3e3281eaa42853efcd9dc8b
export TIBETAN_UI_FONT="$font_dir/NotoSerifTibetan.ttf"
