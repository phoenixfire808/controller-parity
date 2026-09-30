//! Backend-independent semantic input primitives for ControllerParity.
//!
//! A platform backend and spreadsheet loader feed this core. Values are normalized
//! deliberately: sticks use [-1, 1], triggers use [0, 1].

/// Stable, backend-independent controller state for common game controls.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ControlState {
    pub face_south: bool,
    pub face_east: bool,
    pub face_west: bool,
    pub face_north: bool,
    pub shoulder_left: bool,
    pub shoulder_right: bool,
    pub trigger_left: f32,
    pub trigger_right: f32,
    pub stick_left: [f32; 2],
    pub stick_right: [f32; 2],
    pub dpad: [bool; 4], // up, down, left, right
    pub stick_left_press: bool,
    pub stick_right_press: bool,
    pub menu: bool,
    pub view: bool,
    /// Pressed backend-specific buttons that have no standardized Bevy semantic.
    pub other_buttons: Vec<u8>,
    /// Current values of backend-specific axes exposed by Bevy.
    pub other_axes: std::collections::HashMap<u8, f32>,
}

/// Convert an observed raw axis value to a signed normalized value.
/// Invalid ranges and non-finite values safely produce neutral input.
pub fn normalize_axis(raw: f32, minimum: f32, center: f32, maximum: f32, invert: bool) -> f32 {
    if !raw.is_finite()
        || !minimum.is_finite()
        || !center.is_finite()
        || !maximum.is_finite()
        || minimum >= center
        || center >= maximum
    {
        return 0.0;
    }
    let value = if raw < center {
        (raw - center) / (center - minimum)
    } else {
        (raw - center) / (maximum - center)
    };
    let value = value.clamp(-1.0, 1.0);
    if invert {
        -value
    } else {
        value
    }
}

/// Apply a one-dimensional deadzone while preserving the remaining range.
pub fn apply_deadzone(value: f32, deadzone: f32) -> f32 {
    if !value.is_finite() || !deadzone.is_finite() || deadzone >= 1.0 {
        return 0.0;
    }
    let d = deadzone.max(0.0);
    let magnitude = value.abs().clamp(0.0, 1.0);
    if magnitude <= d {
        0.0
    } else {
        value.signum() * ((magnitude - d) / (1.0 - d))
    }
}

/// Convert a signed axis to a trigger value in [0, 1].
pub fn signed_axis_to_trigger(value: f32) -> f32 {
    if value.is_finite() {
        ((value.clamp(-1.0, 1.0) + 1.0) * 0.5).clamp(0.0, 1.0)
    } else {
        0.0
    }
}

#[cfg(feature = "bevy")]
#[derive(bevy_ecs::prelude::Resource, Clone, Copy, Debug)]
pub struct ControllerParityConfig {
    /// Per-axis deadzone applied after Bevy's own gamepad settings.
    pub stick_deadzone: f32,
}

#[cfg(feature = "bevy")]
impl Default for ControllerParityConfig {
    fn default() -> Self {
        Self {
            stick_deadzone: 0.12,
        }
    }
}

/// Snapshot of the semantic controls for each connected Bevy gamepad entity.
#[cfg(feature = "bevy")]
#[derive(bevy_ecs::prelude::Resource, Default)]
pub struct ControllerParityState(
    pub std::collections::HashMap<bevy_ecs::entity::Entity, ControlState>,
);

#[cfg(feature = "bevy")]
impl ControllerParityState {
    /// Look up the current semantic state by Bevy's connected gamepad entity.
    pub fn get(&self, entity: bevy_ecs::entity::Entity) -> Option<&ControlState> {
        self.0.get(&entity)
    }
}

/// Bevy plugin that exposes Bevy's standardized gamepad input as semantic state.
///
/// Bevy's built-in input backend handles OS discovery and common SDL/GilRs mappings.
/// Register this plugin after Bevy's input plugin, which is included by DefaultPlugins.
#[cfg(feature = "bevy")]
pub struct ControllerParityPlugin;

#[cfg(feature = "bevy")]
impl bevy_app::Plugin for ControllerParityPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.init_resource::<ControllerParityConfig>()
            .init_resource::<ControllerParityState>()
            .add_systems(bevy_app::Update, update_bevy_gamepads);
    }
}

#[cfg(feature = "bevy")]
fn update_bevy_gamepads(
    gamepads: bevy_ecs::prelude::Query<(bevy_ecs::entity::Entity, &bevy_input::gamepad::Gamepad)>,
    mut output: bevy_ecs::prelude::ResMut<ControllerParityState>,
    config: bevy_ecs::prelude::Res<ControllerParityConfig>,
) {
    use bevy_input::gamepad::{GamepadAxis as A, GamepadButton as B, GamepadInput};
    output.0.clear();
    for (entity, gamepad) in &gamepads {
        let button = |which| gamepad.pressed(which);
        let analog = |which| gamepad.get(which).unwrap_or(0.0).clamp(0.0, 1.0);
        let mut state = ControlState {
            face_south: button(B::South),
            face_east: button(B::East),
            face_west: button(B::West),
            face_north: button(B::North),
            shoulder_left: button(B::LeftTrigger),
            shoulder_right: button(B::RightTrigger),
            trigger_left: analog(B::LeftTrigger2).max(analog(B::LeftTrigger)),
            trigger_right: analog(B::RightTrigger2).max(analog(B::RightTrigger)),
            stick_left: [
                gamepad.get(A::LeftStickX).unwrap_or(0.0),
                gamepad.get(A::LeftStickY).unwrap_or(0.0),
            ],
            stick_right: [
                gamepad.get(A::RightStickX).unwrap_or(0.0),
                gamepad.get(A::RightStickY).unwrap_or(0.0),
            ],
            dpad: [
                button(B::DPadUp),
                button(B::DPadDown),
                button(B::DPadLeft),
                button(B::DPadRight),
            ],
            stick_left_press: button(B::LeftThumb),
            stick_right_press: button(B::RightThumb),
            menu: button(B::Start),
            view: button(B::Select),
            ..Default::default()
        };
        state.stick_left[0] = apply_deadzone(state.stick_left[0], config.stick_deadzone);
        state.stick_left[1] = apply_deadzone(state.stick_left[1], config.stick_deadzone);
        state.stick_right[0] = apply_deadzone(state.stick_right[0], config.stick_deadzone);
        state.stick_right[1] = apply_deadzone(state.stick_right[1], config.stick_deadzone);
        state.other_buttons = gamepad
            .get_pressed()
            .filter_map(|b| match b {
                B::Other(index) => Some(*index),
                _ => None,
            })
            .collect();
        state.other_axes = gamepad
            .get_analog_axes()
            .filter_map(|input| match input {
                GamepadInput::Axis(A::Other(index)) => {
                    gamepad.get(A::Other(*index)).map(|value| (*index, value))
                }
                _ => None,
            })
            .collect();
        output.0.insert(entity, state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_asymmetric_ranges_and_inversion() {
        assert_eq!(normalize_axis(-2.0, -2.0, 1.0, 5.0, false), -1.0);
        assert_eq!(normalize_axis(3.0, -2.0, 1.0, 5.0, false), 0.5);
        assert_eq!(normalize_axis(3.0, -2.0, 1.0, 5.0, true), -0.5);
    }

    #[test]
    fn invalid_axis_is_neutral() {
        assert_eq!(normalize_axis(f32::NAN, -1.0, 0.0, 1.0, false), 0.0);
        assert_eq!(normalize_axis(0.0, 1.0, 1.0, 1.0, false), 0.0);
    }

    #[test]
    fn deadzone_rescales_and_clamps() {
        assert_eq!(apply_deadzone(0.1, 0.2), 0.0);
        assert!((apply_deadzone(0.6, 0.2) - 0.5).abs() < f32::EPSILON);
        assert_eq!(apply_deadzone(2.0, 0.0), 1.0);
    }

    #[test]
    fn trigger_mapping_is_unit_interval() {
        assert_eq!(signed_axis_to_trigger(-1.0), 0.0);
        assert_eq!(signed_axis_to_trigger(0.0), 0.5);
        assert_eq!(signed_axis_to_trigger(1.0), 1.0);
    }
}

#[cfg(all(test, feature = "bevy"))]
mod bevy_tests {
    use super::*;
    use bevy_input::gamepad::{Gamepad, GamepadAxis, GamepadButton, GamepadInput};

    #[test]
    fn plugin_publishes_standard_and_other_controls_for_each_gamepad() {
        let mut app = bevy_app::App::new();
        app.add_plugins(ControllerParityPlugin);
        let mut pad = Gamepad::default();
        pad.digital_mut().press(GamepadButton::South);
        pad.digital_mut().press(GamepadButton::Other(7));
        pad.analog_mut()
            .set(GamepadInput::Axis(GamepadAxis::LeftStickX), 0.6);
        let entity = app.world_mut().spawn(pad).id();

        app.update();

        let state = app.world().resource::<ControllerParityState>();
        let controls = state
            .get(entity)
            .expect("gamepad state should be published");
        assert!(controls.face_south);
        assert!(controls.other_buttons.contains(&7));
        assert!((controls.stick_left[0] - (0.6 - 0.12) / (1.0 - 0.12)).abs() < 0.0001);
    }
}
