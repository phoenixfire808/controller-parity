# Agent-assisted integration guide

This guide is for developers who ask a coding agent to add ControllerParity to an existing Bevy game. It gives the agent a bounded integration task, exact checks, and truthful limits so the agent does not rewrite a project's input architecture unnecessarily.

## 1. Pick the dependency source

Choose one source. Use a local path while trying it out, or Git for another developer's project. Keep the library's `bevy` feature enabled.

```toml
# Local development, when both folders are side by side
controller-parity = { path = "../ControllerParity", features = ["bevy"] }

# Git source, after this repository has been published
controller-parity = { git = "https://github.com/phoenixfire808/controller-parity.git", branch = "main", features = ["bevy"] }
```

The current adapter targets Bevy 0.19. The consuming game's Bevy major/minor release must align. The package depends on the modular `bevy_app`, `bevy_ecs`, and `bevy_input` crates at 0.19. If the game uses a different Bevy version, stop before integrating: check the library's compatibility branch/release or ask to port and test the adapter explicitly. Do not silently add two Bevy versions to one game.

## 2. Register the plugin

Register it once in the app where Bevy's input plugins already run. A normal game using `DefaultPlugins`:

```rust
use bevy::prelude::*;
use controller_parity::{ControllerParityConfig, ControllerParityPlugin, ControllerParityState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ControllerParityPlugin)
        .insert_resource(ControllerParityConfig { stick_deadzone: 0.18 })
        .add_systems(Update, read_controller)
        .run();
}

fn read_controller(pads: Res<ControllerParityState>) {
    for (gamepad_entity, controls) in &pads.0 {
        if controls.face_south {
            info!("{}: confirm", gamepad_entity);
        }
        let [move_x, move_y] = controls.stick_left;
        if controls.dpad[0] { /* up */ }
        if controls.trigger_right > 0.5 { /* right trigger */ }
        let _ = (move_x, move_y);
    }
}
```

If the app has a custom plugin group, confirm that Bevy's `InputPlugin` and gamepad backend (`GilrsPlugin` on supported native targets) are installed before expecting device state. Do not install a second input backend alongside Bevy's backend unless the project's architecture explicitly calls for it.

## 3. Read and customize the semantic state

`ControllerParityState` is a per-frame snapshot keyed by Bevy `Entity` for each currently connected gamepad. Use `pads.get(entity)` to look up one pad, or iterate `pads.0` for local multiplayer.

- Face controls are named by physical position, not letter: `face_south` is typically Xbox A / PlayStation Cross; `face_east` Xbox B / PlayStation Circle; `face_west` Xbox X / PlayStation Square; `face_north` Xbox Y / PlayStation Triangle.
- Shoulders, triggers, stick clicks, D-pad, `menu`, and `view` have semantic fields. Sticks use signed values, usually `[-1, 1]`. Trigger strength is `[0, 1]`.
- Extra standardized-backend buttons/axes are left available in `other_buttons` and `other_axes`. Preserve these if a game supports flight sticks, paddles, or vendor-specific controls.
- Default stick deadzone is `0.12`. To customize it, insert the resource before/after plugin registration. For instance: `app.insert_resource(ControllerParityConfig { stick_deadzone: 0.18 });`.
- Bevy's gamepad entity is a connection identity, not a durable player ID. If the game supports multiplayer, define ownership/seat assignment in the game and handle disconnect/reconnect.

Use the semantic fields at the game layer to connect existing actions (move, aim, confirm, cancel, jump, etc.). Avoid direct dependence on vendor button numbers in gameplay code.

## 4. Prompt to give your coding agent

Paste this into the agent while its working directory is the *game repository*. Replace the dependency source and action names first.

```text
Integrate ControllerParity into this existing Bevy game with the smallest maintainable change.

Dependency source: [local path or published git URL]
ControllerParity feature: bevy
Game Bevy version: [read from Cargo.toml]
Existing player input/action system: [describe or ask you to inspect]
Desired actions to wire: [move, camera, confirm, cancel, jump, ...]

Instructions:
1. Inspect Cargo.toml, the Bevy version, plugin setup, existing input/action code, and local AGENTS.md instructions before editing.
2. Check that this ControllerParity version targets the same Bevy 0.19 line. If it does not, stop and report the mismatch instead of adding duplicate Bevy versions or changing Bevy across the game.
3. Add the dependency with the `bevy` feature. Register `ControllerParityPlugin` once after the existing Bevy input setup. Do not replace, remove, or duplicate the game's keyboard/mouse/action systems.
4. Read `ControllerParityState` from game systems and connect the requested semantic fields to the existing player actions. Keep keyboard/mouse input working, account for multiple gamepads if the game has players/seats, and handle disconnection/reconnection without leaving stuck input.
5. Use `face_south/east/west/north` names rather than assuming a universal A/B/X/Y label. Keep analog trigger and stick values analog. Preserve any project-specific controls from `other_buttons` / `other_axes` where relevant.
6. Make only changes required for the integration. Do not edit ControllerParity's upstream SDL mapping snapshot or claim a physical controller was tested.
7. Run cargo fmt --check and the game's narrowest relevant cargo check/test. Report exact commands and results, files changed, any Bevy version conflict, and hardware/platform behavior not actually tested.
```

## 5. Agent validation checklist

Ask the agent to show these in its final report:

- Resolved `bevy`, `bevy_app`, `bevy_ecs`, and `bevy_input` versions do not introduce duplicate Bevy ECS types.
- `ControllerParityPlugin` is registered exactly once in the correct Bevy app/plugin group.
- Existing keyboard and mouse bindings still work.
- At least one system reads the semantic state without hardcoding controller-specific raw indices.
- If multiplayer is supported, controller-to-player ownership is stable and disconnect state is cleared.
- `cargo fmt --check` and the actual project `cargo check` or integration test were run and passed, or the blocking failure is reported.
- Any real-device claim names the tested controller, OS, connection type, and controls. A compile test is not hardware validation.

## 6. What the package does and does not do today

The current Bevy adapter consumes the `Gamepad` components Bevy publishes and normalizes common controls. Device discovery, hotplug, and standard controller-to-button mapping come from Bevy's gamepad backend and GilRs where used by the platform. The bundled SDL mapping database is a 2,278-row, upstream-attributed spreadsheet/source snapshot and can be opened or regenerated with the CLI.

At this stage, edits to `data/sdl_mappings.csv` do **not** dynamically override Bevy/GilRs runtime mappings. The plugin also does not yet provide action rebinding, automatic player assignment, `.xlsx` generation, or cross-platform custom drivers for gyro, touch surfaces, adaptive triggers, or haptics. For those requirements, ask the agent to identify a compatible backend and make the added support an explicit, separately tested change.

## 7. Local development and checks

From this library repository:

```console
cargo run -- validate data
cargo test
cargo check --features bevy
cargo test --features bevy
cargo fmt -- --check
```

See `README.md`, `PLAN.md`, `data/README.md`, and `RESEARCH.md` for project setup, data provenance, and planned extension work.
