use bevy::prelude::*;
use rand::{thread_rng, seq::SliceRandom};

pub enum MoveDirection {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft
}

#[derive(Copy, Clone)]
pub enum CellType {
    Sand,
    Water,
    Empty
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub cell_type: CellType
    // component_ptr: may not be needed?

}

impl Cell {
    pub fn get_color(&self) -> Color {
        match self.cell_type {
            CellType::Sand => Color::rgb(0.35, 0.32, 0.25),
            CellType::Water => Color::rgb(0.0, 0.5, 0.90), // may need adjustment
            CellType::Empty => Color::rgba(0.0, 0.0, 0.0, 0.0)
        }
    }

    pub fn get_move_direction_preference(&self) -> Vec<Vec<MoveDirection>>{
        match self.cell_type {
            CellType::Sand => {
                let primary = vec![MoveDirection::Down];
                let mut secondary = vec![MoveDirection::DownLeft, MoveDirection::DownRight];
                secondary.shuffle(&mut thread_rng());
                vec![primary, secondary]
            },
            CellType::Water => {
                let primary = vec![MoveDirection::Down];
                let mut secondary = vec![MoveDirection::DownLeft, MoveDirection::DownRight];
                let mut tertiary = vec![MoveDirection::Left, MoveDirection::Right];
                secondary.shuffle(&mut thread_rng());
                tertiary.shuffle(&mut thread_rng());

                vec![primary, secondary, tertiary]
            },
            CellType::Empty => vec![vec![]]
        }
    }
}