use arguments::Arguments;
use bomberman_r::board::Board;
use bomberman_r::error::MyResult;
use bomberman_r::game::Game;
use bomberman_r::io::write_to_file;

mod arguments;

pub fn main() {
    let arguments = match Arguments::collect() {
        Ok(arguments) => arguments,
        Err(error) => {
            eprintln!("Invalid Arguments: {}", error);
            return;
        }
    };

    let output_path = arguments.output_path.clone();

    if let Err(error) = try_main(arguments) {
        let error_message = format!("ERROR: {}", error);
        if let Err(error) = write_to_file(&output_path, &error_message) {
            eprintln!("Could not write error to file: {}", error);
            eprintln!("{}", error_message);
        }
    }
}

fn try_main(arguments: Arguments) -> MyResult<()> {
    let board = Board::from_file(&arguments.board_path)?;
    let mut game = Game::with_board(board);

    game.trigger_bomb(arguments.bomb_position)?;

    game.board().to_file(&arguments.output_path)
}
