# Ward-Beck Plugin Core

What every Ward-Beck module plugin shares. Each module's plugin lives in its own module repo
(`MODULES.<CATEGORY>.<Mxxx>/plugin`, for example `MODULES.DYNAMICS.M466`) and depends on this.

| Folder | What |
|---|---|
| `panel/` | `wbs-panel`: module front panels for plugin editors, drawn with egui. Anodised panel with silver trims and thumb screws, Elma-style coloured knob caps (green, yellow, red, black, white) with scales, red and white latching push-buttons, red / green / amber LEDs, the WBS edgewise VU. A module is laid out as on the rack drawing (x along the module, y across) and shown upright, WBS badge at the top. |
| `art/` | `concept-rack-gemini.svg`: the concept rendering of the M582C / M584 / M456 / M407 rack the look follows (a concept drawing, not archive evidence). |
| `build-shims/linux/` | pkg-config shims for building on a machine without `libx11-xcb-dev` / `libjack-dev` (run `build-shims/linux/links.sh` once, then `PKG_CONFIG_PATH=build-shims/linux LIBRARY_PATH=build-shims/linux/lib`). |

Plugins are built with [nih-plug](https://github.com/robbert-vdh/nih-plug): VST3 and CLAP from one crate
(`cargo xtask bundle <crate> --release`), plus a standalone window for checking a panel without a DAW.

Copyright (c) 2026 Ward-Beck Systems (Ward-Beck.com - Ward-Beck.ca). All rights reserved.
