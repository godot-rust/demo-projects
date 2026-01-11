#!/bin/sh
# Copyright (c) godot-rust; Bromeon and contributors.
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

# Must be in dodge-the-creeps' rust directory, pick up the .cargo/config file.
cd `dirname "$0"`

# We build the host GDExtension first, so that the Godot editor doesn't complain.
cargo +nightly build --package dodge-the-creeps &&
cargo +nightly build --package dodge-the-creeps \
  --features godot/experimental-wasm,godot/experimental-wasm-nothreads,godot/lazy-function-tables \
  --target wasm32-unknown-emscripten -Zbuild-std $@
