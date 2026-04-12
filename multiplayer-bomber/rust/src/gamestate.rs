use godot::{classes::{Control, ENetMultiplayerPeer}, prelude::*};

// Default game server port. Can be any number between 1024 and 49151.
// Not on the list of registered or common ports as of May 2024:
// https://en.wikipedia.org/wiki/List_of_TCP_and_UDP_port_numbers
const DEFAULT_PORT : i32 = 10567;

// The maximum number of players.
const MAX_PEERS : i32 = 12;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameState {
    base: Base<Node>,
    peer: Option<Gd<ENetMultiplayerPeer>>,

    /// Our local player's name.
    #[export]
    pub player_name: GString,

    /// Names for remote players in id:name format.
    #[export]
    pub players: Dictionary<i64, GString>,
    #[export]
    players_ready: Array<i64>,
}

#[godot_api]
impl INode for GameState {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            peer: None,
            player_name: GString::from("The Warrior"),
            players: Dictionary::new(),
            players_ready: Array::new(),
        }
    }

    fn ready(&mut self) {
        let multiplayer = self.base().get_multiplayer().expect("Should be initialized");
        multiplayer.signals().peer_connected().connect_other(self, GameState::player_connected);
        multiplayer.signals().peer_disconnected().connect_other(self, GameState::player_disconnected);
        multiplayer.signals().connected_to_server().connect_other(self, GameState::_connected_ok);
        multiplayer.signals().server_disconnected().connect_other(self, GameState::_server_disconnected);
        multiplayer.signals().connection_failed().connect_other(self, GameState::_connected_fail);
    }
}

#[godot_api]
impl GameState {
    // Signals to let lobby GUI know what's going on.
    #[signal]
    pub fn player_list_changed();
    #[signal]
    pub fn connection_failed();
    #[signal]
    pub fn connection_succeeded();
    #[signal]
    pub fn game_ended();
    // in the original code, this was an int for some reason, but this should actually be a string
    #[signal]
    pub fn game_error(what: GString);

    // Callback from SceneTree.
    fn player_connected(&mut self, id: i64) {
        // Registration of a client begins here, tell the connected player that we are here.
        let player_name = self.player_name.clone();
        self.base_mut().rpc_id(id, "register_player", &[player_name.to_variant()]);
    }

    // Callback from SceneTree.
    fn player_disconnected(&mut self, id: i64) {
        if self.base().has_node("/root/World") {
            if self.base().get_multiplayer().expect("Should be initialized").is_server() {
                // Handle server-side logic for player disconnection.
                let error = format!("Player {id} disconnected");
                self.signals().game_error().emit(&error);
            }
        }
        else {
            // Game is not in progress.
            // Unregister this player.
            self.unregister_player(id);
        }
    }

    // Callback from SceneTree, only for clients (not server).
    fn _connected_ok(&mut self) {
        // We just connected to a server
        self.signals().connection_succeeded().emit();
    }

    // Callback from SceneTree, only for clients (not server).
    fn _server_disconnected(&mut self) {
        self.signals().game_error().emit(&GString::from("Server disconnected"));
        self.signals().game_ended().emit();
    }

    // Callback from SceneTree, only for clients (not server).
    fn _connected_fail(&mut self) {
        self.peer = None; // Remove peer
        self.signals().connection_failed().emit();
    }


    // Lobby management functions.
    #[rpc(any_peer)]
    pub fn register_player(&mut self, new_player_name: GString) {
        let id = self.base().get_multiplayer().expect("Should be initialized").get_remote_sender_id();
        let _ = self.players.insert(id as i64, &new_player_name);
        self.signals().player_list_changed().emit();
    }


    #[func]
    pub fn unregister_player(&mut self, id: i64) {
        self.players.remove(id);
        self.signals().player_list_changed().emit();
    }

    #[rpc(call_local)]
    pub fn load_world(&mut self) {
        // Change scene.
        let world: Gd<Node2D> = load::<PackedScene>("res://world.tscn").instantiate_as::<Node2D>();
        let base = self.base();
        let mut tree = base.get_tree();
        let mut root = tree.get_root().expect("Should have root");
        root.add_child(&world);
        root.upcast::<Node>().get_node_as::<Control>("Lobby").hide();
        let mut score = world.get_node_or_null("Score").expect("Should have score");
        // Do janky untyped function call for now
        let multiplayer = base.get_multiplayer().expect("Should be initialized");
        score.call("add_player", vslice![multiplayer.get_unique_id(), self.player_name.clone()]);
        for (id, name) in &self.players {
            score.call("add_player", vslice![id, name]);
        }

        // unpause and unleash the game!
        tree.set_pause(false);
    }

    #[func]
    pub fn host_game(&mut self, new_player_name: GString){
        self.player_name = new_player_name;
        let mut peer = ENetMultiplayerPeer::new_gd();
        let _ = peer.create_server_ex(DEFAULT_PORT).max_clients(MAX_PEERS).done();
        self.base().get_multiplayer().unwrap().set_multiplayer_peer(&peer);
        self.peer = Some(peer);
    }

    #[func]
    pub fn join_game(&mut self, ip: GString, new_player_name: GString){
        self.player_name = new_player_name;
        let mut peer = ENetMultiplayerPeer::new_gd();
        let _ = peer.create_client_ex(&ip, DEFAULT_PORT).done();
        self.base().get_multiplayer().unwrap().set_multiplayer_peer(&peer);
        self.peer = Some(peer);
    }

    #[func]
    pub fn get_player_list(&self) -> Array<GString> {
        self.players.values_array()
    }

    #[func]
    pub fn begin_game(&mut self) {
        assert!(self.base().get_multiplayer().unwrap().is_server());
        {
            let mut base = self.base_mut();
            base.rpc("load_world", &[]);
        }
        let mut world = self.base().get_node_as::<Node2D>("/root/World");
        let player_scene = load::<PackedScene>("res://player.tscn");
        // Create a dictionary with peer ID. and respective spawn points.
	    // TODO: This could be improved by randomizing spawn points for players.
        // Honestly, this entire bit of code is really janky and should be refactored at some point
        let mut spawn_points = Dictionary::<i64, i64>::new();
        let _ = spawn_points.insert(1, 0); // Server host is always player 1, and spawns at spawn point 0.

        for (id, _) in &self.players {
            let spawn_point = spawn_points.len() as i64; // Get the next spawn point index.
            let _ = spawn_points.insert(id, spawn_point);
        }
        for (id, spawn_point) in &spawn_points {
            let spawn_pos = world.get_node_as::<Node2D>(&format!("SpawnPoints/{spawn_point}")).get_position();
            let mut player = player_scene.instantiate_as::<Node2D>();
            player.set("synced_position", &spawn_pos.to_variant());
            player.set_name(&id.to_string());
            world.get_node_as::<Node2D>("Players").add_child(&player);
            // The RPC must be called after the player is added to the scene tree.
            let player_name = if self.base().is_multiplayer_authority() {
                self.player_name.clone()
            } else {
                self.players.get(id).expect("Should have player name")
            };
            player.rpc( "set_player_name", vslice![player_name]);
        }
    }

    #[func]
    pub fn end_game(&mut self) {
        let base = self.base();
        if let Some(mut world) = base.try_get_node_as::<Node2D>("/root/World") {
            world.queue_free();
        }
        self.signals().game_ended().emit();
        self.players.clear();
    }

    #[func]
    pub fn get_player_color(p_name: GString) -> Color {
        Color::from_hsv(godot::global::wrapf(p_name.hash_u32() as f64 * 0.001, 0.0, 1.0), 0.6, 1.0)
    }
}
