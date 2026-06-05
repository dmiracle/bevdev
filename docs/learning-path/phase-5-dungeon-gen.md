# Phase 5 — Procedural Dungeon Generation

## Goal

Generate a random, walkable, fully-enclosed level instead of a hand-placed room. **The heart of the game.**

## This is mostly algorithm work, not Bevy

You compute a grid/graph in **plain Rust**, then spawn entities from it. Keep generation logic separate from spawning.

## Algorithms to start with

- **BSP (binary space partitioning)** — recursively split space into rooms, connect with corridors. Clean, classic results.
- **Random walk / drunkard's walk** — carve floor by wandering; organic, cave-like.

Start with whichever you find clearest; both map the same way onto ECS.

## Concepts

- Represent the dungeon as a 2D grid (`Vec<Vec<Tile>>` or flat `Vec<Tile>` + width)
- Map grid coords → world `Transform` (e.g. `world = grid * tile_size`)
- Bulk-spawn entities in a loop with `Commands`
- Emit **both** a render mesh and a collider per solid tile (feeds Phase 3)

## Dungeon tie-in

This is the dungeon. Wire it to Phase 4 so entering `Playing` runs the generator.

## External resource

The *Rust Roguelike Tutorial* ("Hands-on Rust") — ignore its rendering, borrow its generation algorithms; they port cleanly to ECS spawning.

## Done when

Every run produces a different layout that is fully walkable and fully enclosed (no gaps to the void).
