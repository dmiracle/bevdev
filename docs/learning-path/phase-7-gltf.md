# Phase 7 — Real Art (glTF Models)

> **Status:** glTF *basics* were learned here (load a `.glb`, place a `SceneRoot`, `Collider.offset`). The project then **reverted to primitives** for development velocity — the full art integration (textures, modular tiles, mesh-AABB colliders, orientation) is consolidated into **Phase 12** (`phase-12-art.md`). This doc remains as the basics reference.

## Goal

Replace primitive cubes with actual 3D models.

## Concepts

- `AssetServer` and `asset_server.load("path.glb#Scene0")`
- `SceneRoot` (spawn a loaded scene into the world)
- `Handle<Scene>`, asset load states
- Where assets live: the `assets/` directory at the project root

## Shopping list (docs.rs/bevy/0.18.1)

| Need | Look up |
|---|---|
| Loader | `AssetServer`, `AssetServer::load` |
| Scene spawn | `SceneRoot` |
| Load tracking | asset load state / `AssetServer::get_load_state` |

## Dungeon tie-in

Load a wall/floor/enemy `.glb`, then instance it across the grid your generator produced (swap the primitive spawn for a `SceneRoot` spawn).

## Reference

Examples `load_gltf.rs`, `asset_loading.rs`, `update_gltf_scene.rs`.

## Done when

You can load a `.glb`, place it in the world, and detect when it has finished loading.
