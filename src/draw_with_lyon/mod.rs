use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::prelude::*;

pub struct DrawPlugin;

use crate::{utils::vec2, GameState};
impl Plugin for DrawPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
    }
}

fn setup(mut commands: Commands) {
    let rect = shapes::Rectangle {
        extents: Vec2::splat(175.0),
        origin: RectangleOrigin::Center,
        radii: Some(BorderRadii::single(25.0)),
    };

    let mut line = shapes::Line(vec2(100., -900.), vec2(100., 100.));
    commands.spawn((ShapeBuilder::with(&rect)
        .stroke((BLACK, 10.0))
        .fill(RED)
        .build(),));

    commands.spawn((ShapeBuilder::with(&line)
        .stroke((BLACK, 10.0))
        .fill(RED)
        .build(),));
}
