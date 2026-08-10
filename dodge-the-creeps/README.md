# Dodge the Creeps

A simple 2D game where you dodge enemies for as long as possible, ported from the Godot ["Your first 2D game"][tutorial]
tutorial.

Concepts on display: type-safe signals, `OnReady` fields, and the interplay between connections declared in the editor
and connections declared in Rust. The gameplay logic lives in [`rust/src`](rust/src); the Godot project in
[`godot`](godot) only holds scenes and assets, whose credits are listed in [godot/README.md](godot/README.md).

[tutorial]: https://docs.godotengine.org/en/latest/getting_started/first_2d_game/index.html
