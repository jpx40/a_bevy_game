use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

use crate::{utils::AddSystem, GameState};
mod gallary3d;
pub struct VectorPlugin;

impl Plugin for VectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin::default())
            .add(OnEnter(GameState::Playing), setup);
    }
}

fn draw(mut painter: ShapePainter) {
    // Draw a circle
    painter.circle(100.0);
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let config = CanvasConfig::new(1024, 1024);
    commands.spawn_canvas(images.as_mut(), config);
}

fn draw_shapes(time: Res<Time>, mut painter: ShapePainter, canvas: Query<(Entity, &Canvas)>) {
    let (canvas_e, canvas) = canvas.single();
    painter.image(canvas.image.clone(), Vec2::splat(20.));

    painter.set_canvas(canvas_e);
    painter.set_scale(Vec3::ONE * 48.0);

    gallary3d::gallery(painter, time.elapsed_secs(), 0..15);
}
