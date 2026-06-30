# Profiling & Performance (cross-cutting)

Not a phase — a topic to reach for *when a profiler says so*, throughout the project. The governing rule: **measure before optimizing.** Bevy's ECS handles tens-to-hundreds of thousands of entities comfortably, so most "is this too many?" worries (e.g. per-tile floor entities) are non-issues — profiling proves it rather than guessing.

## Tiers of tooling

1. **Built-in diagnostics (quick win)** — `FrameTimeDiagnosticsPlugin` + `LogDiagnosticsPlugin` from `bevy::diagnostic` log FPS / frame-time to the console. Two lines, no deps.
2. **On-screen FPS overlay** — `bevy::dev_tools` ships an FPS overlay plugin (confirm exact name/path on docs.rs for 0.18). Pairs with the existing debug-text overlay.
3. **Per-system timing — Tracy** — enable Bevy's `trace_tracy` feature and run the Tracy profiler for a flame-graph of which *system* costs frame time. The real "what's slow" tool; systems show up as labeled spans.
4. **General Rust — `cargo flamegraph`** — CPU sampling for hot non-system code (e.g. an expensive generation algorithm).

## Non-negotiable

- **Profile in an optimized/`--release` build.** Debug builds are misleadingly slow. (`Cargo.toml` already bumps dev `opt-level`, but release is the honest number.)

## Cheap good-habits (do unconditionally)

- Share `Handle`s (one mesh / material, `.clone()` per spawn) — already the practice.
- Don't allocate per-frame in hot systems if trivially avoidable.

## Everything else waits for evidence

Don't restructure for performance until a profiler points at a specific cost. Premature optimization trades clarity (the thing this learning project values) for speed you can't show you need.

## When to flesh this out

Natural trigger: once the dungeon scales up (large maps, many entities) or enemies/AI arrive (Phase 9), or many lights land (Phase 8), giving real frame-time questions to investigate. Until then this is a stub.
