use bevy::prelude::*;

use crate::gamestate::GameState;
use crate::misc::combo::ComboCounter;
use crate::misc::score::ScoreTracker;

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
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 30.0,
                color: Color::GOLD,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        ScoreText,
    ));
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 30.0,
            color: Color::GOLD,
        })])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(25.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        ComboText,
    ));
}

pub fn score_text_update(mut query: Query<&mut Text, With<ScoreText>>, score: Res<ScoreTracker>) {
    let mut text = query.single_mut();
    text.sections[1].value = score.to_string();
}
pub fn combo_text_update(mut query: Query<&mut Text, With<ComboText>>, combo: Res<ComboCounter>) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("{}x", combo.multiplier()) ;
}

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(hud_setup))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(combo_text_update)
                    .with_system(score_text_update),
            );
    }
}
