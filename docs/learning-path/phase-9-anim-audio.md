# Phase 9 — Animation & Audio

## Goal

Make it feel alive: animated characters and positional sound.

## Concepts — Animation

- Animation clips bundled in glTF files
- `AnimationGraph`, `AnimationPlayer`
- Triggering clips (idle/walk/attack) and animation events

## Concepts — Audio

- Spawning audio (`AudioPlayer` + source handle)
- Controlling playback (volume, pause, speed)
- **Spatial** 3D audio tied to entity `Transform`
- **Background music** (non-spatial, global) — distinct from positional sound: one `AudioPlayer` with `PlaybackSettings::LOOP`, no `Transform`-based attenuation. Likely tie to game state (e.g. menu theme vs. dungeon track) via `OnEnter`.

## User assets

- User has their own **music artifacts** to add here (none in the repo yet — will drop them in when this phase starts). Audio files go under `assets/` and load via `asset_server.load("music/...")`. Decide on format then (Bevy reads `.ogg`/`.flac`/`.wav`/`.mp3` depending on enabled features — confirm the feature flags against the actual files at that point).

## Shopping list (docs.rs/bevy/0.18.1)

| Need | Look up |
|---|---|
| Animation | `AnimationPlayer`, `AnimationGraph`, `AnimationClip` |
| Audio | `AudioPlayer`, `AudioSource`, spatial audio settings |

## Dungeon tie-in

Walk/attack animations on the player and enemies; positional sound so footsteps and growls come from the right direction.

## Reference

Examples `animated_mesh.rs`, `animation_graph.rs`, `animation_events.rs`, `spatial_audio_3d.rs`, `audio_control.rs`.

## Done when

The player model animates with movement, and an enemy emits sound that gets louder/directional as you approach.
