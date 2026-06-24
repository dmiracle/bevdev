# Bevy Learning Path — Overview

A staged pathway for building a **3D free-roam dungeon crawler** while learning Bevy from zero.

## Project facts

- **Engine:** Bevy `0.18` (resolves to 0.18.1), Rust edition 2024
- **Game:** 3D free-roam (not grid/step) dungeon crawler
- **Camera target:** grounded first/third-person free movement
- **Docs-first:** prefer `docs.rs/bevy/0.18.1/bevy/` over examples; treat examples as secondary references and confirm names against the `v0.18.1` git tag.

## Working agreement

- Claude is an **advisor/teacher**, not an engineer. Claude explains, plans, points at docs, and reviews — **you write the game code.**
- Claude runs cargo **only for information gathering** (`cargo search`, `cargo info`, `cargo tree`) and announces it so you can mirror it.
- **You run all builds/changes** yourself: `cargo run`, `cargo build`, `cargo check`, `cargo update`, `cargo add`.

## The phases

| Phase | File | Goal |
|------|------|------|
| 0 | [phase-0-ecs.md](phase-0-ecs.md) | ECS mental model — **done** ✅ |
| 1 | [phase-1-3d-scene.md](phase-1-3d-scene.md) | A lit 3D room on screen — **done** ✅ |
| 2 | [phase-2-camera.md](phase-2-camera.md) | Free-roam camera (WASD + mouse-look) — **done** ✅ |
| 3 | [phase-3-collision.md](phase-3-collision.md) | Walls that stop you — **done** ✅ |
| 4 | [phase-4-states.md](phase-4-states.md) | Game states & a menu — **done** ✅ |
| 5 | [phase-5-dungeon-gen.md](phase-5-dungeon-gen.md) | Module/plugin reorg, then procedural dungeon generation — **done** ✅ |
| 6 | [phase-6-movement.md](phase-6-movement.md) | Grounded first-person movement ← **current** |
| 7 | [phase-7-gltf.md](phase-7-gltf.md) | Real art via glTF models |
| 8 | [phase-8-gameplay.md](phase-8-gameplay.md) | Enemies, combat, items |
| 9 | [phase-9-anim-audio.md](phase-9-anim-audio.md) | Animation & audio |
| 10 | [phase-10-ui-polish.md](phase-10-ui-polish.md) | HUD & visual polish |

## Standing resources

- **API docs:** https://docs.rs/bevy/0.18.1/bevy/ (primary)
- **Official book / migration guides:** https://bevy.org/learn/
- **Examples (pin to your version):** https://github.com/bevyengine/bevy/tree/v0.18.1/examples
- **Cheatbook** (unofficial, may lag versions): https://bevy-cheatbook.github.io/

## Cross-cutting topics (not phases)

- [profiling-performance.md](profiling-performance.md) — how to measure FPS/system cost; measure-before-optimizing. Flesh out when the dungeon scales or enemies arrive.

## How to use these docs

Each phase file has the same shape: **Goal → Concepts → What to study → Dungeon tie-in → Done when.** Work top to bottom. Don't advance until the "Done when" milestone is met — each phase is a prerequisite for the next.
