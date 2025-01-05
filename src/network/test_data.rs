use std::io::Write;

use bevy::{ecs::system::RunSystemOnce, prelude::*, tasks::IoTaskPool, utils::HashMap};
use nalgebra::Translation;
use serde::Serialize;

use crate::{player::Player, utils::vec2};

use super::tcp::{ClientDataUploader, ServerDataReadEvent};

pub struct TestDataPlugin;

impl Plugin for TestDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, receive_data)
            .add_systems(Update, send_data);
    }
}

#[derive(Serialize, serde::Deserialize)]
pub enum Data {
    Position(Position),
}

#[derive(Serialize, serde::Deserialize)]
pub struct Position(pub Vec2);

pub fn send_data(
    mut client: Option<ResMut<ClientDataUploader>>,
    query: Query<&Transform, With<Player>>,
) {
    if let Some(mut client) = client {
        for t in query.iter() {
            let res = client.upload(
                Data::Position(Position(vec2(t.translation.x, t.translation.y))),
                0,
            );
        }
    }
}

pub fn receive_data(
    mut server_data_reader: EventReader<ServerDataReadEvent>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for event in server_data_reader.read() {
        match bincode::deserialize::<Data>(&event.data_packet.bytes) {
            Ok(data) => match data {
                Data::Position(p) => {
                    for mut t in query.iter_mut() {
                        t.translation.x = p.0.x;
                        t.translation.y = p.0.y;
                    }
                }
            },
            Err(_) => {}
        }
    }
}
