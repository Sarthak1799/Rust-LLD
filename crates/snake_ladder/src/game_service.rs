use crate::models::{board::Board, dice::Dice, player::Player};

#[derive(Debug, Clone)]
pub struct GameService {
    pub players: Vec<Player>,
    pub board: Board,
    pub game_id: String,
}

#[derive(Debug, Clone)]
pub enum GameServiceError {
    GameServiceError(String),
}

impl GameService {
    pub fn new(size: usize, id: String) -> Self {
        let board = Board::new(size);
        GameService {
            players: Vec::new(),
            board,
            game_id: id,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.board.add_player(player.get_id().to_string());
        self.players.push(player);
    }

    pub fn add_snake(&mut self, start: usize, end: usize) {
        self.board.add_snake(start, end);
    }
    pub fn add_ladder(&mut self, start: usize, end: usize) {
        self.board.add_ladder(start, end);
    }

    pub fn start_game(&mut self) -> Result<(), GameServiceError> {
        let dice = Dice::new(6);
        let mut game_status = false;
        while game_status == false {
            for player in &self.players {
                let roll = dice.roll();
                let position = self
                    .board
                    .move_player(player.get_id(), roll)
                    .map_err(|err| {
                        GameServiceError::GameServiceError(format!(
                            "game id - {}, Failed to move player {}: {:?}",
                            self.game_id,
                            player.get_name(),
                            err
                        ))
                    })?;

                if position >= (self.board.size - 1) {
                    println!(
                        "game id - {}, Player {} wins!",
                        self.game_id,
                        player.get_name()
                    );
                    game_status = true;
                    return Ok(());
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct GameServiceGenerator {
    pub games: Vec<GameService>,
}

impl GameServiceGenerator {
    pub fn new() -> Self {
        GameServiceGenerator { games: Vec::new() }
    }

    pub fn add_game(&mut self, game: GameService) {
        self.games.push(game);
    }

    pub async fn run_all_games(self) -> Result<(), GameServiceError> {
        let mut handles = Vec::new();
        for game in self.games {
            let handle = tokio::spawn(async move {
                println!("Starting game with ID: {}", game.game_id);
                generate_game(game.clone()).await;
            });
            handles.push(handle);
        }

        futures::future::try_join_all(handles).await.map_err(|_| {
            GameServiceError::GameServiceError("Failed to run all games".to_string())
        })?;
        Ok(())
    }
}

pub async fn generate_game(game_service: GameService) {
    let mut game_service = game_service;
    game_service.add_snake(16, 6);
    game_service.add_snake(47, 26);
    game_service.add_snake(49, 11);
    game_service.add_ladder(23, 45);
    game_service.add_ladder(34, 56);
    game_service.add_ladder(51, 67);
    let player1 = Player::new("Alice".to_string(), "1".to_string());
    let player2 = Player::new("Bob".to_string(), "2".to_string());

    game_service.add_player(player1);
    game_service.add_player(player2);

    match game_service.start_game() {
        Ok(_) => println!("Game finished successfully!"),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
