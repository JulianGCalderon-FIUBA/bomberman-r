use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

use bomberman_r::board::Board;
use bomberman_r::error::MyResult;
use bomberman_r::game::Game;
use configuration::Arguments;

pub mod configuration;

pub fn main() {
    let arguments = match Arguments::collect() {
        Ok(arguments) => arguments,
        Err(error) => {
            eprintln!("ERROR: {}", error);
            return;
        }
    };

    let output_path = arguments.output_path.clone();

    if let Err(error) = try_main(arguments) {
        if let Err(error) = write_to_file(&output_path, &error.to_string()) {
            eprintln!("ERROR: {}", error);
        }
    }
}

fn try_main(arguments: Arguments) -> MyResult<()> {
    let mut file = File::open(arguments.board_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let board = Board::from_str(&content)?;
    let mut game = Game::with_board(board);

    game.trigger_bomb(arguments.bomb_position)?;

    let output = game.board().to_string();
    write_to_file(&arguments.output_path, &output)?;

    Ok(())
}

fn write_to_file(path: &Path, content: &str) -> MyResult<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}
