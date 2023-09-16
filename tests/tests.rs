mod scene;

use bomberman_r::board::Board;
use scene::Scene;

#[test]
fn scene1_parse() {
    let scene = Scene::from_file("scenes/scene1.json").expect("Could not load scene");

    let board = Board::try_from(&scene.initial).expect("Should be a valid board");

    assert_eq!(scene.initial, board.to_string());
}

#[test]
fn scene1_explosion() {
    let scene = Scene::from_file("scenes/scene1.json").expect("Could not load scene");

    let mut board = Board::try_from(&scene.initial).expect("Should be a valid board");
    let solution = Board::try_from(&scene.solution).expect("Should be a valid board");

    board.explode(scene.bomb).expect("Tile should exist");

    assert_eq!(board.to_string(), solution.to_string());
}

#[test]
fn scene2_parse() {
    let scene = Scene::from_file("scenes/scene2.json").expect("Could not load scene");

    let board = Board::try_from(&scene.initial).expect("Should be a valid board");

    assert_eq!(scene.initial, board.to_string());
}

#[test]
fn scene2_explosion() {
    let scene = Scene::from_file("scenes/scene2.json").expect("Could not load scene");

    let mut board = Board::try_from(&scene.initial).expect("Should be a valid board");
    let solution = Board::try_from(&scene.solution).expect("Should be a valid board");

    board.explode(scene.bomb).expect("Tile should exist");

    assert_eq!(board.to_string(), solution.to_string());
}

#[test]
fn scene3_parse() {
    let scene = Scene::from_file("scenes/scene3.json").expect("Could not load scene");

    let board = Board::try_from(&scene.initial).expect("Should be a valid board");

    assert_eq!(scene.initial, board.to_string());
}

#[test]
fn scene3_explosion() {
    let scene = Scene::from_file("scenes/scene3.json").expect("Could not load scene");

    let mut board = Board::try_from(&scene.initial).expect("Should be a valid board");
    let solution = Board::try_from(&scene.solution).expect("Should be a valid board");

    board.explode(scene.bomb).expect("Tile should exist");

    assert_eq!(board.to_string(), solution.to_string());
}
