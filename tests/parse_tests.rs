use std::path::Path;

use bomberman_r::board::Board;
use bomberman_r::io::read_from_file;

fn test_parse<P: AsRef<Path>>(path: P) {
    let content = read_from_file(path).expect("Couldn't read file");
    let board: Board = content.parse().expect("Should be a valid game");

    assert_eq!(board.to_string(), content);
}

#[test]
fn parse1() {
    test_parse("boards/board1.txt");
}

#[test]
fn parse2() {
    test_parse("boards/board2.txt");
}

#[test]
fn parse3() {
    test_parse("boards/board3.txt");
}
