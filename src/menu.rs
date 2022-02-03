use crate::AppState;
use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.12, 0.14, 0.19);
const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.65, 0.8, 0.44);
const ACTIVE_BUTTON_COLOR: Color = Color::rgb(0.98, 0.82, 0.48);

#[derive(Debug)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(create_menu))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu));
    }
}

fn create_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Style::default()
            },
            color: Color::WHITE.into(),
            ..ButtonBundle::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Play",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                        font_size: 20.0,
                        color: Color::BLACK,
                    },
                    TextAlignment::default(),
                ),
                ..TextBundle::default()
            });
        });
}

#[allow(clippy::type_complexity)]
fn menu(
    mut state: ResMut<State<AppState>>,
    mut interaction: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in interaction.iter_mut() {
        *color = if matches!(interaction, Interaction::None) {
            NORMAL_BUTTON_COLOR
        } else {
            ACTIVE_BUTTON_COLOR
        }
        .into();

        if matches!(interaction, Interaction::Clicked) {
            state.set(AppState::Game).unwrap();
        }
    }
}
