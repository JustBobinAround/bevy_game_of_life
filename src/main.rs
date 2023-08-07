use bevy::prelude::*;
use bevy::window::PresentMode;
use board::*;
use cell_renderer::*;

mod cell_renderer;
mod board;



// #[derive(Component)]
// struct CellPointer<'a>(&'a Cell); // might not need this. The board will probably own the cells 

fn main() {
    App::new()
    .add_startup_system(setup)
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            present_mode: PresentMode::AutoNoVsync, // Reduces input lag.
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }))
    .add_plugin(BoardPlugin)
    .add_plugin(CellRendererPlugin)
    .run();
}

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}
