# Phase 6 — Grounded First-Person Movement

## Goal

Add a **grounded (Walk) movement mode** alongside the existing **fly mode**, selectable by a `MovementMode` on the camera. Keep both; a runtime toggle between them is **out of scope** (future). Closes the long-deferred "grounded movement + floor collider" item.

## Design: two modes, one branch

- `enum MovementMode { Fly, Walk }`, stored as a `pub` field on `CameraController` (set at spawn in `world.rs`). The future toggle is just `controller.mode = ...` — nothing else changes.
- `camera_controller` branches on the mode:
  - **Fly** — current behavior, unchanged (full-3D `forward()`/`right()`, free Y).
  - **Walk** — flatten the movement direction to XZ (`dir.y = 0.0`, `normalize_or_zero()`), then pin `translation.y` to a constant eye height (floor 0 + ~1.7).
- `get_direction` stays as-is (returns raw 3D dir); the flatten + Y-pin live in `camera_controller`.
- Mouse-look is full 3D in **both** modes — only *movement* is horizontalized in Walk.

## Concepts

- **Flatten movement to the XZ plane**: zero the Y of the movement dir, re-`normalize_or_zero()`. Looking straight down + W → near-zero XZ → `normalize_or_zero` returns zero (no move, no NaN), which is correct.
- **Fixed eye height (Walk)**: clamp the camera's Y to a constant. Flat dungeon → no gravity, no floor collider needed (a wall-style collider on a flat floor would misbehave with the MTV resolver).
- Wall collision is unchanged in both modes — `resolve_collisions` already stops you on the XZ axes.

## Shopping list

Mostly glam math, no new Bevy API:

| Need | Look up |
|---|---|
| Horizontal axes | `Vec3` with `.y = 0.0`, `normalize_or_zero()` |
| Local axes | `Transform::forward()` / `right()` (then flatten) |

## Dungeon tie-in

You traverse the procedurally generated cave at human eye height; walls stop you, the floor supports you implicitly via the Y-clamp.

## Done when

In **Walk** mode: W/A/S/D move you horizontally at a constant height no matter where you look; you can't fly up or sink through the floor; wall collision still slides you. **Fly** mode still behaves exactly as before. (Gravity + a downward ground-check only become necessary if terrain ever goes uneven/multi-level. Runtime toggle between modes is a future task.)
