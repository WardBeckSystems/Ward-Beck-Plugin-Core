#!/bin/sh
# Copyright (c) 2026 Ward-Beck Systems (Ward-Beck.com - Ward-Beck.ca). All rights reserved.
# Links the runtime libraries as the unversioned .so names the linker looks for (build-shims/linux/lib, not in git).
set -e
d=$(dirname "$0")/lib
mkdir -p "$d"
for l in libjack.so.0 libX11-xcb.so.1; do
  p=$(ldconfig -p | awk -v l="$l" '$1==l{print $NF; exit}')
  [ -n "$p" ] && ln -sf "$p" "$d/${l%.*}" || echo "missing $l" >&2
done
