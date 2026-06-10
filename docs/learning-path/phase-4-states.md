# Phase 4 — Game States & a Menu

## Goal

Main menu → playing → paused → game over, with clean transitions.

## Step 0 — Split `main.rs` into module-per-concern plugins

Do this first, while the code still compiles and behaves (don't mix it with new features):

- Target layout: `main.rs` (App construction only), `camera.rs`, `collision.rs`, `world.rs` — one module per concern, each exposing a `pub struct FooPlugin` with `impl Plugin` whose `build` registers that module's systems/resources.
- Rust module concepts: `mod foo;` declarations, private-by-default visibility (`pub` only what crosses the boundary), `use crate::...` paths, per-module `use bevy::prelude::*;`.
- Cross-plugin ordering: replace the in-`main` `.chain()` with `.after(camera::camera_controller)` on `resolve_collisions` (system sets later in this phase are the scalable version).
- Method: create files, cut-paste the existing clusters, then follow `cargo check` errors until quiet — the compiler walks you through every needed `mod`/`pub`/`use`.

**Step 0 done when:** `main()` only adds plugins, behavior is unchanged, and nothing is `pub` that doesn't need to be.

## Concepts

- `States` (a state enum) and the `init_state` / state-management API
- `OnEnter(State)` / `OnExit(State)` schedules
- Run conditions: `in_state(...)`, `.run_if(...)`
- Spawning level entities on enter, despawning them on exit

## Shopping list (docs.rs/bevy/0.18.1)

| Need | Look up |
|---|---|
| State type | `States` derive, `State<T>`, `NextState<T>` |
| Enter/exit | `OnEnter`, `OnExit` |
| Conditional systems | `in_state`, `run_if` |

## Pattern

Tag everything you spawn for a state with a marker component, then a single "cleanup" system on `OnExit` despawns all entities with that marker.

## Dungeon tie-in

Entering `Playing` generates + spawns the dungeon; leaving it tears the whole level down so a new run starts fresh.

## Reference

Examples `game_menu.rs`, `loading_screen.rs`, `run_conditions.rs`.

## Done when

You can move between states from input, and each transition cleanly spawns/despawns the right entities with no leftovers.
