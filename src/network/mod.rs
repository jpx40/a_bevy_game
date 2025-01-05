pub mod tcp;
use std::{
    net::{IpAddr, Ipv4Addr},
    ops::Add,
    os::unix::net::SocketAddr,
};
pub mod server;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
pub mod client;
pub struct NetworkPlugin;

pub mod test_data;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkState>()
            .add_plugins((tcp::NetworkingPlugin))
            .add_plugins(test_data::TestDataPlugin);
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ServerID {
    pub addr: Addr,
    pub id: u32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Addr {
    pub ip: IpAddr,
    pub port: u16,
}
impl Default for Addr {
    fn default() -> Self {
        Self {
            ip: IpAddr::V4([0, 0, 0, 0].into()),
            port: 8080,
        }
    }
}

#[derive(Clone, Resource)]
pub struct NetworkState {
    pub addr: Addr,
    pub role: Role,
    pub id: u32,
}
impl Default for NetworkState {
    fn default() -> Self {
        Self {
            role: Role::None,
            addr: Addr::default(),
            id: 0,
        }
    }
}
#[repr(u8)]
#[derive(Clone, PartialEq)]
pub enum Role {
    None,
    Client,
    Server,
}

impl Default for Role {
    fn default() -> Self {
        Role::Client
    }
}
