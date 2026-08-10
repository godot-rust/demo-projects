# Hot reload

Showcases how classes and their state can be hot-reloaded in the Godot editor: you change Rust code, rebuild, and the
running editor picks up the new library without a restart. All the code is in [`rust/src/lib.rs`](rust/src/lib.rs).

## Trying it out

1. Build the library and open `godot/project.godot` in the Godot editor.
2. Select the `Reloadable` node in `MainScene` and keep the Inspector visible -- it shows the exported
   `favorite_planet` property.
3. Edit `rust/src/lib.rs` and apply one of the changes marked with a `HOT-RELOAD:` comment, for example:
   - change the value returned by `get_number()`,
   - uncomment the `some_string` export to add a new property,
   - uncomment `get_planet()` to add a new method,
   - uncomment `Jupiter` to extend the `Planet` enum.
4. Run `cargo build` and switch back to the editor. The extension is reloaded automatically; the Inspector and the
   editor docs reflect your change, and `favorite_planet` keeps its value across the reload.

The `[Rust] Init/Deinit stage ...` lines printed to the terminal that launched the editor come from `on_stage_init` /
`on_stage_deinit` and let you observe when reloading happens.
