//! InteractMaterial - Extended material for interactive objects
//!
//! This material extends StandardMaterial with custom shader functionality.
//! All InteractMaterials in a group share the same base StandardMaterial,
//! which allows synchronized shader effects across multiple objects.

use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::AsBindGroup,
};

/* === Plugin === */

pub struct InteractShaderPlugin;

impl Plugin for InteractShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<InteractMaterial>::default());
        // .add_systems(
        //     PostUpdate,
        //     create_interact_material.after(AssetEventSystems),
        // );
    }
}

/* === Definitions === */

/// This allows interactive objects to use standard PBR rendering while
/// having access to custom shader extensions for special effects.
pub type InteractMaterial = ExtendedMaterial<StandardMaterial, InteractMaterialExt>;

/// Uses the PBR fallback shader for demonstration purposes.
#[derive(Asset, AsBindGroup, Reflect, Clone, Default)]
pub struct InteractMaterialExt {}

impl MaterialExtension for InteractMaterialExt {}

/* === Systems === */

// Synchronizes all InteractMaterials when the shared palette material changes
//
// Listens for AssetEvent::Added or AssetEvent::Modified on the palette material
// and updates all InteractMaterial instances to use the new base material.
// This ensures all interactive objects stay synchronized with palette changes.
//
// Runs in PostUpdate after AssetEventSystems to ensure asset events are processed.
//
// ! Disabled in this demo, as it does not contribute to the issue
//
// fn create_interact_material(
//     mut messages: MessageReader<AssetEvent<StandardMaterial>>,
//     mut interact_materials: ResMut<Assets<InteractMaterial>>,
//     standard_materials: Res<Assets<StandardMaterial>>,
//     shared_handles: Res<SharedHandles>,
// ) {
//     for message in messages.read() {
//         if let &AssetEvent::Added { id } | &AssetEvent::Modified { id } = message
//             && id == shared_handles.palette_material.id()
//             && let Some(base) = standard_materials.get(&shared_handles.palette_material)
//         {
//             debug!("Propagating palette material changes to InteractMaterials base");
//             for (_, material) in interact_materials.iter_mut() {
//                 material.base = base.clone();
//             }
//         }
//     }
// }
