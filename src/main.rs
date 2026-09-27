use std::io;



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
    println!();
    println!("The grid is indexed from 1 to 9.");
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

    println!("Name of the first player");
    let mut player1_name = String::new();
    io::stdin().read_line(&mut player1_name).expect("Failed to read line");
    let player1_name = player1_name.trim();

    println!("Name of the second player");
    let mut player2_name = String::new();
    io::stdin().read_line(&mut player2_name).expect("Failed to read line");
    let player2_name = player2_name.trim();

    println!("{} will be X. {} will be O. Good luck!", player1_name, player2_name);
    let mut play = String::new();
    print_board(board);

    let mut found_winner = false;
    let mut found_tie = false;

    loop{
        loop{
            play.clear();
            println!("{}, where do you want to play?", player1_name);
            io::stdin().read_line(&mut play).expect("Failed to read line");
            match play.trim().parse::<usize>(){
                Ok(i) => {if (1..=9).contains(&i) {
                                    let check_move = make_move(&mut board, i-1, Cell::X);
                                    if !check_move {println!("The cell is already occupied. Try again")}
                                    else {
                                        print_board(board);
                                        match check_win(&board) {
                                            Some(_) => {println!("{} has won!", player1_name);
                                                        found_winner = true;
                                                        break;},
                                            None => ()
                                        }
                                        match check_tie(&board) {
                                            true => {println!("It's a tie");
                                                    found_tie = true;
                                                    break;},
                                            false => ()
                                        }
                                        break;
                                    }
                                    

                                } else {println!("Enter valid index")}
                            },
                Err(_) => println!("Please enter a valid number")
            }
        }

        if found_tie | found_winner {break;}

        loop{
            play.clear();
            println!("{}, where do you want to play?", player2_name);
            io::stdin().read_line(&mut play).expect("Failed to read line");
            match play.trim().parse::<usize>(){
                Ok(i) => {if (1..=9).contains(&i) {
                                    let check_move = make_move(&mut board, i-1, Cell::O);
                                    if !check_move {println!("The cell is already occupied. Try again")}
                                    else {
                                        print_board(board);
                                        match check_win(&board) {
                                            Some(_) => {println!("{} has won!", player2_name);
                                                        found_winner = true;
                                                        break;},
                                            None => ()
                                        }
                                        match check_tie(&board) {
                                            true => {println!("It's a tie");
                                                    found_tie = true;
                                                        break;},
                                            false => ()
                                        }
                                        break;
                                    }
                                    

                                } else {println!("Enter valid index")}
                            },
                Err(_) => println!("Please enter a valid number")
            }
        }
        
        if found_tie | found_winner {break;}
        

    }



}
