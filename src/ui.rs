use std::{fmt::format, process::exit};

use bevy::{
    color::palettes::css::*,
    math::ops,
    prelude::*,
    sprite::Anchor,
    text::{FontSmoothing, LineBreak, TextBounds},
};


pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        
        app.add_systems(OnEnter(crate::GameState::Playing), setup);
    }
}
#[derive(Component)]
pub struct Count(i32);
#[derive(Component)]
pub struct Counter;




fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("font/DroidSansMono/DroidSansMNerdFont-Regular.otf");
    commands.spawn((
        // Create a Text with multiple child spans.
        Text::new("Jumps: 0"),
        TextFont {
            font,
            font_size: 42.0,
            ..default()
        },
        Count(0),
    ));
}

#[derive(Event)]
pub enum UiAction {
    Increase,
    Clear,
    Nothing
}



pub fn update_counter(
    mut event: EventReader<UiAction>,
  mut  query: Query< &mut Count,With<Counter>>,
) {
    
    if query.is_empty() {
        
        return;
    }
    for e in event.read() {
        match e {
            UiAction::Increase => {
                let  mut count = query.single_mut();
                count.0 =  1 + count.0  ;
               
            }

            UiAction::Clear => {
                let mut  count = query.single_mut();
                count.0 = 0;
            }
        }
    }
}

pub fn update_text(
  mut  query: Query< ( &mut Count,&mut  TextSpan),With<Counter>>,
) {
    
   
    for ( mut count, mut text) in query.iter_mut() {

                count.0 =  1 + count.0  ;
                **text = format!("{}", count.0);
            

                count.0 = 0;
                **text = format!("{}", count.0);
                
            
        
    }
}
