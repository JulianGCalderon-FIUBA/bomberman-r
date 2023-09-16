mod scene;

use bomberman_r::game::Game;
use scene::Scene;

#[test]
fn scene1_parse() {
    let scene = Scene::from_file("scenes/scene1.json").expect("Could not load scene");

    let game = Game::try_from(&scene.initial).expect("Should be a valid game");

    assert_eq!(scene.initial, game.to_string());
}

#[test]
fn scene1_explosion() {
    let scene = Scene::from_file("scenes/scene1.json").expect("Could not load scene");

    let mut game = Game::try_from(&scene.initial).expect("Should be a valid game");
    let solution = Game::try_from(&scene.solution).expect("Should be a valid game");

    game.explode(scene.bomb).expect("Tile should exist");

    assert_eq!(game.to_string(), solution.to_string());
}

#[test]
fn scene2_parse() {
    let scene = Scene::from_file("scenes/scene2.json").expect("Could not load scene");

    let game = Game::try_from(&scene.initial).expect("Should be a valid game");

    assert_eq!(scene.initial, game.to_string());
}

#[test]
fn scene2_explosion() {
    let scene = Scene::from_file("scenes/scene2.json").expect("Could not load scene");

    let mut game = Game::try_from(&scene.initial).expect("Should be a valid game");
    let solution = Game::try_from(&scene.solution).expect("Should be a valid game");

    game.explode(scene.bomb).expect("Tile should exist");

    assert_eq!(game.to_string(), solution.to_string());
}

#[test]
fn scene3_parse() {
    let scene = Scene::from_file("scenes/scene3.json").expect("Could not load scene");

    let game = Game::try_from(&scene.initial).expect("Should be a valid game");

    assert_eq!(scene.initial, game.to_string());
}

#[test]
fn scene3_explosion() {
    let scene = Scene::from_file("scenes/scene3.json").expect("Could not load scene");

    let mut game = Game::try_from(&scene.initial).expect("Should be a valid game");
    let solution = Game::try_from(&scene.solution).expect("Should be a valid game");

    game.explode(scene.bomb).expect("Tile should exist");

    assert_eq!(game.to_string(), solution.to_string());
}
