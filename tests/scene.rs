use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use bomberman_r::position::Position;
use serde::Deserialize;

#[derive(Deserialize)]
struct InnerScene {
    initial: Vec<String>,
    solution: Vec<String>,
    bomb: (usize, usize),
}

#[derive(Debug)]
pub struct Scene {
    pub initial: String,
    pub solution: String,
    pub bomb: Position,
}

impl Scene {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let inner_scene: InnerScene = serde_json::from_reader(reader)?;

        let scene = Scene {
            initial: inner_scene.initial.join("\n"),
            solution: inner_scene.solution.join("\n"),
            bomb: Position {
                x: inner_scene.bomb.0,
                y: inner_scene.bomb.1,
            },
        };

        Ok(scene)
    }
}
