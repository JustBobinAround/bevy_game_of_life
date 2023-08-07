use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_debug_text_overlay::{screen_print, OverlayPlugin};
use board::*;
use cell_renderer::*;

mod cell_renderer;
mod board;



// #[derive(Component)]
// struct CellPointer<'a>(&'a Cell); // might not need this. The board will probably own the cells 

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            present_mode: PresentMode::AutoNoVsync, // Reduces input lag.
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }))
    .add_plugin(OverlayPlugin { font_size: 12.0, ..default() })
    .add_startup_system(setup)
    .add_plugin(BoardPlugin)
    .add_plugin(CellRendererPlugin)
    .add_system(screen_print_text)
    .run();
}
fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

// Notice how we didn't have to add any special system parameters
fn screen_print_text() {
    screen_print!("\nLeft Click: Spawn a cell
Right Click: Remove a cell
Space: Toggle Pause / Play
Escape: Clear all living cells
Scroll: Move Vertical
LShift + Scroll: Move Horizontal");
}
