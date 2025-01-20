![logo.png](https://github.com/godot-rust/assets/blob/master/gdext/banner.png?raw=true)

# Demo projects for godot-rust

_**[Website]** | **[Main repo]** |  [Book] | [API Docs] | [Sponsor]_

This repository collects official demos and examples for the **[gdext][Main repo]** library. There are few demos at the moment, and we don't
intend to showcase every aspect of the library here. Instead, demos are designed to be more "real-world" oriented, without getting so big
that readers are overhwelmed.

Demo projects do not replace the book and API docs, but are meant to be read in addition. If you're encountering concepts in the
code that seem strange, please check both book and docs first!


## Rust demos

We currently feature the following demo projects:

- [**Dodge the Creeps**](dodge-the-creeps)  
  A simple game where you dodge enemies. Ported from the official Godot tutorial.

- [**Hot reload**](hot-reload)  
  Showcases how classes and their state can be hot-reloaded in the Godot editor.

- [**Multiplayer bomber**](hot-reload)  
  A port of godot's [multiplayer bomber demo](https://github.com/godotengine/godot-demo-projects/tree/master/networking/multiplayer_bomber). Showcases how to use basics of godot's abstraction for multiplayer with godot-rust.


## Engine and library versions

Demos are written to work with **latest stable** Godot version (last official release), and last `master` version of godot-rust.
We generally try to update this repo within a few days or weeks.

We additionally run CI with newer in-dev versions, but that support is best-effort. Demos do not provide compatibility with older Godot
versions, as the projects need migration and this would prevent us from showcasing newer versions. godot-rust itself however offers reliable
support for older versions, see [Compatibility and Stability][book-compatibility].

If you're interested in GDScript demos for Godot itself, check out [godot-demo-projects].


## Contributions

If you would like to contribute a demo project, please open an issue **before** submitting a big pull request.
Keep in mind that this repo is not a collection of user projects, but an educational resource to complement
the book and API docs of the library. Therefore, demos need to follow certain standards regarding code quality,
documentation and maintainability. But don't worry, if you open an issue, we can guide you through! 🙂

Like the library, all contributions are subject to the [Mozilla Public License 2.0][mpl].


[Main repo]: https://github.com/godot-rust/gdext
[API Docs]: https://godot-rust.github.io/docs/gdext
[Sponsor]: https://github.com/sponsors/Bromeon
[Website]: https://godot-rust.github.io
[Book]: https://godot-rust.github.io/book
[mpl]: https://www.mozilla.org/en-US/MPL
[book-compatibility]: https://godot-rust.github.io/book/toolchain/compatibility.html
[godot-demo-projects]: https://github.com/godotengine/godot-demo-projects
