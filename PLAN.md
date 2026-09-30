# ControllerParity detailed plan

## Product contract

A portable input parity layer translates backend-specific raw events into a stable semantic controller state, then optionally translates semantic state into game action bindings. Spreadsheet data is the editable registry and mapping source. The first integration target is Bevy. Supporting arbitrary controllers means layered discovery and fallbacks, not relying only on a finite vendor list.

## Milestones

### 0. Scope and conventions
- Lock canonical semantic control names, IDs, schema versions, contribution and verification status.
- Distinguish controller/device metadata, raw backend element mapping, semantic controls, and game actions. Never bind vendor raw button numbers directly in game profiles.
- Define capability reporting: available, unavailable, and unknown; missing hardware controls must not fabricate inputs.

### 1. Spreadsheet registry
- Maintain CSV sheets under `data/` that open in common spreadsheet editors.
- Add schema/version and validation for unique IDs, references, allowed semantic names, numeric ranges, axis center, deadzones, duplicate raw elements, and source/verification fields.
- Build import/export to a normalized versioned JSON registry. Generated files are disposable outputs.
- Add a contribution template, device test checklist, provenance, platform/transport variants, and review process.

### 2. Normalized runtime core
- Device identity and hotplug lifecycle; stable per-session IDs without collecting personal identifiers.
- Canonical button booleans and signed stick axes in [-1,1], trigger values in [0,1], D-pad direction set, optional motion/touch/paddle capabilities.
- Per-axis calibration, inversion, center/range normalization, radial stick deadzone, trigger mode conversion, hysteresis for noisy digital thresholds, and configurable sensitivity.
- Preserve simultaneous inputs, reconnect behavior, focus loss, multiple controllers, and unknown controls; expose raw diagnostics behind an opt-in debug interface.
- Keep backend adapters replaceable. Standard gamepad APIs provide broad generic coverage; database overrides improve unusual layouts and quirks.

### 3. Bevy integration
- Provide a Bevy plugin that consumes gamepad connection/button/axis events and publishes semantic state/events/resources.
- Offer ergonomic action bindings, contexts, rebinding persistence, disconnect reset, and support for multiple players.
- Document Cargo dependency, plugin setup, query examples, and keyboard/mouse coexistence.
- Avoid requiring application-specific code changes beyond plugin registration and mapping actions.

### 4. Compatibility coverage
- Prioritize Xbox/XInput-style, PlayStation, Nintendo Switch-layout, generic HID, wired/wireless variants, and common third-party models based on community reports.
- Capture OS/backend distinctions (Windows, Linux, macOS, browser/WASM where feasible) and connection mode differences. Verify rather than assume vendor/product IDs and layouts.
- Treat motion, touch surfaces, paddles, audio, and vendor extensions as optional capabilities, because transport/backend access differs.
- Publish coverage by tested model, OS, backend, transport, and controls, with clear unverified status.

### 5. Quality and release
- Unit tests for normalization, deadzone, trigger conversion, profile validation, bindings, hotplug reset, and multi-device routing.
- Fixture tests using representative CSV rows and backend event traces; hardware test matrix and contributor instructions.
- Bevy example game, CI across supported targets, schema migration policy, release artifacts and semver.

## Current state (2026-09-30)

Implemented:
- Rust library core with normalized common controls and tested axis/deadzone helpers.
- Optional Bevy 0.19 adapter plugin that reads Bevy's `Gamepad` components, produces semantic state per connected entity, applies configurable deadzone, and preserves `Other` buttons/axes.
- CLI to import SDL GameControllerDB text into a spreadsheet-compatible CSV table and structurally validate tables, SDL GUIDs, duplicate GUID/platform keys, required mapping fields, and row widths.
- Snapshot of 2,278 upstream SDL mapping records in spreadsheet CSV format, with original upstream data and custom license notice retained.
- Quick-start integration guide and research/reuse record.

Not yet implemented: Excel `.xlsx` workbook packaging, runtime loading of the sheet overrides into Bevy/GilRs, semantic game-action bindings/rebinding persistence, custom motion/touch/haptics backends, automatic upstream refresh, expanded profile validation for every custom sheet, and cross-platform physical device testing. The current Bevy plugin uses Bevy/GilRs' built-in normalized mappings; edits to the imported SDL CSV are registry/source data and do not currently mutate GilRs runtime mappings.

## Important architecture decision

A spreadsheet is appropriate for declarative device quirks and game mappings, but it cannot itself provide OS device discovery or input delivery. This implementation delegates discovery and the broad standard gamepad mapping layer to Bevy/GilRs. Bevy systems consume one compact normalized state, and spreadsheet-derived custom mappings can be layered in later without replacing the OS backend. Unknown devices retain generic support when the platform backend recognizes them.
