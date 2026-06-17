# Learning Log

Granular record of concepts learned, bugs hit, and decisions made — organized by phase. Newest entries at the bottom of each phase. New sessions: read **Current state** and **Open questions** first, then skim the latest phase.

---

## Current state

- **Phase 5 Step 0 done ✅ — `main.rs` split into per-concern plugin modules** (`state`, `camera`, `collision`, `world`, `debug`); `main.rs` is now ~26 lines of `mod`/`use`/`add_plugins`. Game runs, full cycle verified. **Cursor-grab cleanup also done**: `toggle_cursor_grab` split into gated `pause_game` (Escape, `in_state(Playing)`) + `resume_game` (click, `in_state(Paused)`), both cursor-free — cursor is now purely `OnEnter`-driven. Menu no longer responds to Escape/click. **Next: actual dungeon generation (plain-Rust grid/graph algorithm first, then spawn from it).** (Commit per phase — user runs git.)
- Commits happen per phase on feature branches (user runs git).

## Open questions / deferred items

- **Grounded movement** — `forward()` includes Y, so W while looking up flies. Correct fly-cam behavior for now; flatten forward/right to XZ when dungeon walking arrives (Phase 3/5).
- **Player radius** — resolution chosen (Minkowski inflation of wall half-extents by `PLAYER_RADIUS` ~0.4 inside `resolve_collisions`); implementation in flight. Later: promote the const to a tunable (doorway widths in Phase 5 depend on it).
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

## Phase 3 — Collision ✅

- **AABB**: axis-aligned box as center + half-extents (`half = full_size / 2`; `Vec3::splat` for uniform). `min = c - h`, `max = c + h`. Collider must match the visible mesh.
- **Point-inside test**: three AND-ed range checks, one per axis.
- **MTV resolution (designed, not yet implemented)**: measure distance to both faces per axis, take the smallest of the 6 pushes, correct ONLY that axis → produces wall-sliding instead of sticking.
- **Player vs walls queries**: `With<CameraController>` / `Without<CameraController>` make the two `Transform` accesses provably disjoint — omitting `Without` panics with a conflicting-query error. Player via `single`; walls via `for (t, c) in &walls` iteration.
- **System ordering**: `resolve_collisions` chained after `camera_controller` (corrects the position movement just wrote). Separate `add_systems` calls accumulate.
- **Throttled logging**: `LogTimer(Timer)` resource; `tick(time.delta())` once per frame (outside loops, unconditionally); `just_finished()` fires one frame per interval. `TimerMode::Repeating` auto-resets. Alternatives: `t_info!` macro_rules (`$($arg:tt)*` forwarding idiom; statics for hidden state trade away Bevy `Time`), built-in `info_once!`.
- **Bug: query never matched** — `Collider` was *defined* but never *attached*; queries only match components that exist on entities. Debug move: `walls.iter().count()`. Fix: add the component to the spawn tuple.
- **Printing math types**: `Vec3` implements `Debug` and `Display` — `info!("{pos:?}")` works directly (inline capture, edition 2021+); per-field `{:.2}` for decimal control.
- **Bug: mutating a copy** — `let pos = transform.translation;` copies (`Vec3` is `Copy`); corrections applied to the local never reached the World, so the position "reverted" every frame. Fix: resolve on the local through the wall loop, write it back to the transform once after the loop. Python contrast: assignment there aliases, in Rust `Copy` types duplicate.
- **Bug: MTV sign** — depth was computed correctly but always applied positive; pushes through a wall's min-side face. The `p_min[i] < p_max[j]` comparison already encodes the direction: min-face wins → negative correction, max-face wins → positive. Keep `(axis, signed_depth)` instead of re-finding the min.
- **MTV scale is exactly 1.0** — snap precisely to the face; any multiplier overshoots and reads as bounce/jitter.
- **glam index helpers**: `min_position()` returns the index of the smallest component; `Vec3` supports indexing (`v[i]`).
- **Per-frame `info!` causes visible hitching** — unthrottled logging in a collision branch masqueraded as a physics bug. Gate per-frame logs behind the `LogTimer` or delete after use.
- **Test geometry matters** — a 1 m cube can't demonstrate sliding (you slip off instantly) and approaching from above makes the top face win the MTV (reads as bouncing). A long/tall/thin wall (`Cuboid::new(10, 4, 0.5)`) is the right rig; spawn height vs wall top caught us once (camera at y=4.5 sails over a y≤4 wall — collision correctly never fires).
- **On-screen debug text (Phase 9 preview)**: UI is entities — `Text::new(..)` + `Node { position_type: Absolute, top/left: Val::Px(..) }` + marker component; update system assigns `text.0 = format!(..)`. Runs `.after(resolve_collisions)` so the displayed position is post-correction. No extra camera needed; UI overlays the existing `Camera3d`.
- **Player radius via Minkowski inflation** — point-vs-(box grown by r) ≡ sphere-of-r-vs-box: add `Vec3::splat(PLAYER_RADIUS)` to half-extents when building min/max in `resolve_collisions`; detection, depths, MTV all unchanged. Keeps the camera's ~0.1 near plane out of wall faces (r ≥ ~0.3). Known approximation: corners act squared-off, not rounded — universally accepted in AABB games.
- **Scratch crate workflow**: sibling crate (`~/a/rscratch`) with just `glam` for math experiments; `std::any::type_name::<T>()` reveals a type's true path (`bevy::prelude::Vec3` *is* `glam::f32::vec3::Vec3` — re-export, not a wrapper). Discovery moves: `K` hover / `gd` into `~/.cargo/registry` source, docs.rs re-export listings, `cargo tree -i glam`.
- **Primitive as source of truth**: `meshes.add(cuboid)` bakes the shape into vertex buffers — one-way; the `Mesh` doesn't remember its primitive. Keep the `Cuboid` (it's `Copy`) and derive both the mesh and `half_size` for the collider from it; walls spawn from a `for (size, pos) in [...]` data array (tuple destructuring in the `for` pattern) — proto-dungeon-generator shape.
- **Handles are shared, assets aren't deduped**: `materials.add` in a loop makes N identical assets; hoist one handle and `.clone()` it per spawn (cheap, reference-counted).
- **Ordering constraints accumulate**: `.chain()` on a tuple = pairwise `.after()`; separate `add_systems` calls merge into one schedule graph, so a system can be chained in one call and referenced by `.after()` in another. Constrain only real data dependencies — everything else stays parallel. Pipeline here: `camera_controller → resolve_collisions → update_debug_text`.
- **Temporaries vs named bindings**: an unbound `query.single_mut().unwrap().translation` drops its `Mut` at end of statement (forcing a re-acquire later); `let mut transform = ...` keeps it alive to end of scope. Holding the player `Mut` while iterating `&walls` is fine — the `With`/`Without` filters already proved the queries disjoint. Writing through a `Mut` flags change detection even if the value is unchanged (unconditional write-back each frame is harmless today; gate it when something reacts to `Changed<Transform>`).

## Phase 4 — Game states & menu ✅

- **States API (0.18)**: enum deriving `States` (+ `Clone, PartialEq, Eq, Hash, Debug, Default`); `#[default]` marks the boot state. `.init_state::<T>()` on the App creates the `State<T>`/`NextState<T>` resources — deriving alone puts nothing in the World (same lesson as Collider-never-attached, one level up: forgetting it compiles fine, then panics at first `Res<State<T>>`).
- **Reading/writing state**: read `Res<State<T>>` → `.get()`; request changes via `ResMut<NextState<T>>::set(..)` — applied at a transition point between frames, not instantly.
- **`run_if(in_state(..))`** on the chained tuple gates the whole pipeline as a unit; the toggle system stays ungated so it can run while `Paused` to resume. This fixed the cursor-released camera-spin bug (oldest open question).
- **`::` vs `.`**: `::` navigates namespaces (modules, types, associated items — `GameState::Paused`, `Vec3::ZERO`); `.` accesses values. `variable::Trait` is nonsense — "print with Debug" is spelled `{:?}` in the format string, not a path. `{}` requires `Display`, `{:?}` requires `Debug`.
- **Don't depend on bevy subcrates directly**: `bevy_state = "0.18.1"` alongside `bevy` resolves to one crate instance today, but version drift links *two copies* — your derive implements copy A's trait, bevy expects copy B's → baffling "trait not implemented" errors. Everything reaches through `bevy::prelude`/`bevy::` paths; `cargo tree -i bevy_state` shows the duplication risk. (Root cause of the "unresolved crate" detour: a `use bevy_state::...` line copied from standalone-crate docs *caused* the error; cargo-adding the dep papered over it.)
- **Identity transitions**: `NextState::set(Playing)` while already `Playing` is a no-op request — harmless until `OnEnter(Playing)` has side effects; guard on current state when that lands (combat clicks, Phase 7).
- **`OnEnter`/`OnExit` are transition schedules** (like `Startup`, but fired by state changes): one-time per transition. `OnEnter(default)` runs once at boot — so booting into `Menu` fires `OnEnter(Menu)`, which is why cursor-release-on-menu works without a `Startup` grab. Pattern: `OnEnter` = spawn/setup, `OnExit` = despawn cleanup (tag with a marker, `Query<Entity, With<Marker>>`, `commands.entity(e).despawn()` in a loop).
- **One-time vs per-frame placement bug**: registering `menu_input` (which polls `just_pressed(Space)`) in `OnEnter(Menu)` ran it once on entry — keypress never seen. Per-frame polling belongs in `Update` + `run_if(in_state(Menu))`; `OnEnter` is for one-shot setup only. Masked at first because ungated `toggle_cursor_grab`'s left-click branch also sets `Playing`, so clicking "worked."
- **Cursor follows state**: lock/unlock moved out of input handlers into `OnEnter(Playing)` = lock, `OnEnter(Menu)/(Paused)` = release. Input handlers should *request a state change*; the cursor reacts to the state. Shared `set_cursor(&mut CursorOptions, locked: bool)` helper (deref-coerce `&mut single_mut().unwrap()`); thin `lock_cursor`/`release_cursor` system wrappers.
- **Still ungated**: `toggle_cursor_grab` runs in all states (Escape→Paused, click→Playing everywhere) — Escape-from-menu reaches a nonsensical Paused, click-from-menu is a second menu exit. Deliberately deferred to the **post-split cleanup pass**: gate Escape→Paused to `in_state(Playing)` and click→Playing to `in_state(Paused)`.

## Phase 5 Step 0 — module/plugin split ✅

- **`mod` vs `use` are different jobs.** `mod foo;` *mounts* a file into the crate's module tree — written **once**, in the parent (all `mod` lines live in `main.rs`). `use path::Item;` brings a name into scope — written wherever needed. `mod` is not an import; coming from Python/JS, `use` is the import-like one, `mod` has no equivalent (those languages infer it from the filesystem).
- **Path resolution is relative to the current module.** A bare `state::` works in `main.rs` because `main.rs` *is* the crate root where `state` is mounted; in `camera.rs` it fails (looks for `crate::camera::state`). `crate::` is absolute (from root, works everywhere), `super::` = parent, `self::` = here. Unify on `use crate::...` in every file for consistency.
- **Plugin shape**: tiny `pub struct FooPlugin;` + `impl Plugin { fn build(&self, app) {...} }`. `build` is wiring ONLY (`add_systems`/`init_state`/`insert_resource`); the systems are free functions alongside it, mostly **private** — registering them inside the same module's `build` means they never cross a boundary. `build` takes no `pub` (trait method). `add_plugins((A, B, C))` takes ONE tuple arg.
- **Visibility rules**: `pub` only what another module names. `pub struct` does NOT make fields pub — struct-literal construction at a call site needs every field visible, so `CameraController`/`Collider` needed `pub` *fields* (world.rs builds them). Enum variants of a `pub enum` are auto-public (no per-variant `pub`). A system needs `pub` only if referenced from *outside* its module.
- **Private type in public interface (E0446)**: making `resolve_collisions` pub forced `LogTimer` pub (it appears in the signature as `ResMut<LogTimer>`). The real lesson: the pub wasn't warranted — `resolve_collisions` is registered in its own `build`, so it needn't be pub at all. Reverted both to private.
- **`.chain()` does NOT survive a module split** — splitting the gameplay tuple across plugins silently dropped the ordering, and Bevy does *not* error on a missing constraint (it runs systems in an unspecified order; here both `camera_controller` and `resolve_collisions` write the camera `Transform`, so order matters). Reintroduced the one real constraint with `resolve_collisions.after(camera::camera_controller)` inside `CollisionPlugin` (needs only `camera_controller` pub). `update_debug_text`'s ordering is cosmetic (debug read) → left unordered. SystemSets are the scalable tool when cross-module ordering grows — deferred.
- **Concern leakage to watch**: `world::setup` was still spawning the debug-text entity (importing `DebugText`). Moved that spawn into `debug.rs`'s own `setup_debug` (Startup), so `debug` owns the overlay end-to-end and `world` is pure scene. Component definition moving modules ≠ its spawn moving — check both.
- **`cargo fmt` normalizes whitespace but doesn't *insert* missing blank lines between items** — a missing separator after a `use` block slipped past it; caught by eye.
- **Final shape**: `main.rs` = `mod` + `use` + `add_plugins((DefaultPlugins, StatePlugin, CameraPlugin, CollisionPlugin, WorldPlugin, DebugPlugin)).run()`. Each file: own `use bevy::prelude::*;` (also needed for derive-macro names), one concern, one plugin, minimal pub surface.
