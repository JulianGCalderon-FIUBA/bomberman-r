mod scene;

use bomberman_r::board::Board;
use bomberman_r::game::Game;
use scene::Scene;

fn test_scene(path: &str) {
    let scene = Scene::from_file(path).expect("Could not load scene");

    let board = Board::try_from(&scene.initial).expect("Should be a valid game");
    let solution = Board::try_from(&scene.solution).expect("Should be a valid game");

    let mut game = Game::with_board(board);
    game.explode(scene.bomb).expect("Tile should exist");

    assert_eq!(game.board().to_string(), solution.to_string());
}

#[test]
fn scene1() {
    test_scene("scenes/scene1.json");
}

#[test]
fn scene2() {
    test_scene("scenes/scene2.json");
}

#[test]
fn scene3() {
    test_scene("scenes/scene3.json");
}

#[test]
fn scene4() {
    test_scene("scenes/scene4.json");
}
