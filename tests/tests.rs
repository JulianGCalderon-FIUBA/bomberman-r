mod scene;

use bomberman_r::board::Board;
use scene::Scene;

#[test]
fn scene1_parse() {
    let scene = Scene::from_file("scenes/scene1.json").expect("Could not load scene");

    let board = Board::from(&scene.initial);

    let stringed_board = board.to_string();

    assert_eq!(scene.initial, stringed_board);
}
