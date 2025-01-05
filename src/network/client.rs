use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    vec,
};

use bevy::{ecs::system::RunSystemOnce, prelude::*, tasks::IoTaskPool, utils::HashMap};

use super::{
    tcp::{
        connect_to_server, AutoReconnect, ClientConfig, ClientDataUploader, ClientStream,
        ServerDataUploader,
    },
    Addr, NetworkState,
};

pub fn init_client(mut commands: Commands, net: Res<NetworkState>) {
    let config = ClientConfig {
        server_address: SocketAddr::new(net.addr.ip, net.addr.port),
        auto_reconnect: AutoReconnect::None,
    };

    commands.insert_resource(config);
    commands.insert_resource(ClientDataUploader { list: None });
    commands.insert_resource(ClientStream {
        stream: Arc::new(Mutex::new(None)),
    });

    commands.queue(|w: &mut World| {
         if let Err(connection_error) = w.run_system_once(connect_to_server){
             // let error_text = format!("Encountered an error while trying to connect to the server!: {connection_error:#?}");
             eprintln!("Encountered an error while trying to connect to the server!: {connection_error:#?}");
         }
     });
}
