//! Interactive object system for managing hitboxes and material replacement
//!
//! This module handles the interaction system where:
//! 1. InteractionSink entities act as hitboxes for groups of objects
//! 2. GLTF scenes are linked to sinks via InteractiveScene component
//! 3. When a scene loads, all its StandardMaterials are replaced with InteractMaterial
//!    instances that share the sink's material handle
//!
//! This allows multiple GLTF scenes to share a single material for shader effects
//! while maintaining individual hitboxes for interaction.

use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
    scene::SceneInstanceReady,
};

use crate::{SharedHandles, shaders::interaction::InteractMaterial};

/* === Plugin === */

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(upgrade_interaction_materials);
    }
}

/* === Definitions === */

/// A hitbox entity that acts as the interaction point for a group of objects
///
/// When added to an entity, the `on_add` hook (`setup_interaction_sink`) creates
/// an InteractMaterial handle based on the SharedHandles palette material.
/// Multiple GLTF scenes can reference the same sink, and they will all share
/// this material handle for unified shader effects. When the shared palette
/// material is loaded/modified, it is propagated to all InteractMaterial assets.
#[derive(Component, Reflect, Default)]
#[component(on_add = setup_interaction_sink)]
#[require(Pickable::default())]
pub struct InteractionSink {
    pub material: Handle<InteractMaterial>,
}

/// Links a GLTF scene to an InteractionSink
///
/// When the scene is ready (SceneInstanceReady event), all StandardMaterial
/// components in the scene's hierarchy are replaced with InteractMaterial
/// components using the sink's material handle.
#[derive(Component)]
pub struct InteractiveScene {
    pub sink: Entity,
}

/* === Triggers === */

/// Replaces StandardMaterials with InteractMaterials when scene is ready
///
/// This observer responds to SceneInstanceReady events. When a scene with an
/// InteractiveScene component finishes loading:
/// 1. Gets the linked InteractionSink entity
/// 2. Iterates through all descendants of the scene
/// 3. Replaces the StandardMaterial with the shared InteractMaterial from the sink
fn upgrade_interaction_materials(
    on: On<SceneInstanceReady>,
    mut commands: Commands,
    q_children: Query<&Children>,
    q_interaction_sink: Query<&InteractionSink>,
    q_scene_of: Query<&InteractiveScene>,
    q_standard_material: Query<&MeshMaterial3d<StandardMaterial>>,
) {
    // Get the related InteractionSink for this SceneRoot
    let Ok(interaction_sink) = q_scene_of
        .get(on.entity)
        .and_then(|scene_of| q_interaction_sink.get(scene_of.sink))
    else {
        return;
    };

    // Iterate over the SceneRoot children
    for child in q_children.iter_descendants(on.entity) {
        // Replace StandardMaterial with InteractMaterial from the sink
        if q_standard_material.contains(child) {
            commands
                .entity(child)
                .remove::<MeshMaterial3d<StandardMaterial>>()
                .insert(MeshMaterial3d(interaction_sink.material.clone()));
        }
    }
}

/* === Hooks === */

/// Component hook that initializes InteractionSink when added to an entity
///
/// Creates a new InteractMaterial based on the shared palette material and
/// stores the handle in the InteractionSink component. This material will be
/// shared by all GLTF scenes linked to this sink.
fn setup_interaction_sink(mut world: DeferredWorld, context: HookContext) {
    let shared_handles = world.resource::<SharedHandles>();
    let standard_materials = world.resource::<Assets<StandardMaterial>>();

    // Retrieve the new palette StandardMaterial
    let base = standard_materials
        .get(&shared_handles.palette_material)
        .unwrap()
        .clone();

    // Create a new InteractMaterial using the palette as the base
    let material_handle = world
        .resource_mut::<Assets<InteractMaterial>>()
        .add(InteractMaterial { base, ..default() });

    // Replace the default dummy UUID handle in the InteractionSink
    world
        .entity_mut(context.entity)
        .get_mut::<InteractionSink>()
        .unwrap()
        .material = material_handle;
}
