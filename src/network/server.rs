use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use bevy::{ecs::system::RunSystemOnce, prelude::*, tasks::IoTaskPool, utils::HashMap};

use super::{
    tcp::{
        host_server, AutoReconnect, ClientConfig, ClientDataUploader, ClientStream, ServerConfig,
        ServerDataUploader, ServerStreams,
    },
    Addr, NetworkState,
};

pub fn init_server(mut commands: Commands, net: Res<NetworkState>) {
    let config = ServerConfig {
        host_port: net.addr.port,
    };

    commands.insert_resource(config);
    commands.insert_resource(ServerDataUploader { list: Vec::new() });
    commands.insert_resource(ServerStreams {
        streams: Arc::new(Mutex::new(HashMap::new())),
    });

    commands.queue(|w: &mut World| {
        if let Err(connection_error) = w.run_system_once(host_server) {
            eprintln!("Encountered an error while trying to host server!: {connection_error:#?}");
        }
    });
}
