# Phase 8 — Lighting & Atmosphere

## Goal

Make the **empty** dungeon look and feel good to walk through — moody, atmospheric, with depth — before any enemies or items. Right now it's flat-lit with one point light; this phase turns it into a place.

## Concepts (roughly in impact order)

- **Ceiling (do this first)** — the dungeon is currently an open-topped box; without a roof, light/sky leaks in and nothing feels enclosed. Spawn a plane per tile at the wall-top height, **rotated 180° about X** (`Quat::from_rotation_x(PI)`) so its normal faces *down* — otherwise back-face culling makes it invisible from below. Visual-only (no collider), tagged `DungeonTile`, dark/high-roughness material. Height must match the actual (scaled) wall top; eye 1.7 under a 2.0 ceiling is tight, raise walls+ceiling together if cramped.
- **Variable ceiling height** — low tunnels → tall rooms. Mostly visual; floor stays flat so movement/collision are untouched. Needs: (a) **`Map` carries a per-cell `ceiling_height`** — migrate `tiles: Vec<Tile>` to `cells: Vec<Cell>` where `Cell { kind, ceiling_height }` (cleaner than enum-with-data or parallel arrays; reserves room for `floor_height` later). (b) **generation classifies room vs corridor** — post-pass counting open neighbors per floor cell: many open → room → high ceiling, few → tunnel → low. (c) ceiling spawn reads `cell.ceiling_height` instead of a constant.
- **Ambient down + tonemapping** — lower the `AmbientLight` resource so the base isn't a flat fill; a dark dungeon reads as a dungeon. `Camera3d` carries `Tonemapping` + exposure to control the overall look.
- **Torch lights** — warm-colored `PointLight`s (optionally `SpotLight`) placed through the dungeon for pools of light. `shadows_enabled` for cast shadows. Optional: a small system that flickers intensity/color for a fire feel.
- **Distance fog** — `DistanceFog` on the camera: limited visibility into the dark = atmosphere *and* a natural draw-distance cap. Big mood win for enclosed spaces.
- **Bloom + emissive** — `Bloom` on the camera plus `emissive` on `StandardMaterial` makes torch flames / glowing crystals actually glow.
- **PBR material tuning** — `perceptual_roughness` / `metallic` on wall/floor materials instead of flat colors; normal maps for surface detail if your assets have them.
- **(Optional) SSAO** — `ScreenSpaceAmbientOcclusion` adds contact shadowing in corners/crevices; large depth payoff for interiors, some cost.

## Shopping list (docs.rs/bevy/0.18.1)

| Need | Look up |
|---|---|
| Base light | `AmbientLight`, `PointLight`, `SpotLight`, `DirectionalLight` |
| Camera look | `Tonemapping`, exposure, `Bloom` |
| Atmosphere | `DistanceFog`, `FogFalloff` |
| Surfaces | `StandardMaterial` (`emissive`, `perceptual_roughness`, `metallic`, normal map) |
| Corners | `ScreenSpaceAmbientOcclusion` |

## Dungeon tie-in

Place torch lights during generation — every N floor tiles, or in room centers — reusing the per-tile spawn loop and `Res<Map>`. **Watch the light budget:** real-time shadow-casting point lights are expensive and Bevy's clustered renderer caps how many affect a region at once. Place lights *sparingly* (not per tile), and treat "how many lights before it chugs" as the first real use of the profiling doc.

## Reference

Examples (pin to `v0.18.1`): `lighting.rs`, `fog.rs`, `bloom_3d.rs`, `tonemapping.rs`, `ssao.rs`, `pbr.rs`.

## Done when

Walking the empty dungeon feels atmospheric — controlled darkness, warm light pools with shadows, depth fog, glowing sources — rather than uniformly flat-lit. Performance still acceptable (profile if lights get heavy).
