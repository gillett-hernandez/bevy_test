use bevy::prelude::*;

use crate::gamestate::GameState;
use crate::misc::combo::ComboCounter;
use crate::misc::score::ScoreTracker;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]

pub struct ComboText;

pub fn hud_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Query<&ScoreText>,
    combo: Query<&ComboText>,
) {
    if score.is_empty() && combo.is_empty() {
        let bold_font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
        let normal_font: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");

        let bold_style = TextStyle {
            font: bold_font,
            font_size: 30.0,
            color: Color::White,
        };

        let normal_style = TextStyle {
            font: normal_font,
            font_size: 30.0,
            color: Color::GOLD,
        };

        // Text with multiple sections
        commands.spawn(
            (
                // Create a TextBundle that has a Text with a list of sections.
                Text2dBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new("Score:", bold_style),
                            TextSection::new("", normal_style),
                        ],
                        alignment: todo!(),
                        linebreak_behaviour: todo!(),
                    },

                    text_anchor: todo!(),
                    text_2d_bounds: todo!(),
                    transform: todo!(),
                    global_transform: todo!(),
                    visibility: todo!(),
                    computed_visibility: todo!(),
                }
            ),
        );
    }
    //     [
    //         TextSection::new(
    //             "Score: ",
    //             TextStyle {
    //                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //                 font_size: 30.0,
    //                 color: Color::WHITE,
    //             },
    //         ),
    //         TextSection::from_style(TextStyle {
    //             font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    //             font_size: 30.0,
    //             color: Color::GOLD,
    //         }),
    //     ])
    //     .with_style(Style {
    //         position_type: PositionType::Absolute,
    //         position: UiRect {
    //             top: Val::Px(5.0),
    //             left: Val::Px(15.0),
    //             ..default()
    //         },
    //         ..default()
    //     }),
    //     ScoreText,
    // ));
    //     commands.spawn((
    //     // Create a TextBundle that has a Text with a list of sections.
    //         TextBundle::from_sections([TextSection::from_style(TextStyle {
    //             font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    //             font_size: 30.0,
    //             color: Color::GOLD,
    //         })])
    //         .with_style(Style {
    //             position_type: PositionType::Absolute,
    //             position: UiRect {
    //                 top: Val::Px(25.0),
    //                 left: Val::Px(15.0),
    //                 ..default()
    //             },
    //             ..default()
    //         }),
    //         ComboText,
    //     ));
    // }
}

pub fn score_text_update(
    mut query: Query<(&mut Text, &mut Visibility), With<ScoreText>>,
    score: Res<ScoreTracker>,
    gamestate: Res<State<GameState>>,
) {
    let (mut text, mut visibility) = query.single_mut();
    if gamestate.is_changed() {
        *visibility = if gamestate.0 == GameState::InGame {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    text.sections[1].value = score.to_string();
}
pub fn combo_text_update(
    mut query: Query<(&mut Text, &mut Visibility), With<ComboText>>,
    combo: Res<ComboCounter>,
    gamestate: Res<State<GameState>>,
) {
    let (mut text, mut visibility) = query.single_mut();
    if gamestate.is_changed() {
        *visibility = if gamestate.0 == GameState::InGame {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }

    text.sections[0].value = format!("{}x", combo.multiplier());
}

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(hud_setup.in_schedule(OnEnter(GameState::InGame)))
            .add_system(
                Update,
                (combo_text_update, score_text_update).run_if(in_state(GameState::InGame)),
            );
    }
}
