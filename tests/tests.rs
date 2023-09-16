mod scene;

use bomberman_r::board::Board;
use scene::Scene;

#[test]
fn scene1_parse() {
    let scene = Scene::from_file("scenes/scene1.json").expect("Could not load scene");

    let board = Board::from(&scene.initial);

    assert_eq!(scene.initial, board.to_string());
}

#[test]
fn scene1_explosion() {
    let scene = Scene::from_file("scenes/scene1.json").expect("Could not load scene");

    let mut board = Board::from(&scene.initial);
    let solution = Board::from(&scene.solution);

    board.explode(scene.bomb.0, scene.bomb.1);

    assert_eq!(board.to_string(), solution.to_string());
}

#[test]
fn scene2_explosion() {
    let scene = Scene::from_file("scenes/scene2.json").expect("Could not load scene");

    let mut board = Board::from(&scene.initial);
    let solution = Board::from(&scene.solution);

    board.explode(scene.bomb.0, scene.bomb.1);

    assert_eq!(board.to_string(), solution.to_string());
}

#[test]
fn scene3_explosion() {
    let scene = Scene::from_file("scenes/scene3.json").expect("Could not load scene");

    let mut board = Board::from(&scene.initial);
    let solution = Board::from(&scene.solution);

    board.explode(scene.bomb.0, scene.bomb.1);

    assert_eq!(board.to_string(), solution.to_string());
}
