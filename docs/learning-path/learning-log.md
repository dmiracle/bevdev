# Learning Log

Granular record of concepts learned, bugs hit, and decisions made — organized by phase. Newest entries at the bottom of each phase. New sessions: read **Current state** and **Open questions** first, then skim the latest phase.

---

## Current state

- **Phase 3 (collision) in progress.** Walls now have `Collider` components and the wall query loop fires. **Next task: implement the point-inside-AABB test against `player_pos`, then MTV resolution.**
- Commits happen per phase on feature branches (user runs git).

## Open questions / deferred items

- **Cursor-released camera spin** — after Escape, `camera_controller` still reads mouse motion. Proper fix: run conditions with game states (Phase 4). Deliberately deferred.
- **Grounded movement** — `forward()` includes Y, so W while looking up flies. Correct fly-cam behavior for now; flatten forward/right to XZ when dungeon walking arrives (Phase 3/5).
- **Player radius** — collision is point-based; later inflate wall half-extents by a player radius so the camera can't touch wall faces.
- **Floor collider** — floor has no `Collider`; ground handling decision deferred.

---

## Tooling & environment (cross-cutting)

- **Cargo version requirements**: `"0.16.1"` means `^0.16.1` (≥0.16.1, <0.17.0). For 0.x crates the first non-zero digit is the breaking-change digit. `cargo update` moves within the range; crossing a minor requires editing `Cargo.toml` by hand. `cargo upgrade` is from the `cargo-edit` add-on (not installed).
- **Finding latest versions**: `cargo search bevy` (shows absolute newest incl. RCs), `cargo info bevy` (shows what cargo would resolve — excludes pre-releases). SemVer requirements never match pre-releases unless explicitly named.
- **rust-analyzer setup**: rustup proxies can exist at `~/.cargo/bin` while the component is missing (`error: Unknown binary`) — fix with `rustup component add rust-analyzer`.
- **nvim 0.11+ LSP**: core ships no default server configs; without nvim-lspconfig, `vim.lsp.config()` needs explicit `cmd`, `filetypes`, `root_markers`. `:LspLog` is an nvim-lspconfig command; raw log via `vim.lsp.get_log_path()`.
- **Default LSP keymaps**: `K` hover (press twice to enter the float), `<C-s>` signature help (insert mode), `gd` definition, `grn` rename, `gra` code action, `gO` symbols, `[d`/`]d` diagnostics.
- **Local docs**: `cargo doc --open` builds docs matching your exact lockfile versions.

---

## Phase 0 — ECS mental model ✅

(From the official hello-world tutorial, pre-existing.)

- `App`, `DefaultPlugins`, custom `Plugin`
- `Component`, `Resource`, `Entity`, spawn tuples
- Systems = plain functions; params declare data access; parallelism falls out of disjoint access
- `Startup` vs `Update` schedules; `.chain()` for ordering
- `Query<&T, With<U>>`, `Res`/`ResMut`, `Timer`/`TimerMode`

## Phase 1 — Lit 3D room ✅

- **Three entity kinds make a scene**: camera (`Camera3d` + `Transform`), light (`PointLight`/`DirectionalLight`), renderables (`Mesh3d` + `MeshMaterial3d` + `Transform`). `DefaultPlugins` already includes everything needed.
- **Assets vs handles**: `Assets<Mesh>` / `Assets<StandardMaterial>` are World-owned resource stores; `.add()` returns a cheap `Handle` you attach to entities. Handles created as locals in `setup`, persisted by living on entities. Rust globals are not used — state that outlives a function lives in the World (component or resource).
- **`Color` is color *spaces*, not color names**: variants are `Srgba`, `Hsla`, etc. Named colors live in `bevy::color::palettes::{css,tailwind,basic}` (as `Srgba`, convert via `Color::from`). `Color::srgb(r, g, b)` for direct construction. Discovery: autocomplete on the type, or docs.rs Variants + Implementations.
- **`..default()` = struct update syntax + `Default` trait**: `..` fills unlisted fields from another value of the same type; `default()` is Bevy's prelude shorthand for `Default::default()`.
- **Lighting**: with no light entity, only Bevy's default `AmbientLight` resource illuminates (flat, no shading). `shadows_enabled: true` on `PointLight` gives shadow-casting; cube needed something to cast onto (sits on floor: y = half its height).

## Phase 2 — Free-roam camera ✅

- **Many small systems, not one big update**: each behavior is its own system; register multiple with tuples. Only impose order where a real dependency exists (`.chain()`, `.after()`).
- **Persistent per-entity state = component**: `CameraController { yaw, pitch, speed, sensitivity }` stores between-frame state AND acts as a query marker. Locals die when a system returns; the World persists.
- **Query mechanics**: the type parameter IS the request — `Query<(&mut Transform, &mut CameraController)>` matches entities having both, grants declared access. Requesting a component's data also filters (no `With` needed if data is wanted anyway). `single_mut()` = "exactly one match" → `Result` → `.unwrap()` during development. Returned `Mut<T>` derefs to the component.
- **Coordinates**: right-handed, Y-up; camera forward is local −Z. `transform.forward()/right()/up()` give local axes as `Dir3`.
- **Movement math**: accumulate contributions per key into one `Vec3`, `normalize_or_zero()` once (fixes √2 diagonal speed; guards NaN on zero vector), then `translation += dir * speed * time.delta_secs()`.
- **Mouse-look math**: store yaw/pitch as scalars; rebuild rotation each frame (`Quat` yaw about world Y × pitch about local X) — never accumulate quaternions (drift/roll). Clamp pitch ~±1.54 rad. Mouse delta is NOT multiplied by `delta_secs` (already per-frame physical distance). `Res<AccumulatedMouseMotion>` provides the summed frame delta.
- **Frame-1 snap**: rebuilding rotation from yaw/pitch overwrites `looking_at`; fix by seeding initial yaw/pitch instead.
- **Helpers are not systems**: free functions get no injection — they take plain references (`&ButtonInput<KeyCode>`, `&Transform`). Deref coercion turns `&Res<T>`/`&Mut<T>` into `&T` at call sites.
- **Rust syntax learned**: prefix `*` = deref (`*transform.forward()` unwraps `Dir3`→`Vec3`, via the `Deref` trait); `+=`/`-=` compound assignment via `AddAssign`; tail expression (no semicolon) is the return value — a trailing `;` makes it a statement returning `()`; `return` is for early exits; methods (dot-call, from `impl` blocks — glam's `Vec3` ships many) vs free functions (bare-name call).
- **Logging**: `info!`/`warn!`/`error!` visible by default; `debug!`/`trace!` hidden at default level. Gate per-frame logs (e.g. on `mouse.delta != Vec2::ZERO`). `dbg!` for throwaway value checks.
- **Cursor grab (0.18)**: `CursorOptions` is its own component on the window entity (not a `Window` field). Query `&mut CursorOptions, With<PrimaryWindow>`; set `grab_mode` (`CursorGrabMode::Locked` — macOS doesn't support `Confined`) and `visible`. `just_pressed` (edge) vs `pressed` (held) for toggles.
- **Frame timing**: winit runner loop = one iteration per frame; paced by VSync (`Fifo` present mode) → monitor refresh is the effective clock. `Time`'s delta is *measured* wall-clock elapsed, not a target. `FixedUpdate` runs at a constant timestep (~64 Hz, accumulator catch-up) — the home for physics later.

## Phase 3 — Collision (in progress)

- **AABB**: axis-aligned box as center + half-extents (`half = full_size / 2`; `Vec3::splat` for uniform). `min = c - h`, `max = c + h`. Collider must match the visible mesh.
- **Point-inside test**: three AND-ed range checks, one per axis.
- **MTV resolution (designed, not yet implemented)**: measure distance to both faces per axis, take the smallest of the 6 pushes, correct ONLY that axis → produces wall-sliding instead of sticking.
- **Player vs walls queries**: `With<CameraController>` / `Without<CameraController>` make the two `Transform` accesses provably disjoint — omitting `Without` panics with a conflicting-query error. Player via `single`; walls via `for (t, c) in &walls` iteration.
- **System ordering**: `resolve_collisions` chained after `camera_controller` (corrects the position movement just wrote). Separate `add_systems` calls accumulate.
- **Throttled logging**: `LogTimer(Timer)` resource; `tick(time.delta())` once per frame (outside loops, unconditionally); `just_finished()` fires one frame per interval. `TimerMode::Repeating` auto-resets. Alternatives: `t_info!` macro_rules (`$($arg:tt)*` forwarding idiom; statics for hidden state trade away Bevy `Time`), built-in `info_once!`.
- **Bug: query never matched** — `Collider` was *defined* but never *attached*; queries only match components that exist on entities. Debug move: `walls.iter().count()`. Fix: add the component to the spawn tuple.
- **Printing math types**: `Vec3` implements `Debug` and `Display` — `info!("{pos:?}")` works directly (inline capture, edition 2021+); per-field `{:.2}` for decimal control.
