# Phase 7 — Enemies, Combat, Items

## Goal

Make it a game: enemies that chase, combat that hurts, items you pick up.

## Concepts

- Marker components: `Player`, `Enemy`, `Health`, `Item`
- Query filters: `With<T>` / `Without<T>` to separate player from enemies
- Events / messages: "took damage", "picked up item" — decouple cause from effect
- Observers: react to component/entity changes
- Simple chase AI: each frame, move enemy toward player's position

## Shopping list (docs.rs/bevy/0.18.1)

| Need | Look up |
|---|---|
| Events | the message/event API (`Message` / `EventWriter` / `EventReader` equivalents) |
| Reactions | `Observer` / observer API |
| Filters | `With`, `Without`, `Query` |

## Pattern

Don't mutate health directly from the collision system. Emit a damage event; a separate system applies it. This keeps systems small and composable.

## Dungeon tie-in

Spawn enemies and items into rooms during generation (Phase 5). Reuse the Phase 3 collision test for melee/projectile hits.

## Reference

Example `alien_cake_addict.rs` is a complete small 3D game — read the whole thing. Also `observers.rs`, `message.rs`.

## Done when

Enemies pursue you, combat reduces health, and pickups change state via events rather than direct cross-system mutation.
