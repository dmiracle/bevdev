# Phase 1 — A Lit 3D Room ← current

## Goal

Get a lit 3D scene on screen: a camera, a light, a floor, and a couple of cubes.

## The conceptual shift

Your hello-world skeleton already works for 3D — `DefaultPlugins` includes rendering, windowing, and the asset server. **What changes is what you spawn.** A 3D scene is just three kinds of entities:

1. **Camera** — entity with `Camera3d` + a `Transform` positioned and aimed.
2. **Light** — `PointLight` or `DirectionalLight` (without one, everything is black).
3. **Renderables** — entities with `Mesh3d(handle)` + `MeshMaterial3d(handle)` + `Transform`.

## Concepts

- `Camera3d`, `PointLight` / `DirectionalLight`
- `Mesh3d`, `MeshMaterial3d`, `StandardMaterial`
- Asset stores: `ResMut<Assets<Mesh>>`, `ResMut<Assets<StandardMaterial>>`
- Mesh primitives: `Cuboid`, `Plane3d`, `Sphere`
- `Transform::from_xyz(...)`, `.looking_at(target, up)`

## Shopping list (confirm each on docs.rs/bevy/0.18.1)

| Need | Look up | Module |
|---|---|---|
| 3D camera | `Camera3d` | prelude |
| Light | `PointLight`, `DirectionalLight` | prelude |
| Mesh holder | `Mesh3d` | prelude |
| Material holder | `MeshMaterial3d` | prelude |
| Material | `StandardMaterial` | `bevy::pbr` |
| Mesh store | `Assets<Mesh>` | request as `ResMut<...>` |
| Material store | `Assets<StandardMaterial>` | request as `ResMut<...>` |
| Shapes | `Cuboid`, `Plane3d`, `Sphere` | `bevy::math` primitives |
| Positioning | `Transform`, `.looking_at()` | prelude |

## Two new patterns

**1. Getting asset handles** (assets live in a store; you add and get a handle back):

```rust
mut meshes: ResMut<Assets<Mesh>>,
mut materials: ResMut<Assets<StandardMaterial>>,
// ...
let floor_mesh = meshes.add(Plane3d::default().mesh().size(10.0, 10.0));
let floor_mat  = materials.add(Color::srgb(0.3, 0.5, 0.3));
```

**2. The render-entity spawn** (same tuple-spawn you know, different components):

```rust
commands.spawn((Mesh3d(floor_mesh), MeshMaterial3d(floor_mat), Transform::from_xyz(0.0, 0.0, 0.0)));
```

> Snippets are illustrative — verify the meshing API (`.mesh().size(...)`) against the 0.18.1 docs.

## Camera placement

Put the camera back and up, then aim it at the origin:

```rust
Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y)
```

`looking_at(target, up)` rotates the transform so its forward (-Z) points at `target`; `up` (usually `Vec3::Y`) keeps it from rolling.

## Dungeon tie-in

A floor plane + four wall cubes = your first room. Every later object reuses this mesh→material→transform→spawn atom.

## Done when

You can change a cube's position or color in code and see it move/recolor on the next `cargo run`.

## Run it (you run this)

```
cargo run
```
