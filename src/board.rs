use bevy::{prelude::*, input::mouse::MouseWheel};
use rayon::iter::{
    IntoParallelRefIterator,
    ParallelIterator
};
use std::{collections::BTreeSet, sync::{Arc, Mutex}};
type RefHashSet = Arc<Mutex<BTreeSet<u64>>>;

macro_rules! lock_as_mut {
    (|$var:ident | $custom_code: block) => {
        let $var = $var.clone();
        if let Ok(mut $var) = $var.lock(){
            $custom_code
        };
    };
}

macro_rules! lock_readonly {
    (|$var:ident | $custom_code: block) => {
        if let Ok($var) = $var.lock(){
            $custom_code
        };
    };
}


pub const BOARD_WIDTH: f32 =  1024.0;
pub const BOARD_HEIGHT: f32 = 512.0;
pub const CELL_SIZE: f32 = 16.0;

#[derive(Resource)]
struct TickTimer(Timer);

#[derive(Resource)]
struct Paused(bool);

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Board::new(Vec2::from((BOARD_WIDTH, BOARD_HEIGHT))))
        .insert_resource(TickTimer(Timer::from_seconds(0.03, TimerMode::Repeating)))
        .insert_resource(Paused(false))
        .add_startup_system(setup)
        .add_system(update_board)
        .add_system(handle_click)
        .add_system(handle_keys);
    }
}

#[derive(Resource)]
pub struct Board { 
    cells: BTreeSet<u64>,
    board_size: Vec2, // may need to be un-public at some point
    num_rows: usize,
    num_columns: usize,
    pause: bool,
    scroll_x: i32,
    scroll_y: i32,
    shift_down: bool,
}

impl Board {

    pub fn new(size: Vec2) -> Self {
        let columns = (size.x / CELL_SIZE) as usize;
        let rows = (size.y / CELL_SIZE) as usize;

        Board {
            cells: BTreeSet::new(),
            board_size: size,
            num_columns: columns,
            num_rows: rows,
            pause: true,
            scroll_x: 10000,
            scroll_y: 10000,
            shift_down: false,
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

    fn perform_cell_actions(&mut self) {
        if !self.pause {
            self.cells = find_living(&mut self.cells);
        }
    }

    pub fn get_color_at_coordinates(&self, column: usize, row: usize) -> Color {
        let column = (column as i32) + self.scroll_x;
        let row = (row as i32) + self.scroll_y;
        let coord = encode_coord(column, row);
        if self.cells.contains(&coord) {
            Color::rgb(0.4, 0.4, 0.4)
        }else{
            Color::rgb(0.0, 0.0, 0.0)
        }
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
    board.perform_cell_actions();
}

fn handle_click (
    board: ResMut<Board>,
    buttons: Res<Input<MouseButton>>,
    window: Query<&Window>
) {
    let window = window.single();

    // This is for large displays
    let x_offset = (window.width() - BOARD_WIDTH)/2.0;
    let y_offset = (window.height() - BOARD_HEIGHT)/2.0;

    if let Some(mut pos) = window.cursor_position() {
        pos.x -= x_offset;
        pos.y -= y_offset;
        if buttons.pressed(MouseButton::Left) {
            spawn_cell_at_pos(pos, board)
        } else if buttons.pressed(MouseButton::Right) {
            kill_cell_at_pos(pos, board)
        }
    }
}

fn handle_keys (
    mut board: ResMut<Board>,
    keys: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    if keys.just_released(KeyCode::LShift){
        board.shift_down = false;
    }
    if keys.just_pressed(KeyCode::LShift) {
        board.shift_down = true;
    }
    if keys.just_pressed(KeyCode::Space) {
        board.pause = !board.pause;
    }
    if keys.just_pressed(KeyCode::Escape) {
        board.cells.clear();
    }

    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                if board.shift_down {
                    board.scroll_x += ev.y as i32;
                } else {
                    board.scroll_y += ev.y as i32;
                }
            }
            MouseScrollUnit::Pixel => {
            }
        }
    }
}

fn calculate_position(
    window_position: Vec2, 
    cols: usize, 
    rows: usize,
    scroll_x: i32,
    scroll_y: i32
) -> u64 {
    let pos = window_position;
    let x_pos = pos.x.floor();
    let y_pos = pos.y.floor();

    
    let cols = cols as f32;
    let rows = rows as f32;

    let x_pos = ((x_pos / CELL_SIZE % cols) as i32) + scroll_x;
    let y_pos = ((y_pos / CELL_SIZE % rows) as i32) + scroll_y;
    
    encode_coord(x_pos,y_pos)
}

fn spawn_cell_at_pos(
    pos: Vec2,
    mut board: ResMut<Board>
) {
    let cols = board.num_columns;
    let rows = board.num_rows;

    let scroll_x = board.scroll_x;
    let scroll_y = board.scroll_y;

    board.cells.insert(calculate_position(
            pos, cols, rows, scroll_x, scroll_y));
}

fn kill_cell_at_pos(
    pos: Vec2,
    mut board: ResMut<Board>
){
    let cols = board.num_columns;
    let rows = board.num_rows;

    let scroll_x = board.scroll_x;
    let scroll_y = board.scroll_y;

    board.cells.remove(&calculate_position(
            pos, cols, rows, scroll_x, scroll_y));
}

fn find_living(cells: &mut BTreeSet<u64>) -> BTreeSet<u64> {
    let new_cells: RefHashSet = Arc::new(Mutex::new(BTreeSet::new()));
    let possible_newborns: RefHashSet = Arc::new(Mutex::new(BTreeSet::new()));

    cells.par_iter().for_each(|coord|{
        let (x, y) = decode_coord(*coord);
        let count = count_neighbors_and_newborns(
            &x, &y, cells, possible_newborns.clone());
        if count >= 2 && count <= 3 {
            lock_as_mut!(|new_cells|{
                let coord = encode_coord(x, y);
                new_cells.insert(coord);
            });
        }
    });
    lock_readonly!(|possible_newborns|{
        possible_newborns.par_iter().for_each(|coord|{
            let (x, y) = decode_coord(*coord);
            let count = count_neighbors(&x, &y, cells);
            if count == 3 {
                lock_as_mut!(|new_cells|{
                    let coord = encode_coord(x, y);
                    new_cells.insert(coord);
                });
            }
        });
    });

    // This is how you can exit a mutex without cloning...
    // I don't really know how stable this is, but it 
    // seems to work fine.
    let final_cells: BTreeSet<u64>;
    if let Ok(mut new_cells) = new_cells.lock() {
        final_cells = std::mem::take(&mut *new_cells);
    } else {
        unreachable!("final_cells should always consume new_cells mutex lock");
    };
    final_cells
}

fn encode_coord(x: i32, y: i32) -> u64 {
    let upper_u32: u64 = (x as u64) << 32;
    let lower_u32: u64 = y as u64;
    upper_u32 | lower_u32
}
fn decode_coord(coord: u64) -> (i32, i32) {
    let x = (coord >> 32) as i32;
    let y = coord as i32;
    (x, y)
}

fn count_neighbors(
    x: &i32, 
    y: &i32,
    cells: &BTreeSet<u64>,
) -> usize {

    let x = *x;
    let y = *y;
    let mut count = 0;
    let init_coord = encode_coord(x, y);

    if !cells.contains(&init_coord) {
        for i in -1..=1 {
            for j in -1..=1 {
                let coord = encode_coord(x+i, y+j);
                if cells.contains(&coord) && coord != init_coord {
                    count += 1;
                }
            }
        }
    }


    count 
}

fn count_neighbors_and_newborns(
    x: &i32, 
    y: &i32,
    cells: &BTreeSet<u64>,
    possible_newborns: RefHashSet
) -> usize {

    let x = *x;
    let y = *y;
    let mut count: isize = -1;

    for i in -1..=1 {
        for j in -1..=1 {
            let coord = encode_coord(x+i, y+j);
            if cells.contains(&coord) {
                count += 1;
            } else {
                lock_as_mut!(|possible_newborns|{
                    possible_newborns.insert(coord);
                });
            }
        }
    }

    assert!(count >= 0, "Count should always be greater than zero because it counts the cell that
            called this function");

    count as usize
}

fn setup (
    mut commands: Commands,
    board: Res<Board>
) {
    commands.spawn(board.get_board_rect());
}
