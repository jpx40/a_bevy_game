use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::os::linux::net;
use std::time::Duration;

use crate::network::{Addr, Role, ServerID};
use crate::utils::port;
use crate::GameState;
use crate::{loading::TextureAssets, network::NetworkState};
use bevy::prelude::*;
use bevy::text::cosmic_text::rustybuzz::script::YI;
use bevy::text::cosmic_text::ttf_parser::Width;
use bevy_color::palettes::css::RED;

use bevy_simple_text_input::{
    TextInput, TextInputPlugin, TextInputSubmitEvent, TextInputSystem, TextInputTextColor,
    TextInputTextFont,
};

use bevy_tasks::TaskPool;
use rand::Rng;
use std::path::Path;
pub struct MenuPlugin;
const BORDER_COLOR_ACTIVE: Color = Color::srgb(0.75, 0.52, 0.99);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BACKGROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
            .add_systems(Update, host_button.run_if(in_state(GameState::Menu)))
            .add_systems(Update, update_net_text.run_if(in_state(GameState::Menu)))
            .add_systems(Update, trigger_reload.run_if(in_state(GameState::Menu)))
            .add_systems(
                Update,
                click_builder_button.run_if(in_state(GameState::Menu)),
            )
            .add_systems(
                OnExit(GameState::Menu),
                (cleanup_menu, cleanup_menu2, cleanup_menu3, cleanup_menu4),
            )
            .add_systems(Update, listener.after(TextInputSystem));
    }
}
#[derive(Component)]
pub struct MainCamera;
#[derive(Component)]
pub struct HostButton;
#[derive(Component)]
pub struct PlayButton;
#[derive(Component)]
pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct Menu;

#[derive(Component)]
struct HostButtonParent;

fn update_net_text(
    mut commands: Commands,
    mut net_q: ResMut<NetworkState>,
    mut net_ids: Query<&mut NetworkStateComp, With<NetworkStateComp>>,
    mut text_query: Query<(&mut Text, Entity), (With<NetworkText>, Without<AddressSetted>)>,
) {
    if text_query.is_empty() {
        return;
    }
    if net_q.role == Role::None {
        return;
    }
    if net_ids.is_empty() {
        return;
    }
    let (mut text_query, e) = text_query.single_mut();

    let mut m = net_ids.single_mut();
    let mut id = net_q.id;
    if net_q.role == Role::Server {
        loop {
            if !m.server_id.contains_key(&id) {
                break;
            } else {
                id = rand::thread_rng().gen_range(100..100000);
            }
        }

        m.server_id.insert(
            id,
            ServerID {
                addr: net_q.addr.clone(),
                id,
            },
        );
        net_q.id = id;
        let map = m.server_id.clone();

        commands.queue(move |world: &mut World| {
            let path = Path::new(r#"./net_id.db"#);
            match File::create(path) {
                Ok(mut file) => {
                    let v = serde_json::to_vec(&map).unwrap();
                    file.write_all(&v);
                }
                Err(_) => (),
            }
        });
    }
    **text_query = format!(
        "IP: {}\nPort: {}\nID: {}",
        net_q.addr.ip.to_string(),
        net_q.addr.port,
        id
    );
    commands.entity(e).insert(AddressSetted);
}
#[derive(Component)]
struct AddressSetted;
fn host_button(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonColors),
        (Changed<Interaction>, With<HostButton>, With<Button>),
    >,

    text_query2: Query<Entity, With<NetworkText>>,
    mut text_query: Query<&mut Text, With<NetworkTextRole>>,
) {
    if interaction_query.is_empty() {
        return;
    }

    let (interaction, mut color, button_colors) = interaction_query.single_mut();
    // if  text_query.is_empty()  {
    //     return;
    //          }

    match *interaction {
        Interaction::Pressed => {
            commands.queue(move |world: &mut World| {
                let net = world.get_resource_mut::<NetworkState>();

                if let Some(mut net) = net {
                    net.role = Role::Server;

                    let port = port::free_local_ipv4_port_in_range(8080..10000);
                    if let Some(port) = port {
                        net.addr.port = port;
                    }
                    let id = rand::thread_rng().gen_range(100..100000);

                    net.id = id;
                }
            });

            for mut text in &mut text_query {
                **text = "Server".to_string();
            }
            for e in text_query2.iter() {
                commands.entity(e).remove::<AddressSetted>();
            }
        }
        Interaction::Hovered => {
            *color = button_colors.hovered.into();
        }
        Interaction::None => {
            *color = button_colors.normal.into();
        }
    }
}
#[derive(Component)]
pub struct NetworkText;

#[derive(Component)]
pub struct NetworkStateComp {
    pub addr: Addr,
    pub server_id: HashMap<u32, ServerID>,
}
#[derive(Component)]
pub struct NetworkTextRole;

#[derive(Component)]
pub struct NetworkTextInput;
#[derive(Component)]
pub struct BuilderButton;
#[derive(Component)]
pub struct ReloadTimer {
    pub timer: Timer,
}

fn trigger_reload(
    time: Res<Time>,
    mut query: Query<(&mut ReloadTimer, &mut NetworkStateComp), With<NetworkStateComp>>,
) {
    if query.is_empty() {
        return;
    }
    let (mut timer, mut state) = query.single_mut();
    timer.timer.tick(Duration::from_secs_f32(time.delta_secs()));

    if timer.timer.finished() {
        // let mut net_id_map = HashMap::new();
        //
        let pool = TaskPool::new();
        let results = pool.scope(|s| {
            s.spawn(async {
                let path = Path::new(r#"./net_id.db"#);
                if path.exists() {
                    let mut buf = Vec::new();

                    let file = File::open(path);
                    match file {
                        Ok(mut file) => match file.read_to_end(&mut buf) {
                            Ok(_) => match serde_json::from_slice::<HashMap<u32, ServerID>>(&buf) {
                                Ok(v) => Some(v),
                                Err(err) => {
                                    eprintln!("Failed to parse JSON: Error: {}", err);
                                    None
                                }
                            },
                            Err(err) => {
                                eprintln!("Failed to read file: Error: {}", err);
                                None
                            }
                        },
                        Err(err) => {
                            eprintln!("Failed to opon net_id.db, Error: {}", err);
                            None
                        }
                    }
                } else {
                    None
                }
                // return some other value from the first task
            });
        });

        for v in results {
            if let Some(map) = v {
                state.server_id.extend(map);
            }
        }
    }
}
fn setup_menu(addr: Res<NetworkState>, mut commands: Commands, textures: Res<TextureAssets>) {
    let pool = TaskPool::new();
    let mut net_id_map = HashMap::new();
    let results = pool.scope(|s| {
        s.spawn(async {
            let path = Path::new(r#"./net_id.db"#);
            if path.exists() {
                let mut buf = Vec::new();

                let file = File::open(path);
                match file {
                    Ok(mut file) => match file.read_to_end(&mut buf) {
                        Ok(_) => match serde_json::from_slice::<HashMap<u32, ServerID>>(&buf) {
                            Ok(v) => Some(v),
                            Err(err) => {
                                eprintln!("Failed to parse JSON: Error: {}", err);
                                None
                            }
                        },
                        Err(err) => {
                            eprintln!("Failed to read file: Error: {}", err);
                            None
                        }
                    },
                    Err(err) => {
                        eprintln!("Failed to opon net_id.db, Error: {}", err);
                        None
                    }
                }
            } else {
                None
            }
            // return some other value from the first task
        });
    });

    for v in results {
        if let Some(map) = v {
            net_id_map.extend(map);
        }
    }

    info!("menu");
    commands.spawn((Camera2d, Msaa::Off, IsDefaultUiCamera, MainCamera));

    commands.spawn((
        NetworkStateComp {
            addr: addr.addr.clone(),
            server_id: net_id_map,
        },
        ReloadTimer {
            timer: Timer::from_seconds(5., TimerMode::Repeating),
        },
    ));
    commands.spawn((
        Text::new("Client"),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 67.0,
            ..default()
        },
        // Set the justification of the Text
        TextLayout::new_with_justify(JustifyText::Center),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        NetworkTextRole,
    ));
    let button_colors = ButtonColors::default();
    commands
        .spawn((
            HostButtonParent,
            Node {
                width: Val::Px(400.0),
                height: Val::Px(200.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
        ))
        .with_children(|children| {
            children
                .spawn((
                    HostButton,
                    Button,
                    Node {
                        width: Val::Px(140.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(button_colors.normal),
                    button_colors,
                    // ChangeState(GameState::Playing),
                ))
                .with_child((
                    Text::new("Host"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ));
            children.spawn(
                ((
                    NetworkText,
                    Text::new(format!(
                        "IP: {}\nPort: {}",
                        addr.addr.ip.to_string(),
                        addr.addr.port
                    )),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                )),
            );
        });
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(140.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(button_colors.normal),
                    button_colors,
                    ChangeState(GameState::Playing),
                ))
                .with_child((
                    Text::new("Play"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ));
        })
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    BuilderButton,
                    Button,
                    Node {
                        width: Val::Px(140.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(button_colors.normal),
                    button_colors,
                    ChangeState(GameState::Builder),
                ))
                .with_child((
                    Text::new("Build"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ));
        })
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(200.0),
                    border: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                BorderColor(BORDER_COLOR_ACTIVE),
                BackgroundColor(BACKGROUND_COLOR),
                TextInput,
                TextInputTextFont(TextFont {
                    font_size: 34.,
                    ..default()
                }),
                NetworkTextInput,
                TextInputTextColor(TextColor(TEXT_COLOR)),
            ));
        });
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                bottom: Val::Px(5.),
                width: Val::Percent(100.),
                position_type: PositionType::Absolute,
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(170.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::NONE),
                    ButtonColors {
                        normal: Color::NONE,
                        ..default()
                    },
                    OpenLink("https://bevyengine.org"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Made with Bevy"),
                        TextFont {
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                    ));
                    parent.spawn((
                        ImageNode {
                            image: textures.bevy.clone(),
                            ..default()
                        },
                        Node {
                            width: Val::Px(32.),
                            ..default()
                        },
                    ));
                });
            children
                .spawn((
                    PlayButton,
                    Button,
                    Node {
                        width: Val::Px(170.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    ButtonColors {
                        normal: Color::NONE,
                        hovered: Color::linear_rgb(0.25, 0.25, 0.25),
                    },
                    OpenLink("https://github.com/NiklasEi/bevy_game_template"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Open source"),
                        TextFont {
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                    ));
                    parent.spawn((
                        ImageNode::new(textures.github.clone()),
                        Node {
                            width: Val::Px(32.),
                            ..default()
                        },
                    ));
                });
        });
}

#[derive(Component)]
struct ChangeState(GameState);

#[derive(Component)]
struct OpenLink(&'static str);

fn click_play_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
            Option<&OpenLink>,
        ),
        (Changed<Interaction>, With<Button>, With<PlayButton>),
    >,
) {
    for (interaction, mut color, button_colors, change_state, open_link) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                } else if let Some(link) = open_link {
                    if let Err(error) = webbrowser::open(link.0) {
                        warn!("Failed to open link {error:?}");
                    }
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}
fn click_builder_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
            Option<&OpenLink>,
        ),
        (Changed<Interaction>, With<Button>, With<BuilderButton>),
    >,
) {
    for (interaction, mut color, button_colors, change_state, open_link) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                } else if let Some(link) = open_link {
                    if let Err(error) = webbrowser::open(link.0) {
                        warn!("Failed to open link {error:?}");
                    }
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}
fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
fn cleanup_menu2(mut commands: Commands, menu: Query<Entity, With<NetworkText>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn cleanup_menu3(mut commands: Commands, menu: Query<Entity, With<NetworkTextRole>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn cleanup_menu4(mut commands: Commands, menu: Query<Entity, With<HostButtonParent>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
fn listener(
    mut events: EventReader<TextInputSubmitEvent>,
    mut net_res: ResMut<NetworkState>,
    net_ids: Query<&NetworkStateComp, With<NetworkStateComp>>,
    q: Query<Entity, With<NetworkTextInput>>,

    mut text_query2: Query<&mut Text, With<NetworkTextRole>>,
) {
    // if net_ids.is_empty() {
    //     return;
    // }
    // if q.is_empty() {
    //     return;
    // }

    // let e = q.single();

    // let m = net_ids.single();

    for event in events.read() {
        for e in q.iter() {
            if event.entity == e {
                for m in net_ids.iter() {
                    match event.value.parse::<u32>() {
                        Ok(i) => {
                            if net_res.id != i {
                                if m.server_id.contains_key(&i) {
                                    let v = m.server_id.get(&i).unwrap();
                                    net_res.addr = v.addr.clone();
                                    net_res.role = Role::Client;
                                    net_res.id = i;

                                    for mut t in text_query2.iter_mut() {
                                        **t = "Client".to_string();
                                    }
                                } else {
                                    eprintln!("Server not found");
                                }
                            } else {
                                eprintln!("Already connected!");
                            }
                        }
                        Err(_) => eprintln!("Wrong Format of id"),
                    }
                }
            }
            // info!("{:?} submitted: {}", event.entity, event.value);
        }
    }
}
