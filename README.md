# ControllerParity

Spreadsheet-authored controller parity for Bevy games. It wraps Bevy's own gamepad runtime and surfaces a stable semantic state per connected pad. SDL/GilRs community mappings provide wide GUID coverage; spreadsheets hold the upstream map snapshot and community profiles.

## Add it to a Bevy game

```toml
# Cargo.toml
[dependencies]
controller-parity = { path = "../ControllerParity", features = ["bevy"] }
```

```rust
use bevy::prelude::*;
use controller_parity::{ControllerParityPlugin, ControllerParityState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // includes Bevy's input + gamepad backend
        .add_plugins(ControllerParityPlugin)
        .add_systems(Update, player_input)
        .run();
}

fn player_input(pads: Res<ControllerParityState>) {
    for (entity, controls) in &pads.0 {
        if controls.face_south { info!("{entity}: confirm"); }
        let [x, y] = controls.stick_left;
        if x != 0.0 || y != 0.0 { info!("{entity}: move {x:.2}, {y:.2}"); }
    }
}
```

`ControllerParityPlugin` only needs Bevy's `bevy_app`, `bevy_ecs`, and `bevy_input` crates. It adds `ControllerParityState` and a per-frame `HashMap<Entity, ControlState>`. `Entity` is Bevy's gamepad identity for the current connection. Configure `ControllerParityConfig.stick_deadzone` before gameplay (default `0.12`). Standard input names are semantic, not physical button lettering: `face_south` is Xbox A / PlayStation Cross for standard layouts. Buttons use booleans, sticks use signed `[-1,1]`, trigger values use `[0,1]`; extra Bevy buttons and axes remain accessible in `other_buttons` / `other_axes`.

### Bevy version

The optional adapter currently targets Bevy 0.19. Match this feature to the Bevy version used by your game. Bevy makes breaking API changes between releases; compatibility aliases/branches are a planned release task.

## Spreadsheet data and tools

CSV opens in Excel/LibreOffice/Google Sheets. `data/` contains the editable files; `data/gamecontrollerdb-sdl.txt` is the attributed upstream source and `data/sdl_mappings.csv` is its imported spreadsheet view (2,278 mapping rows in this snapshot).

```console
cargo run -- validate data
cargo run -- import-sdl data/gamecontrollerdb-sdl.txt data/sdl_mappings.csv
cargo test
cargo test --features bevy
```

`import-sdl` regenerates the spreadsheet view. Keep source/license attribution with redistributed data. `reported_upstream` indicates database provenance, not hardware testing by this project. Do not overwrite upstream data with guessed mappings. See `data/README.md` for schemas and `RESEARCH.md` for research sources and reuse notes.

## What coverage means

Bevy's built-in backend and GilRs handle device discovery, hotplugging, and standardized controls for many HID devices, applying SDL-compatible mappings. This project does not maintain its own kernel/USB/Bluetooth drivers. The imported database contributes thousands of model/platform mapping records; it does not guarantee all controllers or advanced features such as gyro, touchpad, adaptive triggers, rumble, paddles, split Joy-Con, or platform-specific extensions will work identically. Such controls need backend support plus independently verified profiles. The database is an extensible base, not a claim of exhaustive physical testing.

## Integrating with a coding agent

See [`AGENT_INTEGRATION_GUIDE.md`](AGENT_INTEGRATION_GUIDE.md) for the copy-paste integration prompt, setup instructions, game-side example, and agent validation checklist.

## Project roadmap

See `PLAN.md` for profile validation, XLSX generation, actions/rebinding, backend coverage, physical test matrix, and release milestones. The current adapter gives games semantic state; a declarative game-action binding layer, device-specific override loader, runtime mapping reload, and release-quality CI are still in progress.
