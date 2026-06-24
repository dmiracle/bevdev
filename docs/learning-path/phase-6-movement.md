# Phase 6 — Grounded First-Person Movement

## Goal

Walk the dungeon instead of flying through it. Closes the long-deferred "grounded movement + floor collider" item.

## The problem today

The camera is a free-fly cam: `get_direction` uses `transform.forward()`/`right()`, which include a Y component, so W while looking up/down flies. There's no notion of a floor or a walking height.

## Concepts

- **Flatten movement to the XZ plane**: take `forward()`/`right()`, zero the Y, re-`normalize_or_zero()`. Now W walks along the ground regardless of where you're looking. Mouse-look stays full 3D (pitch still aims the view up/down) — only *movement* is horizontalized.
- **Fixed eye height**: clamp the camera's Y to a constant (floor height + eye height). Flat dungeon → no gravity, no floor collider needed (see the deferred-item reasoning: a wall-style collider on a flat floor would misbehave with the MTV resolver).
- Wall collision is unchanged — `resolve_collisions` already stops you on the XZ axes.

## Shopping list

Mostly glam math, no new Bevy API:

| Need | Look up |
|---|---|
| Horizontal axes | `Vec3` with `.y = 0.0`, `normalize_or_zero()` |
| Local axes | `Transform::forward()` / `right()` (then flatten) |

## Dungeon tie-in

You traverse the procedurally generated cave at human eye height; walls stop you, the floor supports you implicitly via the Y-clamp.

## Done when

W/A/S/D move you horizontally at a constant height no matter where you look; you can't fly up or sink through the floor; wall collision still slides you. (Gravity + a downward ground-check only become necessary if terrain ever goes uneven/multi-level.)
