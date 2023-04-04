use bevy::prelude::*;
use crate::cell::*;

pub const BOARD_WIDTH: f32 =  1024.0;
pub const BOARD_HEIGHT: f32 = 512.0;
pub const CELL_SIZE: f32 = 16.0;

#[derive(Resource)]
struct TickTimer(Timer);
// if cell never gets used in main's context, it may be worth declaring mod cell here maybe?

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Board::new(Vec2::from((BOARD_WIDTH, BOARD_HEIGHT))))
        .insert_resource(TickTimer(Timer::from_seconds(0.05, TimerMode::Repeating)))
        .add_startup_system(setup)
        .add_system(update_board)
        .add_system(handle_click);
    }
}

#[derive(Resource)]
pub struct Board { 
    grid: Vec<Vec<Cell>>,
    board_size: Vec2, // may need to be un-public at some point
    num_rows: usize,
    num_columns: usize
}

impl Board {

    pub fn new(size: Vec2) -> Self {
        let columns = (size.x / CELL_SIZE) as usize;
        let rows = (size.y / CELL_SIZE) as usize;

        Board {
            grid: Board::create_default_grid(columns, rows),
            board_size: size,
            num_columns: columns,
            num_rows: rows,
        }
    }

    pub fn get_board_rect(&self) -> SpriteBundle {
        return SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.1, 0.1),
                custom_size: Some(Vec2::new(self.board_size.x, self.board_size.y)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        }
    }

    fn create_default_grid(columns: usize, rows: usize) -> Vec<Vec<Cell>> {
        let mut grid: Vec<Vec<Cell>>  = Vec::new(); // Theres gotta be a more functional way of populating this
        for _i in 0..columns { 
            let mut column: Vec<Cell> = Vec::new();
            for j in 0..rows {
                if j % 2 == 0 {
                    column.push(Cell { cell_type: CellType::Empty });
                } else {
                    column.push(Cell { cell_type: CellType::Sand })
                }
            }
            grid.push(column);
        }

        return grid;
    }

    fn perform_cell_actions(&mut self) {
        for column in 0.. self.num_columns { 
            for row in 0..self.num_rows {
                let cell_to_move;
                if let CellType::Empty = self.grid[column][row].cell_type {
                    continue;
                } else {
                    cell_to_move = self.grid[column][row]
                }
                if let Some((dest_column, dest_row)) = self.check_for_empty_adjacent_space(cell_to_move, column, row) {
                    self.grid[dest_column][dest_row].cell_type = self.grid[column][row].cell_type;
                    self.grid[column][row].cell_type = CellType::Empty;
                        
                }
            }
        }
    }

    fn check_for_empty_adjacent_space(&self, cell: Cell, column:usize, row:usize) -> Option<(usize, usize)> {
        for direction_preference_category in cell.get_move_direction_preference() {
            for direction_preference in direction_preference_category {
                let column = column as i32;
                let row = row as i32;
                let cell_to_check;
                let column_modifier; 
                let row_modifier;
    
                match direction_preference {
                    MoveDirection::Up => { (column_modifier, row_modifier) = (column, row + 1)},
                    MoveDirection::UpRight => { (column_modifier, row_modifier) = (column + 1, row + 1) },
                    MoveDirection::Right => { (column_modifier, row_modifier) = (column + 1, row) },
                    MoveDirection::DownRight => { (column_modifier, row_modifier) = (column + 1, row - 1) },
                    MoveDirection::Down => { (column_modifier, row_modifier) = (column, row - 1) },
                    MoveDirection::DownLeft => { (column_modifier, row_modifier) = (column - 1, row - 1) },
                    MoveDirection::Left => { (column_modifier, row_modifier) = (column - 1, row) },
                    MoveDirection::UpLeft => { (column_modifier, row_modifier) = (column - 1, row + 1) }
                }
    
                if column_modifier < 0 || row_modifier < 0 { 
                    continue;
                }
    
                let column_modifier = column_modifier as usize;
                let row_modifier = row_modifier as usize;
    
                cell_to_check = self.legal_cell_at(column_modifier, row_modifier);
                if let Some(cell) = cell_to_check {
                    if let CellType::Empty = cell.cell_type {
                        return Some((column_modifier, row_modifier));
                    }
                }
            }
        }
        return None;
    }

    pub fn legal_cell_at(&self, column: usize, row: usize) -> Option<&Cell> {
        let column_vec = self.grid.get(column);
        if let Some(vec) = column_vec {
            vec.get(row)
        } else {
            None
        }
    }

    fn print_board (&self) {
        for column in 0..self.num_columns {
            for row in 0..self.num_rows {
                let mut type_to_print;
                match self.grid[column][row].cell_type {
                    CellType::Empty => type_to_print = "E",
                    CellType::Sand => type_to_print = "S",
                    CellType::Water => type_to_print = "W"
                }

                print!("{type_to_print}    ");
            }
            print!("\n");
        }
    }

    fn get_color_at_coordinates(&self, column: usize, row: usize) -> Color {
        self.grid[column][row].get_color()
    }



}

fn update_board(
    mut board: ResMut<Board>,
    mut timer: ResMut<TickTimer>,
    time: Res<Time>
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    println!("BEFORE ACTIONS");
    board.print_board();

    board.perform_cell_actions();

    println!("AFTER ACTIONS");
    board.print_board();
}

fn handle_click (
    mut board: ResMut<Board>,
    buttons: Res<Input<MouseButton>>,
    window: Query<&Window>
) {
    if buttons.pressed(MouseButton::Left) {
        let window = window.single();

        if let Some(position) = window.cursor_position() {
            let column = board.num_columns / 2;
            let row = board.num_rows - 1;
            board.grid[column][row].cell_type = CellType::Sand;
        } else {
            // cursor is not inside the window
        }
    }
}

fn setup (
    mut commands: Commands,
    board: Res<Board>
) {
    commands.spawn(board.get_board_rect());
}