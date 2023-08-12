use crate::board::*;
use bevy::prelude::*;

#[derive(Component)]
struct RenderedCell;

#[derive(Component)]
struct CellPosition {
    column: usize,
    row: usize,
}

pub struct CellRendererPlugin;
impl Plugin for CellRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_cells)
            .add_system(update_cells);
    }
}

fn initialize_cells(mut commands: Commands) {
    let num_columns = (BOARD_WIDTH / CELL_SIZE) as usize;
    let num_rows = (BOARD_HEIGHT / CELL_SIZE) as usize;

    let start_pos = Vec3::from((
        (CELL_SIZE / 2.0) - (BOARD_WIDTH / 2.0),
        (CELL_SIZE / 2.0) - (BOARD_HEIGHT / 2.0),
        1.0,
    ));
    for column in 0..num_columns {
        for row in 0..num_rows {
            commands.spawn((
                RenderedCell,
                CellPosition { column, row },
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.4, 0.0, 0.0, 1.0),
                        custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_translation(
                        start_pos
                            + Vec3::from((column as f32 * CELL_SIZE, row as f32 * CELL_SIZE, 0.0)),
                    ),
                    ..default()
                },
            ));
        }
    }
}

fn update_cells(
    mut all_rendered_cells: Query<(&CellPosition, &mut Sprite), With<RenderedCell>>,
    board: Res<Board>,
) {
    for (cell_pos, mut sprite) in all_rendered_cells.iter_mut() {
        sprite.color = board.get_color_at_coordinates(cell_pos.column, cell_pos.row);
    }
}
