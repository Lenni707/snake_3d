use bevy::prelude::*;

use crate::snake::Snake;

const FONT_PATH: &str = "font/PixelOperator8-Bold.ttf";

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui)
            .add_systems(Update, update_score);
    }
}

#[derive(Component)]
struct ScoreText;

fn spawn_ui(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(FONT_PATH);

    cmd.spawn((
        Text::new("Score: "),
        TextFont {
            font,
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Px(16.0),
            ..default()
        },
        ScoreText,
    ));
}

fn update_score(
    snake_q: Query<&Snake>,
    mut text_q: Query<&mut Text, With<ScoreText>>,
) {
    let Ok(snake) = snake_q.single() else { return };
    let Ok(mut text) = text_q.single_mut() else { return };
    **text = format!("Score: {}", snake.score);
}