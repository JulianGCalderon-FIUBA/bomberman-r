use std::path::PathBuf;

use bomberman_r::error::{Error, MyResult};
use bomberman_r::position::Position;

pub struct Arguments {
    pub board_path: PathBuf,
    pub output_path: PathBuf,
    pub bomb_position: Position,
}

impl Arguments {
    pub fn collect() -> MyResult<Self> {
        let arguments: Vec<String> = std::env::args().collect();

        if arguments.len() != 5 {
            return Err(Error::InvalidArguments);
        }

        let board_path = PathBuf::from(&arguments[1]);
        let output_dir = PathBuf::from(&arguments[2]);

        let board_path_name = board_path.file_name().ok_or(Error::InvalidArguments)?;
        let output_path = output_dir.join(board_path_name);

        let bomb_x = arguments[3]
            .parse::<usize>()
            .map_err(|_| Error::InvalidArguments)?;

        let bomb_y = arguments[4]
            .parse::<usize>()
            .map_err(|_| Error::InvalidArguments)?;

        let bomb_position = Position::new(bomb_x, bomb_y);

        Ok(Self {
            board_path,
            output_path,
            bomb_position,
        })
    }
}
