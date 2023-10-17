const BOARD_WIDTH: usize = 50;
const BOARD_HEIGHT: usize = 20;

fn main() {
    let mut game_board: [[char; BOARD_WIDTH]; BOARD_HEIGHT] = init_game_board();
    print_board(&game_board);
    // Start screen:
    // Select speed: 1, 2, 3

    // Game loop

    // End screen:
    // Play again? Yes No
}

fn init_game_board() -> [[char; BOARD_WIDTH]; BOARD_HEIGHT] {
    let mut board: [[char; BOARD_WIDTH]; BOARD_HEIGHT] = [[' '; BOARD_WIDTH]; BOARD_HEIGHT];
    let mut x: usize = 0;
    let mut y: usize = 0;
    while y < board.len() as usize {
        while x < board[y].len() as usize {
            if y == 0 || x == 0 || y+1 == (board.len() as usize) || x+1 == (board[y].len() as usize) {
                board[y][x] = '#';
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }
    board
}

fn show_start_screen() {
    let mut x = 0;
    let mut y = 0;
    loop {

    }
}

fn print_board(board: & [[char; BOARD_WIDTH]; BOARD_HEIGHT]) {
    let mut x: usize = 0;
    let mut y: usize = 0;
    while y < board.len() as usize {
        while x < board[y].len() as usize {
            print!("{}", board[y][x]);
            x += 1;
        }
        y += 1;
        print!("\n");
        x = 0;
    }
}
