use connect_four::*;

fn main() {
    let board = Board::new();
    println!("{}", full_search(&board));
}
