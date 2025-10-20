// Bug Reproduction: Bevy Panic when Camera Views Scene in PostUpdate
//
// This reproduction demonstrates a panic that occurs when:
// 1. A GLTF scene is spawned in PostUpdate schedule
// 2. The camera moves to view the scene
// 3. The scene comes into the camera's view frustum
//
// The bug does NOT occur when tiles are spawned in the Update schedule instead.
// PostUpdate was used to sync with AssetLoader timing in the original game.

use std::f32::consts::FRAC_PI_8;

use bevy::{
    light::light_consts::lux,
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::interaction::{InteractionSink, InteractiveScene};

mod interaction;
mod shaders;

/* === Entrypoint === */

pub fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(create_log_plugin()),
        EguiPlugin::default(),
        WorldInspectorPlugin::new(),
        shaders::ShadersPlugin,
        interaction::InteractionPlugin,
    ))
    .add_systems(Startup, setup_scene)
    .add_systems(Update, move_camera)
    .add_systems(PostUpdate, spawn_tiles)
    .run();
}

/* === Plugins === */

fn create_log_plugin() -> LogPlugin {
    LogPlugin {
        level: Level::INFO,
        filter: "wgpu=error,naga=warn,bevy_panic_reproduction=debug".into(),
        ..default()
    }
}

/* === Definitions === */

/// Shared material handles used across the application
///
/// This resource holds a base StandardMaterial that is used to create
/// InteractMaterials for all interactive objects. When this material is
/// updated, all InteractMaterials should be updated to match. For demo
/// purposes, this resource is not dynamically updated.
#[derive(Resource)]
pub struct SharedHandles {
    pub palette_material: Handle<StandardMaterial>,
}

/* === Systems === */

/// Initial scene setup: creates lighting, camera, and shared material resources
///
/// Sets up:
/// - A directional light with daylight illumination
/// - A perspective camera at (50, 50, 50) looking at origin
/// - Shared material handles resource for interactive objects
fn setup_scene(mut commands: Commands, mut standard_materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(SharedHandles {
        palette_material: standard_materials.add(StandardMaterial::default()),
    });

    commands.spawn(DirectionalLight {
        illuminance: lux::AMBIENT_DAYLIGHT,
        shadows_enabled: true,
        ..default()
    });

    commands.spawn((
        Camera::default(),
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: FRAC_PI_8,
            ..PerspectiveProjection::default()
        }),
        Transform::from_translation(Vec3::splat(50.)).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// Moves the camera when spacebar is pressed (toggle on/off)
///
/// **BUG TRIGGER**: When the camera moves and the GLTF scene comes into view,
/// a panic occurs.
///
/// Press Space to toggle camera movement. The crash typically occurs within
/// a few frames after the camera starts moving toward the spawned scene.
fn move_camera(
    mut active: Local<bool>,
    mut marker_query: Query<&mut Transform, With<Camera>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        *active = !*active;
    }

    if !*active {
        return;
    }

    for mut transform in &mut marker_query {
        transform.translation += Vec3::new(1., 0., 1.);
        debug!("[CAMERA]: {:?}", transform);
    }
}

/// **CRITICAL**: Spawns interactive tiles in PostUpdate schedule
///
/// This system runs in PostUpdate to sync with AssetLoader in the actual game.
/// The bug occurs when this runs in PostUpdate but NOT in Update schedule.
///
/// Spawns after 0.1 seconds (one-time) to simulate delayed loading:
/// 1. An InteractionSink cube - acts as a hitbox for interactive objects
/// 2. A GLTF scene (Runway.glb) linked to the sink via InteractiveScene component
///
/// The InteractiveScene component links the GLTF scene to the sink, and when
/// SceneInstanceReady fires, all StandardMaterials in the scene are replaced
/// with InteractMaterial instances sharing the sink's material handle.
fn spawn_tiles(
    mut commands: Commands,
    mut spawned: Local<bool>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if *spawned || time.elapsed_secs() < 0.1 {
        return;
    }

    *spawned = true;

    let cube = meshes.add(Mesh::from(Cuboid::default()));
    let scene = asset_server.load("Runway.glb#Scene0");

    // Spawn the interaction sink (hitbox) at position (64., 0)
    let sink = commands
        .spawn((
            Mesh3d(cube),
            Transform::from_translation(MapTransform::to_vec3(Vec2::new(64., 0.))),
            InteractionSink::default(),
        ))
        .id();

    // Spawn the GLTF scene linked to the sink at position (64, 0)
    commands.spawn((
        SceneRoot(scene.clone()),
        Transform::from_translation(MapTransform::to_vec3(Vec2::new(64., 0.))),
        InteractiveScene { sink },
    ));

    info!("Scene ready! Press Space to start moving the camera.");
}

/// Utility for converting 2D map coordinates to 3D world space
///
/// Assumes Y-axis is up and Z-axis is negated for typical top-down map coordinates
struct MapTransform;

impl MapTransform {
    pub const fn to_vec3(position: Vec2) -> Vec3 {
        Vec3::new(position.x, 0.0, -position.y)
    }
}
