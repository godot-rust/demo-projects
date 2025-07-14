use godot::classes::Button;
use godot::classes::ENetMultiplayerPeer;
use godot::classes::Label;
use godot::classes::LineEdit;
use godot::classes::LinkButton;
use godot::classes::Os;
use godot::classes::Panel;
use godot::classes::ProjectSettings;
use godot::classes::enet_connection::CompressionMode;
use godot::classes::object::ConnectFlags;
use godot::global::Error;
use godot::prelude::*;

const DEFAULT_PORT: i32 = 8910;

#[derive(GodotClass)]
#[class(base=Panel)]
pub struct Lobby {
    #[export]
    address: Option<Gd<LineEdit>>,
    #[export]
    host_button: Option<Gd<Button>>,
    #[export]
    join_button: Option<Gd<Button>>,
    #[export]
    status_ok: Option<Gd<Label>>,
    #[export]
    status_fail: Option<Gd<Label>>,
    #[export]
    port_forward_label: Option<Gd<Label>>,
    #[export]
    find_public_ip_button: Option<Gd<LinkButton>>,
    peer: Option<Gd<ENetMultiplayerPeer>>,
    base: Base<Panel>,
}

use godot::classes::IPanel;

use crate::pong::Pong;

#[godot_api]
impl IPanel for Lobby {
    fn init(base: Base<Panel>) -> Self {
        Self {
            address: None,
            host_button: None,
            join_button: None,
            status_ok: None,
            status_fail: None,
            port_forward_label: None,
            find_public_ip_button: None,
            peer: None,
            base,
        }
    }

    fn ready(&mut self) {
        /*
            # Connect all the callbacks related to networking.
        multiplayer.peer_connected.connect(_player_connected)
        multiplayer.peer_disconnected.connect(_player_disconnected)
        multiplayer.connected_to_server.connect(_connected_ok)
        multiplayer.connection_failed.connect(_connected_fail)
        multiplayer.server_disconnected.connect(_server_disconnected)
             */
        let multiplayer = self.base().get_multiplayer().unwrap();
        let gd_ref = self.to_gd();
        multiplayer
            .signals()
            .peer_connected()
            .builder()
            .connect_other_gd(&gd_ref, |mut this: Gd<Self>, _id: i64| {
                godot_print!("Someone connected, start the game!");
                let pong: Gd<Pong> = load::<PackedScene>("res://pong.tscn")
                    .instantiate()
                    .unwrap()
                    .cast();
                // Connect deferred so we can safely erase it from the callback.
                pong.signals()
                    .game_finished()
                    .builder()
                    .flags(ConnectFlags::DEFERRED)
                    .connect_other_gd(&this, |mut this: Gd<Self>| {
                        this.bind_mut()
                            ._end_game("Client disconnected.".to_string());
                    });

                this.bind_mut()
                    .base_mut()
                    .get_tree()
                    .unwrap()
                    .get_root()
                    .unwrap()
                    .add_child(&pong);
                this.bind_mut().base_mut().hide();
            });
        multiplayer
            .signals()
            .peer_disconnected()
            .builder()
            .connect_other_mut(&self.to_gd(), |this: &mut Self, _id: i64| {
                if this.base().get_multiplayer().unwrap().is_server() {
                    this._end_game("Client disconnected.".to_string());
                } else {
                    this._end_game("Server disconnected.".to_string());
                }
            });
        multiplayer
            .signals()
            .connected_to_server()
            .builder()
            .connect_other_mut(&self.to_gd(), |_this: &mut Self| {
                // This function is not needed for this project.
            });
        multiplayer
            .signals()
            .connection_failed()
            .builder()
            .connect_other_mut(&self.to_gd(), |this: &mut Self| {
                this._set_status("Couldn't connect.".to_string(), false);
                let mut multiplayer = this.base().get_multiplayer().unwrap();
                multiplayer.set_multiplayer_peer(Gd::null_arg()); // Remove peer.
                this.host_button.as_mut().unwrap().set_disabled(false);
                this.join_button.as_mut().unwrap().set_disabled(false);
            });
        multiplayer
            .signals()
            .server_disconnected()
            .builder()
            .connect_other_mut(&self.to_gd(), |this: &mut Self| {
                this._end_game("Server disconnected.".to_string());
            });

        let gd_ref = self.to_gd();

        // Clone the Gd<Button> references to avoid borrowing self mutably and immutably at the same time
        let host_button = self.host_button.as_ref().unwrap().clone();
        host_button
            .signals()
            .pressed()
            .builder()
            .connect_other_mut(&gd_ref, |this: &mut Self| {
                this._on_host_pressed();
            });

        let join_button = self.join_button.as_ref().unwrap().clone();
        join_button
            .signals()
            .pressed()
            .builder()
            .connect_other_mut(&gd_ref, |this: &mut Self| {
                this._on_join_pressed();
            });
    }
}

#[godot_api]
impl Lobby {
    fn _set_status(&mut self, text: String, is_ok: bool) {
        // Simple way to show status.
        if is_ok {
            self.status_ok.as_mut().unwrap().set_text(&text);
            self.status_fail.as_mut().unwrap().set_text("");
        } else {
            self.status_ok.as_mut().unwrap().set_text("");
            self.status_fail.as_mut().unwrap().set_text(&text);
        }
    }

    #[func]
    fn _end_game(&mut self, with_error: String) {
        if self.base().has_node("/root/Pong") {
            // Erase immediately, otherwise network might show
            // errors (this is why we connected deferred above).
            self.base().get_node_as::<Node>("/root/Pong").free();
            self.base_mut().show();
        }

        let mut multiplayer = self.base().get_multiplayer().unwrap();
        multiplayer.set_multiplayer_peer(Gd::null_arg()); // Remove peer.
        self.host_button.as_mut().unwrap().set_disabled(false);
        self.join_button.as_mut().unwrap().set_disabled(false);

        self._set_status(with_error, false);
    }

    fn _on_host_pressed(&mut self) {
        let mut peer = ENetMultiplayerPeer::new_gd();
        self.peer = Some(peer.clone());
        // Set a maximum of 1 peer, since Pong is a 2-player game.
        let err = peer.create_server_ex(DEFAULT_PORT).max_clients(1).done();
        if err != Error::OK {
            // Is another server running?
            self._set_status("Can't host, address in use.".to_string(), false);
            return;
        }
        peer.get_host()
            .unwrap()
            .compress(CompressionMode::RANGE_CODER);

        let mut multiplayer = self.base().get_multiplayer().unwrap();
        multiplayer.set_multiplayer_peer(&peer);
        self.host_button.as_mut().unwrap().set_disabled(true);
        self.join_button.as_mut().unwrap().set_disabled(true);
        self._set_status("Waiting for player...".to_string(), true);
        let project_settings = ProjectSettings::singleton();
        self.base_mut().get_window().unwrap().set_title(
            (project_settings
                .get_setting("application/config/name")
                .to_string()
                + ": Server")
                .as_str(),
        );

        // Only show hosting instructions when relevant.
        self.port_forward_label.as_mut().unwrap().set_visible(true);
        self.find_public_ip_button
            .as_mut()
            .unwrap()
            .set_visible(true);
    }

    fn _on_join_pressed(&mut self) {
        let ip = self.address.as_mut().unwrap().get_text();
        if !ip.is_valid_ip_address() {
            self._set_status("IP address is invalid.".to_string(), false);
            return;
        }

        let mut peer = ENetMultiplayerPeer::new_gd();
        self.peer = Some(peer.clone());
        peer.create_client(&ip, DEFAULT_PORT);
        peer.get_host()
            .unwrap()
            .compress(CompressionMode::RANGE_CODER);
        let mut multiplayer = self.base().get_multiplayer().unwrap();
        multiplayer.set_multiplayer_peer(&peer);

        self._set_status("Connecting...".to_string(), true);
        let project_settings = ProjectSettings::singleton();
        self.base_mut().get_window().unwrap().set_title(
            (project_settings
                .get_setting("application/config/name")
                .to_string()
                + ": Client")
                .as_str(),
        );
    }

    fn _on_find_public_ip_pressed(&mut self) {
        let mut os = Os::singleton();
        os.shell_open("https://icanhazip.com/");
    }
}
