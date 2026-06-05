# Phase 2 — Free-Roam Camera

## Goal

A grounded camera you steer: WASD to move, mouse to look.

## Decide first

- **Fly-cam** (6DOF, no gravity) — easiest, good to start.
- **Grounded FPS** (gravity + stay on floor) — what the dungeon crawler ultimately wants. Needs Phase 3 collision to feel right.

Start fly-cam, then add grounding once collision exists.

## Concepts

- `Res<ButtonInput<KeyCode>>` — held-key polling
- `Res<Time>` and `time.delta_secs()` — **framerate-independent** movement
- Mouse-look: reading mouse motion + cursor grab/lock
- Rotating a `Transform` with quaternions; translating along its local axes

## Shopping list (docs.rs/bevy/0.18.1)

| Need | Look up |
|---|---|
| Keyboard | `ButtonInput<KeyCode>`, `KeyCode` |
| Mouse motion | mouse-motion event/message type |
| Cursor lock | window cursor grab mode + visibility |
| Time | `Time`, `Time::delta_secs` |
| Movement | `Transform::translation`, `Transform::forward()`/`right()`, `Transform::rotate_*` |

## The golden rule

Every per-frame movement multiplies by `time.delta_secs()`. If you ever see movement speed change when FPS changes, you forgot this.

## Dungeon tie-in

Walk around your Phase 1 room. This camera entity is the thing collision (Phase 3) will constrain.

## Reference

Example `free_camera_controller.rs` is almost exactly this — read it, but write your own.

## Done when

Movement is smooth and framerate-independent, mouse-look works, and you can capture/release the cursor cleanly (e.g. Escape to free the mouse).
