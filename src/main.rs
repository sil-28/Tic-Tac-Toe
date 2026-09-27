
#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    X,
    O
}

const WINNING_COMBINATIONS: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

fn print_cell(c: Cell) -> String{
    match c{
        Cell::Empty => " ".to_string(),
        Cell::O => "O".to_string(),
        Cell::X => "X".to_string()
    }
}


fn print_board(board : [Cell; 9]){
    println!("{} | {} | {}", print_cell(board[0]), print_cell(board[1]), print_cell(board[2]));
    println!("--+---+---");
    println!("{} | {} | {}", print_cell(board[3]), print_cell(board[4]), print_cell(board[5]));
    println!("--+---+---");
    println!("{} | {} | {}", print_cell(board[6]), print_cell(board[7]), print_cell(board[8]));
}



fn make_move(board : &mut [Cell; 9], index: usize, value: Cell) -> bool{
    match board[index] {
        Cell::Empty => {board[index] = value; true},
        _ => {false},
    }
}

fn check_win(board : &[Cell; 9]) -> Option<Cell>{
    for combo in WINNING_COMBINATIONS{
        if board[combo[0]] == board[combo[1]] && board[combo[0]] == board[combo[2]]{
            if board[combo[0]] != Cell::Empty {
                return Some(board[combo[0]])
            } 
        }  
    }

    None
}

fn check_tie(board : &[Cell; 9]) -> bool{
    if board.contains(&Cell::Empty){
        return false
    }
    true
}


fn main() {
    let mut board = [Cell::Empty; 9];
    print_board(board);




}
