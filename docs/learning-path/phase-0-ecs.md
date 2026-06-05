# Phase 0 — ECS Mental Model ✅

**Status: complete** (covered by the official hello-world tutorial in `src/main.rs`).

## Goal

Stop thinking in objects/classes; think in **components + systems**.

## Concepts covered

- `App`, `Plugin`, `World`
- `Entity`, `Component`, `Resource`
- `System` (a function whose params declare the data it reads/writes)
- `Schedule`: `Startup` vs `Update`
- `Query`, query filters (`With<T>`), `Commands`
- `Res` / `ResMut`, `Timer` / `TimerMode`
- System ordering with `.chain()`

## The key insight

A system is just a function. Its parameters (`Query`, `Res`, `Commands`, …) declare what slice of the `World` it touches, and Bevy schedules systems in parallel based on those declarations. There is no central `Game` object — state lives in components on entities and in resources.

## Study (if revisiting)

- Docs: the `bevy::ecs` module on docs.rs
- Example: `ecs_guide.rs`

## Done when ✅

You can explain why a system needs no `self` and why two systems that touch disjoint data can run in parallel.
