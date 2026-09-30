# Upstream research and reuse record

Researched 2026-09-30 before implementing the project.

## SDL_GameControllerDB

- Repository: https://github.com/mdqinc/SDL_GameControllerDB
- Data file: `data/gamecontrollerdb-sdl.txt`, fetched from `https://raw.githubusercontent.com/mdqinc/SDL_GameControllerDB/master/gamecontrollerdb.txt` on 2026-09-30.
- License: custom permissive license, preserved verbatim in `data/SDL_GAMECONTROLLERDB_LICENSE.txt`. Attribution/origin must be retained; altered versions must be identified.
- Reuse: format and mappings are used as a broad community-maintained seed. Original rows remain in upstream SDL text format; the importer translates mappings into our semantic spreadsheet format. Do not represent the database as exhaustive, universally verified, or authored by ControllerParity.
- Upstream docs describe a mapping row as GUID, name, field mappings and platform, with SDL2/SDL3 APIs and community submission workflow.

## GilRs

- Repository mirror: https://github.com/Arvamer/gilrs (primary project at https://gitlab.com/gilrs-project/gilrs)
- License: MIT OR Apache-2.0 per upstream README.
- Reuse: no GilRs source copied. Its docs establish that Rust's established gamepad runtime already provides hotplugging, standardized controls, SDL-compatible mappings, and Windows/Linux/macOS support. Bevy's built-in gamepad backend uses GilRs; ControllerParity wraps Bevy's normalized Gamepad component rather than shipping a second competing OS input stack.

## Bevy and Bevy Archie

- Bevy: https://github.com/bevyengine/bevy, dual MIT/Apache-2.0. Inspected the official `v0.19.0/examples/input/gamepad_input.rs` API. The adapter targets Bevy 0.19's `Query<(Entity, &Gamepad)>`, `just_pressed`, and `get(GamepadAxis/GamepadButton)` API.
- Bevy Archie: https://github.com/greysquirr3l/bevy-archie, MIT OR Apache-2.0. Researched its docs as a comprehensive existing option with action mapping, remapping, device profiles, and advanced features. No code copied. It is an existing alternative/inspiration; ControllerParity differentiates through spreadsheet-authored data and a compact semantic state layer.

## Coverage caveat

SDL mapping data improves standardized button/axis recognition across many GUIDs; it does not deliver identical advanced sensor/touch/rumble features across every OS, transport, controller, or Bevy backend. Those require separate backend/profile work and physical validation. Data file counts must be measured during import; imported mappings retain upstream provenance and are labeled reported rather than hardware-tested by us.
