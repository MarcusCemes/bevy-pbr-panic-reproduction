# Bevy 0.17.2 GLTF Material Replacement Panic Reproduction

**Update: fixed by [#21410](https://github.com/bevyengine/bevy/pull/21410) in Bevy 0.18.**

## Bug Summary

This reproduction demonstrates a panic that occurs in Bevy when a GLTF scene with dynamically replaced materials comes into view of a camera. The crash happens specifically when:

1. A GLTF scene is spawned in the **PostUpdate** schedule
2. The scene's materials are replaced via the `upgrade_interaction_materials` observer when `SceneInstanceReady` fires
3. The camera moves to view the scene (press Space)
4. The scene enters the camera's view frustum

## Critical Details

- **The bug occurs**: When tiles are spawned in **PostUpdate** schedule
- **The bug does NOT occur**: When tiles are spawned in **Update** schedule
- **Timing**: Crash happens approximately 1 second after launch when Space is pressed
- **Bevy Version**: 0.17.2

```
thread 'Compute Task Pool (19)' panicked at C:\Users\Marcus\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\bevy_pbr-0.17.2\src\render\light.rs:1841:84:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\panicking.rs:697
   1: core::panicking::panic_fmt
             at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\core\src\panicking.rs:75
   2: core::panicking::panic
             at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\core\src\panicking.rs:145
   3: core::option::unwrap_failed
             at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\core\src\option.rs:2072
   4: bevy_pbr::render::light::specialize_shadows
             at C:\Users\Marcus\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\bevy_pbr-0.17.2\src\render\light.rs:1841
   5: core::ops::function::FnMut::call_mut
             at C:\Users\Marcus\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\ops\function.rs:166
   6: core::ops::function::impls::impl$3::call_mut
             at C:\Users\Marcus\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\ops\function.rs:294
   7: bevy_ecs::system::function_system::impl$64::run::call_inner
             at C:\Users\Marcus\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\bevy_ecs-0.17.2\src\system\function_system.rs:926
   8: bevy_ecs::system::function_system::impl$64::run
             at C:\Users\Marcus\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\bevy_ecs-0.17.2\src\system\function_system.rs:929
   9: bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<bevy_pbr::prepass::PrepassPipeline>,tuple$<bevy_ecs::change_detection::Res<bevy_render::render_asset::RenderAssets<bevy_render::mesh::RenderMesh> >,bevy_ecs::ch
             at C:\Users\Marcus\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\bevy_ecs-0.17.2\src\system\function_system.rs:711
  10: core::hint::black_box
             at C:\Users\Marcus\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\hint.rs:482
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
Encountered a panic in system `bevy_pbr::render::light::specialize_shadows`!
```

## Reproduction Steps

1. Run `cargo run`
2. Press **Space** to start camera movement
3. **Crash occurs** when the GLTF scene comes into the camera's view (~1s)

## Environment

- **Rust Edition**: 2024
- **Bevy**: 0.17.2
- **Additional Dependencies**: bevy-inspector-egui 0.34.0 (not essential to reproduce)
- **Platform**: Windows
