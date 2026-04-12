/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::classes::{
    AcceptDialog, Button, Control, IControl, ItemList, Label, LineEdit, MultiplayerApi, Os, Panel,
};
use godot::prelude::*;
use godot::tools::get_autoload_by_name;

use crate::gamestate::GameState;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct Lobby {
    #[init(node = "Connect/Name")]
    player_name: OnReady<Gd<LineEdit>>,
    #[init(node = "Connect/ErrorLabel")]
    error_label: OnReady<Gd<Label>>,
    #[init(node = "Connect/Host")]
    host_button: OnReady<Gd<Button>>,
    #[init(node = "Connect/Join")]
    join_button: OnReady<Gd<Button>>,
    #[init(node = "Connect/IPAddress")]
    ip_address: OnReady<Gd<LineEdit>>,
    #[init(node = "Connect")]
    connect_panel: OnReady<Gd<Panel>>,
    #[init(node = "Players")]
    players_panel: OnReady<Gd<Panel>>,
    #[init(node = "Players/List")]
    players_list: OnReady<Gd<ItemList>>,
    #[init(node = "Players/Start")]
    start_button: OnReady<Gd<Button>>,
    #[init(node = "ErrorDialog")]
    error_dialog: OnReady<Gd<AcceptDialog>>,
    #[init(val = OnReady::manual())]
    multiplayer: OnReady<Gd<MultiplayerApi>>,
    base: Base<Control>,
}

#[godot_api]
impl IControl for Lobby {
    fn ready(&mut self) {
        self.multiplayer
            .init(self.base().get_multiplayer().unwrap());
        let gamestate = get_autoload_by_name::<GameState>("gamestate");
        gamestate.signals().player_list_changed().connect_other(self, Lobby::refresh_lobby);
        gamestate.signals().connection_failed().connect_other(self, Lobby::on_connection_failed);
        gamestate.signals().connection_succeeded().connect_other(self, Lobby::on_connection_success);
        gamestate.signals().game_ended().connect_other(self, Lobby::on_game_ended);
        gamestate.signals().game_error().connect_other(self, Lobby::on_game_error);

        // button signals
        self.host_button.signals().pressed().connect_other(self, Lobby::on_host_pressed);
        self.join_button.signals().pressed().connect_other(self, Lobby::on_join_pressed);
        self.start_button.signals().pressed().connect_other(self, Lobby::on_start_pressed);
    }
}

#[godot_api]
impl Lobby {
    #[func]
    fn on_host_pressed(&mut self) {
        if self.player_name.get_text().is_empty() {
            self.error_label.set_text("Invalid name!");
            return;
        }

        self.connect_panel.hide();
        self.players_panel.show();
        self.error_label.set_text(&GString::default());
        let player_name = self.player_name.get_text();
        get_autoload_by_name::<GameState>("gamestate").bind_mut().host_game(player_name);
        self.refresh_lobby();
    }

    #[func]
    fn on_join_pressed(&mut self) {
        if self.player_name.get_text().is_empty() {
            self.error_label.set_text("Invalid name!");
            return;
        }

        let ip = self.ip_address.get_text();
        self.host_button.set_disabled(true);
        self.join_button.set_disabled(true);
        self.error_label.set_text(&GString::default());

        let player_name = self.player_name.get_text();
        get_autoload_by_name::<GameState>("gamestate").bind_mut().join_game(ip, player_name);
    }

    #[func]
    fn on_connection_success(&mut self) {
        self.connect_panel.hide();
        self.players_panel.show();
    }

    #[func]
    fn on_connection_failed(&mut self) {
        self.error_label.set_text("Connection failed.");
        self.host_button.set_disabled(false);
        self.join_button.set_disabled(false);
    }

    #[func]
    fn on_game_ended(&mut self) {
        self.base_mut().show();
        self.connect_panel.show();
        self.players_panel.hide();
        self.host_button.set_disabled(false);
        self.join_button.set_disabled(false);
    }

    #[func]
    fn on_game_error(&mut self, error: GString) {
        self.error_dialog.set_text(&error);
        self.error_dialog.popup_centered();
        self.host_button.set_disabled(false);
        self.join_button.set_disabled(false);
    }

    #[func]
    fn refresh_lobby(&mut self) {
        // add current player at the top of the players list
        self.players_list.clear();
        self.players_list.add_item(&format! ("{} (You)", get_autoload_by_name::<GameState>("gamestate").bind().player_name));

        let game_state = get_autoload_by_name::<GameState>("gamestate");
        let binding = game_state.bind();
        let other_players = binding.get_player_list();

        for player in other_players.iter_shared() {
            self.players_list.add_item(&player);
        }
        let is_server = self.base().get_multiplayer().unwrap().is_server();
        self.start_button.set_disabled(!is_server);
    }

    #[func]
    fn on_start_pressed(&mut self) {
        get_autoload_by_name::<GameState>("gamestate").bind_mut().begin_game();
    }

    #[func]
    fn on_find_public_ip_pressed(&self) {
        Os::singleton().shell_open("https://icanhazip.com/");
    }
}