use bevy::prelude::*;

pub mod interaction;

/* === Plugin === */

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(interaction::InteractShaderPlugin);
    }
}
