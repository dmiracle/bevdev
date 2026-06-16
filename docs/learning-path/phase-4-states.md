# Phase 4 — Game States & a Menu

## Goal

Main menu → playing → paused → game over, with clean transitions.

> **Note:** the module/plugin split originally slated as Step 0 here was deferred (user's call, 2026-06-10) — it now lives as **Phase 5 Step 0**, where the dungeon generator forces the issue anyway. Phase 4 stays in `main.rs`.

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
