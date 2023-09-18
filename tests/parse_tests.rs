use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use bomberman_r::board::Board;

fn test_parse(path: &str) {
    let mut file = File::open(path).expect("Could not open file");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Could not read file");
    let content = content.replace("\r\n", "\n");

    let board = Board::from_str(&content).expect("Should be a valid game");

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
