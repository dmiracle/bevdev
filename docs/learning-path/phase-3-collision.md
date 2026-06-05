# Phase 3 — Walls That Stop You

## Goal

Can't walk through walls.

## Bevy has no physics engine — choose your approach

1. **Manual AABB collision** — best for learning, great for box-shaped dungeon walls. You write the overlap test and resolution yourself. **Start here.**
2. **Avian (`avian3d` crate)** — modern, ECS-native, pure-Rust physics. Add when you want a real character controller, slopes, or dynamic objects.

## Concepts (manual route)

- A custom `Collider` component (store an axis-aligned box / half-extents)
- Box-vs-box overlap test
- Resolving penetration (push the player out along the smallest axis)
- Querying all wall colliders each frame and testing against the player

## Concepts (Avian route, later)

- `avian3d` plugin setup, `RigidBody`, `Collider`, kinematic character-controller pattern
- Add the dependency yourself: `cargo add avian3d`

## Dungeon tie-in

Your dungeon generator (Phase 5) should emit **collision geometry alongside visuals** — same grid produces both a mesh and a collider per wall.

## Done when

You can collide against an arbitrary set of wall entities (queried, not hardcoded), and the player slides along walls instead of stopping dead or clipping through.
