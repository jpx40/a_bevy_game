use crate::menu::ButtonColors;
use crate::{menu::MainCamera, utils::vec3, GameState};
use accesskit::{Node as Accessible, Role};
use avian2d::{math::*, prelude::*};
use bevy::text::FontLoader;
use bevy::window::PrimaryWindow;
use bevy::{
    a11y::AccessibilityNode,
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::focus::HoverMap,
    prelude::*,
    winit::WinitSettings,
};
use bevy::{color::palettes::css::*, prelude::*};
use bevy_color::palettes::tailwind::ORANGE_100;
use glam::vec2;
use ops::powf;

const FONT_SIZE: f32 = 20.;
const LINE_HEIGHT: f32 = 21.;
pub struct BuilderPlugin;

#[derive(Resource)]
pub struct SelectedColor(pub Color);
impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DrawState { list: Vec::new() })
            .insert_resource(SelectedColor(ORANGE.into()))
            .add_systems(OnEnter(GameState::Builder), spawn_layout)
            .add_systems(
                Update,
                update_scroll_position.run_if(in_state(GameState::Builder)),
            )
            .add_systems(Update, color_button.run_if(in_state(GameState::Builder)))
            .add_systems(Update, draw_rect.run_if(in_state(GameState::Builder)));
    }
}

#[derive(Component)]
pub struct Plattform;
#[derive(Component)]
struct SaveButton;
#[derive(Component)]
struct SaveButtonParent;
#[derive(Clone, Copy)]
pub enum Kind {
    Plattform,
}

fn spawn_nested_text_bundle(builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
    builder.spawn((
        Text::new(text),
        TextFont { font, ..default() },
        TextColor::BLACK,
    ));
}

fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Fira_Sans/FiraSans-Bold.ttf");

    // Top-level grid (app frame)
    commands
        .spawn((
            Node {
                // Use the CSS Grid algorithm for laying out this node
                display: Display::Grid,
                // Make node fill the entirety of its parent (in this case the window)
                width: Val::Percent(20.0),
                height: Val::Percent(100.0),
                // Set the grid to have 2 columns with sizes [min-content, minmax(0, 1fr)]
                //   - The first column will size to the size of its contents
                //   - The second column will take up the remaining available space
                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
                // Set the grid to have 3 rows with sizes [auto, minmax(0, 1fr), 20px]
                //  - The first row will size to the size of its contents
                //  - The second row take up remaining available space (after rows 1 and 3 have both been sized)
                //  - The third row will be exactly 20px high
                grid_template_rows: vec![
                    GridTrack::auto(),
                    GridTrack::flex(1.0),
                    GridTrack::px(10.),
                ],
                ..default()
            },
            BackgroundColor(Color::WHITE),
        ))
        .with_children(|builder| {
            // Header
            builder
                .spawn(Node {
                    display: Display::Grid,
                    // Make this node span two grid columns so that it takes up the entire top tow
                    grid_column: GridPlacement::span(2),
                    padding: UiRect::all(Val::Px(6.0)),
                    ..default()
                })
                .with_children(|builder| {
                    spawn_nested_text_bundle(builder, font.clone(), "Bevy CSS Grid Layout Example");
                });

            // Main content grid (auto placed in row 2, column 1)
            builder
                .spawn((
                    Node {
                        // Make the height of the node fill its parent
                        height: Val::Percent(100.0),
                        // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
                        // As the height is set explicitly, this means the width will adjust to match the height
                        aspect_ratio: Some(0.25),
                        // Use grid layout for this node
                        display: Display::Grid,
                        // Add 24px of padding around the grid
                        padding: UiRect::all(Val::Px(14.0)),
                        // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
                        // This creates 4 exactly evenly sized columns
                        grid_template_columns: RepeatedGridTrack::flex(3, 0.25),
                        // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                        // This creates 4 exactly evenly sized rows
                        // grid_template_rows: RepeatedGridTrack::flex(3, 0.25),
                        // Set a 12px gap/gutter between rows and columns
                        row_gap: Val::Px(6.0),
                        column_gap: Val::Px(6.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                ))
                .with_children(|builder| {
                    // Note there is no need to specify the position for each grid item. Grid items that are
                    // not given an explicit position will be automatically positioned into the next available
                    // grid cell. The order in which this is performed can be controlled using the grid_auto_flow
                    // style property.

                    item_rect(builder, ORANGE);
                    item_rect(builder, BISQUE);
                    item_rect(builder, BLUE);
                    item_rect(builder, CRIMSON);
                    item_rect(builder, AQUA);
                    item_rect(builder, ORANGE_RED);
                    item_rect(builder, DARK_GREEN);
                    item_rect(builder, FUCHSIA);
                    item_rect(builder, TEAL);
                    item_rect(builder, ALICE_BLUE);
                    item_rect(builder, CRIMSON);
                    item_rect(builder, ANTIQUE_WHITE);
                    item_rect(builder, YELLOW);
                    item_rect(builder, DEEP_PINK);
                    item_rect(builder, YELLOW_GREEN);
                    item_rect(builder, SALMON);
                });

            // Footer / status bar
            builder.spawn((
                Node {
                    // Make this node span two grid column so that it takes up the entire bottom row
                    grid_column: GridPlacement::span(2),
                    ..default()
                },
                BackgroundColor(WHITE.into()),
            ));

            // Modal (absolutely positioned on top of content - currently hidden: to view it, change its visibility)
            builder.spawn((
                // Node {
                //     position_type: PositionType::Absolute,
                //     margin: UiRect {
                //         top: Val::Px(100.),
                //         bottom: Val::Auto,
                //         left: Val::Auto,
                //         right: Val::Auto,
                //     },
                //     width: Val::Percent(60.),
                //     height: Val::Px(30.),
                //     max_width: Val::Px(60.),
                //     ..default()
                // },
                // Visibility::Hidden,
                // BackgroundColor(Color::WHITE.with_alpha(0.8)),
            ));
        });
}

#[derive(Component)]
pub struct ColorButton(pub Color);
/// Create a colored rectangle node. The node has size as it is assumed that it will be
/// spawned as a child of a Grid container with `AlignItems::Stretch` and `JustifyItems::Stretch`
/// which will allow it to take its size from the size of the grid area it occupies.
fn item_rect(builder: &mut ChildBuilder, color: Srgba) {
    builder
        .spawn((
            Node {
                display: Display::Grid,
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(BLACK.into()),
        ))
        .with_children(|builder| {
            builder.spawn((
                Button,
                ColorButton(color.into()),
                Node::default(),
                BackgroundColor(color.into()),
            ));
        });
}

fn color_button(
    mut sel_color: ResMut<SelectedColor>,
    mut interaction_query: Query<
        (&Interaction, &ColorButton),
        (Changed<Interaction>, With<Button>, With<ColorButton>),
    >,
) {
    for (interaction, color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                sel_color.0 = color.0.clone();
                let color = color.0.to_srgba().to_hex();

                println!("{color}");
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_colors = ButtonColors::default();
    commands
        .spawn((
            SaveButtonParent,
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
                    SaveButton,
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
                    Text::new("Save"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ));
        });
}

#[derive(Clone, Copy)]
pub struct Form {
    translation: Vec3,
    size: Vec3,
    color: Color,
    kind: Kind,
    id: u16,
}
impl Default for Form {
    fn default() -> Self {
        Self {
            translation: vec3(0., 0., 0.),
            size: vec3(0., 0., 0.),
            color: ORANGE.into(),
            kind: Kind::Plattform,
            id: 0,
        }
    }
}

#[derive(Resource)]
pub struct DrawState {
    pub list: Vec<Form>,
}

impl DrawState {
    pub fn len(&self) -> usize {
        self.list.len()
    }
}

pub fn new_plattform(pos: Vec3, size: Vec2) -> (Sprite, Transform, RigidBody, Collider, Plattform) {
    (
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.8),
            custom_size: Some(size),
            ..default()
        },
        Transform::from_xyz(pos.x, pos.y, pos.z),
        RigidBody::Static,
        Collider::rectangle(size.x, size.y),
        Plattform,
    )
}
#[derive(Component)]
struct StartPos {
    pos: Vec3,
}
#[derive(Component)]
pub struct LenText;
#[derive(Component)]
pub struct LenLine {
    pub start: Vec2,
    pub end: Vec2,
    pub len: f32,
    pub is_disabled: bool,
}
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct InProgress;

#[derive(Component)]
pub struct PlattformID(pub u16);

fn draw_rect(
    mut state: ResMut<DrawState>,
    color: Res<SelectedColor>,
    mut commands: Commands,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,

    q_window: Query<&Window, With<PrimaryWindow>>,

    mut current: Query<
        (&mut Sprite, Entity, &mut Transform, &StartPos),
        (With<Plattform>, With<PlattformID>, With<InProgress>),
    >,
    button: Res<ButtonInput<MouseButton>>,
) {
    let (camera, camera_transform) = q_camera.single();
    if button.pressed(MouseButton::Left) {
        let size = q_window.single().size();
        let psize = q_window.single().physical_size();
        if let Some(position) = q_window.single().cursor_position() {
            let position = camera.viewport_to_world(camera_transform, position);

            if let Ok(mut ray) = position {
                let ray = ray.origin.truncate();
                let position = vec2(ray.x, ray.y);
                if current.is_empty() {
                    let mut el = Form::default();
                    let id = (state.len() + 1) as u16;
                    el.id = id;
                    el.translation = vec3(position.x, position.y, 0.);
                    el.color = color.0.clone();
                    commands.spawn(((
                        Sprite {
                            color: el.color,
                            custom_size: Some(vec2(el.size.x, el.size.y)),
                            ..default()
                        },
                        StartPos {
                            pos: vec3(position.x, position.y, 0.),
                        },
                        Transform::from_xyz(position.x, position.y, 0.),
                        RigidBody::Static,
                        Plattform,
                        InProgress,
                        PlattformID(id),
                    ),));
                    state.list.push(el);
                } else {
                    for (mut sprite, _, mut t, start) in current.iter_mut() {
                        let len = (powf((start.pos.x - position.x), 2.)
                            + powf((start.pos.y - position.y), 2.))
                        .sqrt();

                        let tx = start.pos.x + (len / 2.);
                        let ty = start.pos.y - (len / 2.);
                        t.translation.x = tx;
                                            t.translation.y = ty;
                        let width = {
                            let mut l = position.x - start.pos.x;

                            if l < 0. {
                                l = l * -1.;
                                l
                            } else {
                                l
                            }
                        };

                        let height = {
                            let mut l = position.y - start.pos.y;

                            if l < 0. {
                                l = l * -1.;
                                l
                            } else {
                                l
                            }
                        };
                    
                        let l = state.len() - 1;
                        state.list[l].size = vec3(width, height, 0.);
                        sprite.custom_size = Some(vec2(width, height))
                        // gizmos.line_2d(line.start, position, WHITE);
                        // line.end = position;

                        // let len = (powf((line.start.x - line.end.x), 2.)
                        //     + powf((line.start.y - line.end.y), 2.))
                        // .sqrt();

                        // if !len_text.is_empty() {
                        //     let mut text = len_text.single_mut();
                        //     **text = format!("Len: {:.2}", len);
                        // }
                    }
                }
            }
        }
    } else {
        for (_, e, _, _) in current.iter() {
            commands.entity(e).remove::<InProgress>();
        }
    }
}

/// Updates the scroll position of scrollable nodes in response to mouse input
pub fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (mut dx, mut dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (
                mouse_wheel_event.x * LINE_HEIGHT,
                mouse_wheel_event.y * LINE_HEIGHT,
            ),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        if keyboard_input.pressed(KeyCode::ControlLeft)
            || keyboard_input.pressed(KeyCode::ControlRight)
        {
            std::mem::swap(&mut dx, &mut dy);
        }

        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_x -= dx;
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}
