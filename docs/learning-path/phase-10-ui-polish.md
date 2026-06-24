# Phase 10 — HUD & Polish

## Goal

A HUD reflecting live game state, plus atmospheric rendering.

## Concepts — UI

- Bevy UI is its own entity tree (`Node` + style components), separate from 3D
- Flexbox-style layout; text; buttons
- A grid layout (great for an inventory)
- Driving UI from game state each frame

## Concepts — Polish

- Post-processing: bloom, anti-aliasing
- Skybox / ambient atmosphere
- Lighting tuning for mood

## Shopping list (docs.rs/bevy/0.18.1)

| Need | Look up |
|---|---|
| Layout | `Node`, UI layout/style components |
| Text | text/UI text components |
| Buttons | button + interaction components |
| Post-fx | bloom, anti-aliasing components on the camera |
| Skybox | `Skybox` |

## Dungeon tie-in

Health bar, minimap, inventory grid; bloom + skybox + tuned lights to make the dungeon feel like a place.

## Reference

Examples `button.rs`, `text.rs`, `flex_layout.rs`, `grid.rs`, `bloom_3d.rs`, `anti_aliasing.rs`, `skybox.rs`.

## Done when

The HUD updates from live game state and the scene has deliberate atmosphere instead of flat default lighting.
