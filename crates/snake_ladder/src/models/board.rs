use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Board {
    pub size: usize,
    pub cells: Vec<Cells>,
    pub player_positions: HashMap<String, usize>,
}

#[derive(Debug, Clone)]
pub enum BoardError {
    InvalidPosition,
    PlayerNotFound,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let cells = vec![Cells::Empty; size];
        let player_positions = HashMap::new();
        Board {
            size,
            cells,
            player_positions,
        }
    }

    pub fn add_snake(&mut self, start: usize, end: usize) {
        self.cells[start] = Cells::Snake((start, end));
    }

    pub fn add_ladder(&mut self, start: usize, end: usize) {
        self.cells[start] = Cells::Ladder((start, end));
    }

    pub fn add_player(&mut self, player_name: String) {
        if !self.player_positions.contains_key(&player_name) {
            self.player_positions.insert(player_name, 0);
        }
    }

    pub fn move_player(&mut self, player_name: &str, steps: usize) -> Result<usize, BoardError> {
        if let Some(position) = self.player_positions.get_mut(player_name) {
            let new_position = *position + steps;
            if new_position >= self.size {
                return Ok(self.size - 1); // Player cannot move beyond the board size
            }
            *position = new_position;

            match self.cells[new_position] {
                Cells::Snake((_, end)) => {
                    println!(
                        "Player {} encountered a snake! Moving to {}",
                        player_name, end
                    );
                    *position = end;
                }
                Cells::Ladder((_, end)) => {
                    println!(
                        "Player {} encountered a ladder! Moving to {}",
                        player_name, end
                    );
                    *position = end;
                }
                Cells::Empty => {
                    println!("Player {} moved to {}", player_name, new_position);
                }
            }
            Ok(new_position)
        } else {
            Err(BoardError::PlayerNotFound)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Cells {
    Empty,
    Snake((usize, usize)),
    Ladder((usize, usize)),
}
