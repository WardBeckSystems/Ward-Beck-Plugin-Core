# Ward-Beck Plugin Core

Every Ward-Beck module plugin, and what they share. Each plugin lives in its own module repo
(`MODULES.<CATEGORY>.<Mxxx>`, folder `SOFTWARE/plugin`) and depends on this repo's panel kit.

## Plugins

| Module | Plugin | Year | Source | Download (VST3 + CLAP: Windows, macOS, Linux) |
|---|---|---|---|---|
| M404 | Hi-Lo Pass Equalizer | 1968 | [MODULES.FILTERS.M404](https://github.com/WardBeckSystems/MODULES.FILTERS.M404/tree/main/SOFTWARE/plugin) | [releases](https://github.com/WardBeckSystems/MODULES.FILTERS.M404/releases/latest) |
| M462 | Equalizer | 1974 | [MODULES.FILTERS.M462](https://github.com/WardBeckSystems/MODULES.FILTERS.M462/tree/main/SOFTWARE/plugin) | [releases](https://github.com/WardBeckSystems/MODULES.FILTERS.M462/releases/latest) |
| M462A | Equalizer | 1976 | [MODULES.FILTERS.M462A](https://github.com/WardBeckSystems/MODULES.FILTERS.M462A/tree/main/SOFTWARE/plugin) | [releases](https://github.com/WardBeckSystems/MODULES.FILTERS.M462A/releases/latest) |
| M462B | Parametric EQ | 1974 | [MODULES.FILTERS.M462B](https://github.com/WardBeckSystems/MODULES.FILTERS.M462B/tree/main/SOFTWARE/plugin) | [releases](https://github.com/WardBeckSystems/MODULES.FILTERS.M462B/releases/latest) |
| M464 | High Pass / Low Pass Filter | 1976 | [MODULES.FILTERS.M464](https://github.com/WardBeckSystems/MODULES.FILTERS.M464/tree/main/SOFTWARE/plugin) | [releases](https://github.com/WardBeckSystems/MODULES.FILTERS.M464/releases/latest) |
| M466 | Compressor - Limiter | 1976 | [MODULES.DYNAMICS.M466](https://github.com/WardBeckSystems/MODULES.DYNAMICS.M466/tree/main/SOFTWARE/plugin) | [releases](https://github.com/WardBeckSystems/MODULES.DYNAMICS.M466/releases/latest) |
| M467 | Noise Gate | 1976 | [MODULES.DYNAMICS.M467](https://github.com/WardBeckSystems/MODULES.DYNAMICS.M467/tree/main/SOFTWARE/plugin) | [releases](https://github.com/WardBeckSystems/MODULES.DYNAMICS.M467/releases/latest) |

Each plugin is one Rust crate: the module's signal path (`dsp`, also run by the
[Ward-Beck Audio Tap](https://github.com/WardBeckSystems/Ward-Beck-Audio-Tap) as an insert) and its front panel,
laid out from the module's own drawings. Every panel carries the WBS logo bottom right (a link to
[ward-beck.ca](https://ward-beck.ca)).

**Formats.** VST3 and CLAP for Windows, macOS (universal) and Linux are public, in each module repo's releases.
Pro Tools formats (AAX Native, AAX DSP, and TDM / RTAS for legacy Pro Tools 9 and 10 systems) are developed
separately and are not public.

**Install.** Unzip the VST3 into `C:\Program Files\Common Files\VST3`, `/Library/Audio/Plug-Ins/VST3` or `~/.vst3`,
and the CLAP into `…\Common Files\CLAP`, `/Library/Audio/Plug-Ins/CLAP` or `~/.clap`. The macOS builds are not
notarised yet: if macOS blocks one, run `xattr -dr com.apple.quarantine` on it.

## Building

```bash
cd SOFTWARE/plugin                              # in a module repo
cargo xtask bundle wbs-m464 --release          # target/bundled/wbs-m464.vst3 and .clap
cargo test --no-default-features               # the DSP's tests
cargo run --release --features standalone --bin wbs-m464-standalone   # the panel without a DAW
```

Inside a [WBSps.ca](https://github.com/WardBeckSystems/WBSps.ca) checkout, `python3 tools/build_plugins.py`
builds and tests every plugin, deploys it into `~/.vst3` and `~/.clap`, and checks that REAPER starts and loads
each one as VST3 and as CLAP.

**Releases.** Each module repo's `.github/workflows/plugin.yml` calls this repo's
[`plugin-release.yml`](.github/workflows/plugin-release.yml): push a tag `plugin-v<version>` in the module repo
and it builds Windows, macOS and Linux and publishes the release.

## In this repo

| Folder | What |
|---|---|
| `panel/` | `wbs-panel`: module front panels for plugin editors, drawn with egui. Anodised panel with silver trims and thumb screws, Elma-style coloured knob caps (green, yellow, red, black, white) with scales, red and white latching push-buttons, red / green / amber LEDs, the WBS edgewise VU, the WBS logo. A module is laid out as on its drawing (x along the module, y across) and shown upright, WBS badge at the top. |
| `art/` | `concept-rack-gemini.svg`: the concept rendering of the M582C / M584 / M456 / M407 rack the look follows (a concept drawing, not archive evidence). |
| `build-shims/linux/` | pkg-config shims for building on a machine without `libx11-xcb-dev` / `libjack-dev` (run `build-shims/linux/links.sh` once, then `PKG_CONFIG_PATH=build-shims/linux LIBRARY_PATH=build-shims/linux/lib`). |
| `.github/workflows/` | `plugin-release.yml`: the release build every module repo calls. |

Plugins are built with [nih-plug](https://github.com/robbert-vdh/nih-plug).

Copyright (c) 2026 Ward-Beck Systems (Ward-Beck.com - Ward-Beck.ca). All rights reserved.
