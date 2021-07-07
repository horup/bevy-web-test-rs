use bevy::{app::Events, prelude::*, window::WindowResized};
use bevy_kira_audio::Audio;

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Spawning UI");
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    // Text with one section
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        // Use the `Text::with_section` constructor
        text: Text::with_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "hello\nbevy!",
            TextStyle {
                font: asset_server.load("default.ttf"),
                font_size: 16.0,
                color: Color::WHITE,
            },
            // Note: You can use `Default::default()` in place of the `TextAlignment`
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        ),
        ..Default::default()
    });
}

pub fn spawn_sprites(mut commands: Commands, asset_server: Res<AssetServer>, mut materials:ResMut<Assets<ColorMaterial>>) {
    info!("spawning sprites");
    let explosion = asset_server.load("explosion.png");
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(explosion.into()),
        ..Default::default()
    });
}

pub fn spawn_camera(mut commands: Commands) {
    info!("spawning camera");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn play_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("test.ogg"));
}
