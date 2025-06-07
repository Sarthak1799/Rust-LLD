use snake_ladder::game_service::{GameService, GameServiceGenerator};

#[tokio::main]
async fn main() {
    let mut generator = GameServiceGenerator::new();
    let game_service1 = GameService::new(100, "1".to_string());
    let game_service2 = GameService::new(100, "2".to_string());

    generator.add_game(game_service1);
    generator.add_game(game_service2);

    generator
        .run_all_games()
        .await
        .expect("main: failed to run all games");
}
