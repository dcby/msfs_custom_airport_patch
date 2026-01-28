# MSFS Custom Airport Patch Compatibility Patcher

This tool is intended to patch metadata for addons that enhance custom airports
(such as statics) but built with MSFS 2020 SDK. This makes such addons
compatible and working correctly with MSFS 2024.

Effectively it patches addon's `manifest.json` file by adding the
`"package_order_hint": "CUSTOM_AIRPORT_PATCH"` key value.

## Building from source

In most cases you don't have to build the tool yourself. Simply use binary that
is available in the Releases section.

Otherwise make sure you have Rust installed, clone, cd and run:

```sh
cargo build --release
```

## Simple usage

Drag and drop desired `manifest.json` or its' directory to the
`msfs_custom_airport_patch.exe` in Windows Explorer.

## Usage

Specify path to `manifest.json` or path to directory that contains
`manifest.json` as an argument.

```powershell
.\msfs_custom_airport_patch.exe <PATH_TO_FILE_OR_DIR>
```

It is possible to specify multiple paths to process several addons at once.

```powershell
.\msfs_custom_airport_patch.exe <PATH_TO_FILE_OR_DIR> [PATH_TO_FILE_OR_DIR]...
```

## Notes

Manifest file is patched only if it contains `"content_type": "SCENERY"` key
value and not already contains `package_order_hint` key.

The file `manifest.json.bak` is created during patching.
