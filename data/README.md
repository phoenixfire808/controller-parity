# Spreadsheet schema

CSV sheets are the human-editable source of truth. Use UTF-8 and edit with Excel, LibreOffice, or Google Sheets. The `sdl_mappings.csv` sheet is generated from the upstream SDL database by the importer; keep the upstream text file and license alongside it, and never claim the imported `reported_upstream` data was independently hardware-tested by ControllerParity.

## controllers.csv
`controller_id,display_name,vendor,product,platforms,connection_types,profile_id,layout,verification,source,notes`

## controls.csv
`profile_id,raw_kind,raw_index,semantic,minimum,center,maximum,invert,deadzone,trigger_mode,notes`

Raw kind: `button`, `axis`, `hat`, `sensor`, `vendor`. Semantic vocabulary includes face buttons, shoulders, triggers, two sticks and stick clicks, D-pad directions, menu/view/home, capture, paddles, touchpad and motion axes. Use `vendor:<name>` for extension controls; record source and validation status. Ranges are raw observed values and deadzone is 0..1.

## games.csv
`game_id,display_name,engine,platforms,notes`

## game_bindings.csv
`game_id,action_id,semantic,scale,contexts,notes`

## sdl_mappings.csv
`guid,name,platform,mapping,source,verification`

`mapping` retains SDL binding tokens as a quoted CSV cell. GUID/platform pairs can have distinct rows. Mapping entries are community/upstream-reported data and preserve source attribution. The runtime integration currently consumes standardized Bevy/GilRs controls. The table is provided for audit, spreadsheet edits, and future profile override tooling.

## Data contributor process

Use stable IDs. Do not infer input indices from marketing model names. Prefer official reports, upstream mapping rows, or repeatable tests on named OS + connection type. Mark unverified data honestly and note hardware/firmware variation. Avoid personal identifiers and redact serial numbers.
