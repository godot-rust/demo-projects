use godot::classes::enet_connection::CompressionMode;  
use godot::classes::object::ConnectFlags;  
use godot::classes::{  
    Button, ENetMultiplayerPeer, Label, LineEdit, LinkButton, Os, Panel, ProjectSettings, IPanel 
};  
use godot::global::Error;  
use godot::prelude::*;

use crate::pong::Pong;

const DEFAULT_PORT: i32 = 8910;

#[derive(GodotClass)]
#[class(init, base=Panel)]
pub struct Lobby {
    #[export]
    address: OnEditor<Gd<LineEdit>>,
    #[export]
    host_button: OnEditor<Gd<Button>>,
    #[export]
    join_button: OnEditor<Gd<Button>>,
    #[export]
    status_ok: OnEditor<Gd<Label>>,
    #[export]
    status_fail: OnEditor<Gd<Label>>,
    #[export]
    port_forward_label: OnEditor<Gd<Label>>,
    #[export]
    find_public_ip_button: OnEditor<Gd<LinkButton>>,
    peer: Option<Gd<ENetMultiplayerPeer>>,
    base: Base<Panel>,
}

#[godot_api]
impl IPanel for Lobby {

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
                    .instantiate_as::<Pong>();
                // Connect deferred so we can safely erase it from the callback.
                pong.signals()
                    .game_finished()
                    .builder()
                    .flags(ConnectFlags::DEFERRED)
                    .connect_other_gd(&this, |mut this: Gd<Self>| {
                        this.bind_mut()
                            .end_game("Client disconnected.".to_string());
                    });

                this.bind_mut()
                    .base_mut()
                    .get_tree()
                    .unwrap()
                    .get_root()
                    .unwrap()
                    .add_child(&pong);
                this.hide();
            });
        multiplayer
            .signals()
            .peer_disconnected()
            .builder()
            .connect_other_mut(&self.to_gd(), |this: &mut Self, _id: i64| {
                if this.base().get_multiplayer().unwrap().is_server() {
                    this.end_game("Client disconnected.".to_string());
                } else {
                    this.end_game("Server disconnected.".to_string());
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
                this.set_status("Couldn't connect.", false);
                let mut multiplayer = this.base().get_multiplayer().unwrap();
                multiplayer.set_multiplayer_peer(Gd::null_arg()); // Remove peer.
                this.host_button.set_disabled(false);
                this.join_button.set_disabled(false);
            });
        multiplayer
            .signals()
            .server_disconnected()
            .builder()
            .connect_other_mut(&self.to_gd(), |this: &mut Self| {
                this.end_game("Server disconnected.".to_string());
            });

        self.host_button
            .signals()
            .pressed()
            .builder()
            .connect_other_mut(&gd_ref, |this: &mut Self| {
                this.on_host_pressed();
            });

        self.join_button
            .signals()
            .pressed()
            .builder()
            .connect_other_mut(&gd_ref, |this: &mut Self| {
                this.on_join_pressed();
            });
    }
}

#[godot_api]
impl Lobby {
    fn set_status(&mut self, text: &str, is_ok: bool) {
        // Simple way to show status.
        if is_ok {
            self.status_ok.set_text(text);
            self.status_fail.set_text("");
        } else {
            self.status_ok.set_text("");
            self.status_fail.set_text(text);
        }
    }

    #[func]
    fn end_game(&mut self, with_error: String) {
        if self.base().has_node("/root/Pong") {
            // Erase immediately, otherwise network might show
            // errors (this is why we connected deferred above).
            self.base().get_node_as::<Node>("/root/Pong").free();
            self.base_mut().show();
        }
    
        let mut multiplayer = self.base().get_multiplayer().unwrap();
        multiplayer.set_multiplayer_peer(Gd::null_arg()); // Remove peer.
        self.host_button.set_disabled(false);
        self.join_button.set_disabled(false);

        self.set_status(&with_error, false);
    }

    fn on_host_pressed(&mut self) {
        let mut peer = ENetMultiplayerPeer::new_gd();
        self.peer = Some(peer.clone());
        // Set a maximum of 1 peer, since Pong is a 2-player game.
        let err = peer.create_server_ex(DEFAULT_PORT).max_clients(1).done();
        if err != Error::OK {
            // Is another server running?
            self.set_status("Can't host, address in use.", false);
            return;
        }
        peer.get_host()
            .unwrap()
            .compress(CompressionMode::RANGE_CODER);

        let mut multiplayer = self.base().get_multiplayer().unwrap();
        multiplayer.set_multiplayer_peer(&peer);
        self.host_button.set_disabled(true);
        self.join_button.set_disabled(true);
        self.set_status("Waiting for player...", true);
        let application_name = ProjectSettings::singleton()  
            .get_setting("application/config/name")  
            .to_string();  
        self.base_mut()  
            .get_window()  
            .unwrap()  
            .set_title(&format!("{application_name}: Server"));  
        // Only show hosting instructions when relevant.
        self.port_forward_label.set_visible(true);
        self.find_public_ip_button.set_visible(true);
    }

    fn on_join_pressed(&mut self) {
        let ip = self.address.get_text();
        if !ip.is_valid_ip_address() {
            self.set_status("IP address is invalid.", false);
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

        self.set_status("Connecting...", true);
        let application_name = ProjectSettings::singleton()  
            .get_setting("application/config/name")  
            .to_string();  
        self.base_mut()  
            .get_window()  
            .unwrap()  
            .set_title(&format!("{application_name}: Client")); 
    }

    fn _on_find_public_ip_pressed(&mut self) {
        let mut os = Os::singleton();
        os.shell_open("https://icanhazip.com/");
    }
}
