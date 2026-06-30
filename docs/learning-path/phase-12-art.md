# Phase 12 — Art & Assets Pass (glTF)

## Why this is its own phase

The glTF rabbit hole (textures, async loading, AABB colliders, modular tile placement, orientation) branched into five hard problems at once — none of which make the *game* good. So the project builds everything on **primitives** (cubes, planes, `Color` materials) through gameplay/audio/UI, then swaps in real art here in one focused pass. Phase 7 proved the glTF basics (load a `.glb`, place a `SceneRoot`); this phase does the full integration.

This swap is cheap because of the **collider-independent-of-art** design: collision is cell-based AABB data, decoupled from how anything looks. Replacing a primitive mesh with a model is a per-spawn change, not a rewrite. Placement in the plan is flexible — could move earlier if art motivation strikes.

## What this phase absorbs (deferred from earlier)

- **glTF models** for walls/floor/ceiling/props (`SceneRoot`), replacing primitives.
- **Textures** — incl. shared-colormap kits (Kenney-style): models auto-load their referenced textures via `SceneRoot`; "gray/flat" is usually lighting, not missing textures. Texturing primitives needs `StandardMaterial` texture slots + repeat sampling for tiling.
- **Modular tile placement** — edge-based or bitmask-tile selection (corridor / corner / junction / room pieces) placed by neighbor pattern + rotation. The downloaded kit (corridor-*, room-*, gate-*, stairs) is built for this.
- **Runtime mesh-AABB colliders** — size `Collider.half_extents`/`offset` from geometry instead of Blender-measuring. Async: needs a load phase (`Loading` state or preload-during-menu) that computes each model's `Aabb` once (`#Mesh0/Primitive0` → `Mesh::compute_aabb()`) into a `ModelBounds` resource. **Rotation caveat:** the resolver is axis-aligned, so rotated thin-slab pieces need `half_extents` axis-swapped per 90° (cube/cell colliders are rotation-invariant and sidestep this).
- **Per-model scale/seating** — models come at arbitrary size/pivot; bake spawn scale into colliders (resolver ignores `Transform` scale).
- Closes the glTF "detect when finished loading" done-when.

## Done when

The dungeon renders with real models + textures, collision still matches the visuals, and primitives are gone (or kept only as a debug fallback).
