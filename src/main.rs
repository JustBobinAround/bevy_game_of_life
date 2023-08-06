use bevy::prelude::*;
use board::*;
use cell_renderer::*;

mod cell_renderer;
mod board;



// #[derive(Component)]
// struct CellPointer<'a>(&'a Cell); // might not need this. The board will probably own the cells 

fn main() {
    App::new()
    .add_startup_system(setup)
    .add_plugin(BoardPlugin)
    .add_plugins(DefaultPlugins)
    .add_plugin(CellRendererPlugin)
    .run();
}

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}
