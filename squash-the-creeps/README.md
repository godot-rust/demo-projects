# Squash the Creeps

A simple 3D game: move around, jump on the creeps to squash them, and don't let them touch you. Ported from the Godot
["Your first 3D game"][tutorial] tutorial.

Concepts on display: `OnReady` node references, `#[export]` properties with `#[init(val = ...)]` defaults, and type-safe
signals between scene nodes. The gameplay logic lives in [`rust/src`](rust/src); the Godot project in [`godot`](godot)
only holds scenes and assets.

[tutorial]: https://docs.godotengine.org/en/latest/getting_started/first_3d_game/index.html

## Asset credits

`godot/art/House In a Forest Loop.ogg` Copyright &copy; 2012 [HorrorPen](https://opengameart.org/users/horrorpen),
[CC-BY 3.0: Attribution](http://creativecommons.org/licenses/by/3.0/).
Source: https://opengameart.org/content/loop-house-in-a-forest

The 3D models (`godot/art/*.glb`) and materials are taken from the upstream Godot demo.

Font is "Montserrat". Details are in `godot/fonts/LICENSE.txt`.
