use bevy::prelude::*;

use crate::gamestate::GameState;
use crate::misc::combo::ComboCounter;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct ComboText;

pub fn hud_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 60.0,
                color: Color::GOLD,
            }),
        ]),
        ScoreText,
    ));
}

pub fn combo_text_update(mut query: Query<&mut Text, With<ComboText>>, combo: Res<ComboCounter>) {
    let mut text = query.single_mut();
    text.sections[1].value = combo.multiplier_for_count().to_string();
}
pub fn score_text_plugin(mut query: Query<&mut Text, With<ScoreText>>) {
    let text = query.single_mut();
}

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(hud_setup))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(combo_text_update)
                    .with_system(score_text_plugin),
            );
    }
}
