# Net Pong

A networked Pong game, using Godot's high-level multiplayer API with godot-rust. One player presses **Host**, the other
types in the host's IP address and presses **Join**.

Concepts on display: `#[rpc]` methods, multiplayer authority, and `OnEditor` node references wired up in the editor. The
game logic lives in [`rust/src`](rust/src); the Godot project in [`godot`](godot) only holds scenes and assets.

## Copying

Unlike the rest of this repository, the Godot project (scenes, assets) is derived from the upstream demo and stays under
the MIT License; see [LICENSE](LICENSE). The Rust code is [MPL 2.0](../License.txt) as everywhere else.

The godot-rust Ferris icon comes from [the assets repository](https://github.com/godot-rust/assets); its license is
explained [here](https://github.com/godot-rust/assets/blob/master/asset-licenses.md).
