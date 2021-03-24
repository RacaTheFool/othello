/***********************************************************
 * MY OTHELLO GAME
 ***********************************************************
 * This is an Othello game program. Othello is based on
 * Reversi and a full explaination of the rules can be
 * found here: https://en.wikipedia.org/wiki/Reversi
 * 
 * Basic explanation of the rules:
 * The game is played on an 8*8 board with black and white
 * game pieces (represented in this program as "X" and "O"
 * resectively) that can be flipped from one color to
 * the other. Two of each color is placed in the middle at
 * diagonals with black ("X") traditionally at the northeast
 * and southwest corners. Pieces are flipped when they are
 * captured by placing one of your pieces on a horizontal,
 * veritical, and/or diagonal line from one of your own
 * pieces with only the oppenents pieces between the two.
 * Black ("X") goes first. Each player must place to
 * capture at least one of the other players pieces
 * each turn, and their turn is skipped if no location
 * allows them to do so. The game ends once neither
 * player can place down, even if the game board is
 * not full. The player with the most pieces on the
 * board wins.
***********************************************************/

use text_io::read;      // For the read!() function to accept user input
static _X: &str = "X";  // Used to represent player one's pieces
static _O: &str = "O";  // Used to represent player two's pieces
const SIZE: usize = 4;  // The size of the game board

/***********************************************************
 * NUMLET
 ***********************************************************
 * This is a structure used to store the number and letter
 * inputs for the coordinates on the game board. Used by
 * the Vector in the check() function.
***********************************************************/
struct NumLet {
    n: usize,   // Number index
    l: usize    // Letter index
}

/***********************************************************
 * MAIN
 ***********************************************************
 * This function is used to create the game board and start
 * the game. Once the game is finished it will prompt the
 * user if they want to play again.
***********************************************************/
fn main() {
    // The game board; " " is used as the value for empty
    // spaces so as to make displaying the board simpler.
    let mut board = [[" "; SIZE]; SIZE];
    
    create_board(&mut board);   // Used to place the beginning pieces on the board
    let mut play = true;        // True if the user wants to play the game

    // While the user wants to keep playing games keep
    // running the game
    while play {
        othello(board); // Play Othello

        // Used to determine if the user inputs an appropriate
        // response, either a "y"/"Y" or "n"/"N"
        let mut good_in = false;

        // until the user inputs an appropriate response
        // ask them if they would like to play again
        while !good_in {
            println!("Do you want to play again? (y/n)");
            let option: String = read!(); // User input
            if option.to_lowercase() == "n"  { // User doesn't want to play again
                play = false;   // Exit the program
                good_in = true; // User input was good
            }
            else if option.to_lowercase() == "y" { // User wants to play again
                good_in = true; // User input was good
            }
        }
    }
}

/***********************************************************
 * OTHELLO
 ***********************************************************
 * Play the game of Othello. Starts by diplaying the game
 * board and then prompting player one ("X") to take their
 * turn. Turns alternate unless a players turn is skipped,
 * and the game ends once neither player can play. It then
 * displays the winner (or that it's a tie) and the player's
 * scores and returns back the the main() function.
***********************************************************/
fn othello(mut board: [[&str; SIZE]; SIZE]) {
    let mut playing = true;         // True if still playing
    let mut was_skipped = false;    // True if the last player's turn was skipped
    let mut is_player_one = true;   // True if the current player is player one ("X")
    
    // While the game isn't over keep playing
    while playing {
        if !was_skipped {   // Only display the board once if the last player was skipped
            display(board); // Display the game board
        }

        // Create a copy of the game board so that you can test if the current
        // player has any locations they can place.
        let mut test_turn_board = board;

        // If the current player can take their turn then they do so
        if can_take_turn(&mut test_turn_board, is_player_one) {
            if was_skipped { // If the last player was skipped inform the users
                println!("Next Player's turn was Skipped!\n\n");
                was_skipped = false; // The current player can take their turn, so reset to false
            }
            if is_player_one { // Inform the users who's turn it is
                println!("Player X's turn.");
            }
            else {
                println!("Player O's turn.");
            }
            take_turn(&mut board, is_player_one); // Current player takes their turn
        }
        else { // If the current player can't take their turn then...
            if was_skipped { // If the last player was skipped than neither can play (game is over)
                let mut x_count = 0; // Counts "X" pieces
                let mut o_count = 0; // Counts "O" pieces

                // Iterate through each row, and each column in each row
                for (i1, _row) in board.iter().enumerate() {
                    for (i2, _column) in _row.iter().enumerate() {
                        if board[i1][i2] == _X {        // If location is equivalent to "X"...
                            x_count = x_count + 1;      // Increment x_count
                        }
                        else if board[i1][i2] == _O {   // If location is equivalent to "O"...
                            o_count = o_count + 1;      // Increment o_count
                        }
                    }
                }

                // Determine who won, or if there is a tie
                if x_count > o_count { // "X" wins!
                    println!("--------------");
                    println!("Player X wins!");
                    println!("--------------\n");
                    println!("Player X score: {}", x_count); // Display "X" count
                    println!("Player O score: {}", o_count); // Display "O" count
                }
                else if x_count < o_count { // "O" wins!
                    println!("--------------");
                    println!("Player O wins!");
                    println!("--------------\n");
                    println!("Player O score: {}", o_count); // Display "O" count
                    println!("Player X score: {}", x_count); // Display "X" count
                }
                else if x_count == o_count { // It's a tie! ...maybe play again?
                    println!("--------------");
                    println!("It's a tie!");
                    println!("--------------\n");
                    println!("Both Player's scored: {}", x_count); // Display the shared count
                }
                playing = false; // Game is over
            }
            was_skipped = true; // Current player is skipped
        }
        is_player_one = !is_player_one; // Switch who's turn it is
    }
}

/***********************************************************
 * CREATE_BOARD
 ***********************************************************
 * Create the starting board for games
***********************************************************/
fn create_board(board: &mut [[&str; SIZE]; SIZE]) {
    let m = SIZE / 2;           // Find the middle
    board[m - 1][m - 1] = _O;   // Northwest "O"
    board[m - 1][  m  ] = _X;   // Northeast "X"
    board[  m  ][m - 1] = _X;   // Southwest "X"
    board[  m  ][  m  ] = _O;   // Southeast "O"
}

/***********************************************************
 * DISPLAY
 ***********************************************************
 * Display the game board to the user/users
***********************************************************/
fn display(board: [[&str; SIZE]; SIZE]) {
    let mut line = "  -".to_string();   // used for horizontal grid lines
    print!("  ");                       // Proper spacing for column numbers
    for i in 0..SIZE {                  // For each column print the column number
        print!("  {} ", i + 1);         // Account for off-by-one (0...SIZE to 1...SIZES)
        line += "----";                 // Add length to the horizontal grid lines for each column
    }
    println!();                         // End column number line
    println!("{}", line);               // Top of grid
    for (i1, row) in board.iter().enumerate() {             // For each row and column print the grid
        print!("{} |", (i1 + 'A' as usize) as u8 as char);  // Print the row letter starting at "A"
        for (_i2, column) in row.iter().enumerate() {
            print!(" {} |", column);    // Print the board value and the veritical grid line
        }
        println!();                     // Finsh the row
        println!("{}", line);           // Print the next horizontal grid line
    }
}

/***********************************************************
 * TAKE_TURN
 ***********************************************************
 * The player takes their turn by entering a location on the
 * board. If it's a valid location capture the opponent's
 * pieces and end the player's turn.
***********************************************************/
fn take_turn(board: &mut [[&str; SIZE]; SIZE], is_player_one: bool) {
    let mut valid_move = false;     // The player needs to enter a valid location
    let mut num: usize = 0;         // The column number index
    let mut letter: usize = 0;      // The row letter index (index is translated from a char)
    while !valid_move {             // Until the player enters a valid location...
        let pos: String = read!();  // Get the input from the user
        if get_coordinates(&mut num, &mut letter, pos) {        // Try to translate the user input into indexes
            if try_capture(num, letter, board, is_player_one) { // Try to capture the opponent's pieces
                valid_move = true;  // If the input was valid and could capture then it was a valid move
                if is_player_one {  // Place the current player's piece at the location
                    board[letter][num] = _X;
                }
                else {
                    board[letter][num] = _O;
                }
            }
        }
    }
}

/***********************************************************
 * GET_COORDINATES
 ***********************************************************
 * Translate the user input into location indexes, if you
 * can. Return true if it works, otherwise false.
***********************************************************/
fn get_coordinates(num: &mut usize, letter: &mut usize, pos: String) -> bool {
    let mut is_valid = false;       // Bool returned from the function, true if valid location
    if pos.chars().count() == 2 {   // The user should only have input two characters (a letter and a number)

        // If the first character is a number and the second is a letter...
        if pos.chars().nth(0).unwrap().is_numeric() && pos.chars().nth(1).unwrap().is_alphabetic() {
            // Set the number index from the first char
            *num = pos.chars().nth(0).unwrap() as usize - 49;
            // Set the letter index from the second char
            *letter = pos.chars().nth(1).unwrap().to_ascii_uppercase() as usize - 'A' as usize;
            is_valid = true; // Is a valid input
        }
        // Else if the first character is a letter and the second is a number...
        else if pos.chars().nth(1).unwrap().is_numeric() && pos.chars().nth(0).unwrap().is_alphabetic() {
            // Set the number index from the second char
            *num = pos.chars().nth(1).unwrap() as usize - 49;
            // Set the letter index from the first char
            *letter = pos.chars().nth(0).unwrap().to_ascii_uppercase() as usize - 'A' as usize;
            is_valid = true; // Is a valid input
        }
    }
    return is_valid; // Return whether the input was valid or not
}

/***********************************************************
 * TRY_CAPTURE
 ***********************************************************
 * Try to capture the opponent's piece[s]. If it does, then
 * return true, otherwise return false.
***********************************************************/
fn try_capture(num: usize, letter: usize, board: &mut [[&str; SIZE]; SIZE], is_player_one: bool) -> bool {
    let mut is_valid = false;       // If the location on the board is a valid capture location it will become true
    let p_piece: &str;              // Holds the current player's piece
    let o_piece: &str;              // Holds the current opponent's piece
    if board[letter][num] != " " {  // If the location isn't empty, return to the previous function
        return is_valid;            // is_valid is false here
    }
    if is_player_one {              // if current player is player one...
        p_piece = _X;               // Player piece is "X"
        o_piece = _O;               // Opponent piece is "O"
    }
    else {                          // Else if current player isn't player one...
        p_piece = _O;               // Player piece is "O"
        o_piece = _X                // Opponent piece is "X"
    }

    if num > 0 {                                    // Check all situations where the number index > 0
        if check(num, letter, 0, 0, -1, 0,
                 board, p_piece, o_piece) {         // Check left (if letter is != 0)
            is_valid = true;                        // Capture successful
        }
        if check(num, letter, 0, SIZE - 1, -1, 0,
                 board, p_piece, o_piece) {         // Check left again (in case letter is == 0)
            is_valid = true;                        // Capture successful
        }
        if letter > 0 {                             // If letter > 0...
            if check(num, letter, 0, 0, -1, -1,
                     board, p_piece, o_piece) {     // Check left-up diagonal
                is_valid = true;                    // Capture successful
            }
        }
        if letter < SIZE - 1 {                      // If letter < SIZE - 1...
            if check(num, letter, 0, SIZE - 1, -1, 1,
                     board, p_piece, o_piece) {     // Check left-down diagonal
                is_valid = true;                    // Capture successful
            }
        }
    }
    if num < SIZE - 1 {                             // Check all situations where the number index < SIZE - 1
        if check(num, letter, SIZE - 1, 0, 1, 0,
                 board, p_piece, o_piece) {         // Check right (if letter != 0)
            is_valid = true;                        // Capture successful
        }
        if check(num, letter, SIZE - 1, SIZE - 1, 1, 0,
                 board, p_piece, o_piece) {         // Check right again (in case letter == 0)
            is_valid = true;                        // Capture successful
        }
        if letter > 0 {                             // If letter > 0...
            if check(num, letter, SIZE - 1, 0, 1, -1,
                     board, p_piece, o_piece) {     // Check right-up diagonal
                is_valid = true;                    // Capture successful
            }
        }
        if letter < SIZE - 1 {                      // If letter < SIZE -1...
            if check(num, letter, SIZE - 1, SIZE - 1, 1, 1,
                     board, p_piece, o_piece) {     // Check right-down diagonal
                is_valid = true;                    // Capture successful
            }
        }
    }                                               // Only checks still needed are up and down (with no left or right)
    if letter > 0 {                                 // If letter > 0...
        if check(num, letter, 0, 0, 0, -1,
                 board, p_piece, o_piece) {         // Check up (if number != 0)
            is_valid = true;                        // Capture successful
        }
        if check(num, letter, SIZE - 1, 0, 0, -1,
                 board, p_piece, o_piece) {         // Check up again (in case number == 0)
            is_valid = true;                        // Capture successful
        }
    }
    if letter < SIZE -1 {                           // If letter < SIZE - 1
        if check(num, letter, 0, SIZE - 1, 0, 1,
                 board, p_piece, o_piece) {         // Check down (if number != 0)
            is_valid = true;                        // Capture successful
        }
        if check(num, letter, SIZE - 1, SIZE - 1, 0, 1,
                 board, p_piece, o_piece) {         // Check down again (in case number == 0)
            is_valid = true;                        // Capture successful
        }
    }
    return is_valid;                                // Return true if capture was successful
}

/***********************************************************
 * CAN_TAKE_TURN
 ***********************************************************
 * Checks every location on the board for empty (" ")
 * locations. When a location is found it check if it's
 * possible to capture any of the current opponent's
 * pieces. 
***********************************************************/
fn can_take_turn(board: &mut [[&str; SIZE]; SIZE], is_player_one: bool) -> bool {
    let p_piece: &str;      // Holds the current player's piece
    let o_piece: &str;      // Holds the current opponent's piece
    let it_board = *board;  // Create a copy of the board for iteration (mutable/immutable conflict)
    if is_player_one {      // If current player is player one...
        p_piece = _X;       // Player piece is "X"
        o_piece = _O;       // Opponent piece is "O"
    }
    else {                  // Otherwise...
        p_piece = _O;       // Player piece is "O"
        o_piece = _X;       // Opponent piece is "X"
    }

    // Iterate through each row, and each column in each row
    for (i1, row) in it_board.iter().enumerate() {
        for (i2, column) in row.iter().enumerate() {
            if column as &str == " " {                      // If location is empty (" ")...
                let letter = i1;                            // Set the letter index to the row iterator
                let num = i2;                               // Set the number index to the column iterator
                if num > 0 {                                // Check all situations where number index > 0
                    if check(num, letter, 0, 0, -1, 0,
                             board, p_piece, o_piece) {     // Check left (if letter != 0)
                        return true;                        // Player can take their turn
                    }
                    if check(num, letter, 0, SIZE - 1, -1, 0,
                             board, p_piece, o_piece) {     // Check left (in case letter == 0)
                        return true;                        // Player can take their turn
                    }
                    if letter > 0 {                         // If letter > 0...
                        if check(num, letter, 0, 0, -1, -1,
                                 board, p_piece, o_piece) { // Check left-up
                            return true;                    // Player can take their turn
                        }
                    }
                    if letter < SIZE - 1 {                  // If letter < SIZE -1...
                        if check(num, letter, 0, SIZE - 1, -1, 1,
                                 board, p_piece, o_piece) { // Check left-down
                            return true;                    // Player can take their turn
                        }
                    }
                }
                if num < SIZE - 1 {                         // Check all situations where the number index < SIZE -1
                    if check(num, letter, SIZE - 1, 0, 1, 0,
                             board, p_piece, o_piece) {     // Check right (if letter != 0)
                        return true;                        // Player can take their turn
                    }
                    if check(num, letter, SIZE - 1, SIZE - 1, 1, 0,
                             board, p_piece, o_piece) {     // Check right (in case letter == 0)
                        return true;                        // Player can take their turn
                    }
                    if letter > 0 {                         // If letter > 0...
                        if check(num, letter, SIZE - 1, 0, 1, -1,
                                 board, p_piece, o_piece) { // Check right-up
                            return true;                    // Player can take their turn
                        }
                    }
                    if letter < SIZE - 1 {                  // If letter < SIZE - 1...
                        if check(num, letter, SIZE - 1, SIZE - 1, 1, 1,
                                 board, p_piece, o_piece) { // Check right-down
                            return true;                    // Player can take their turn
                        }
                    }
                }                                           // Only checks still needed are up and down (no left or right)
                if letter > 0 {                             // If letter > 0...
                    if check(num, letter, 0, 0, 0, -1,
                             board, p_piece, o_piece) {     // Check up (if number != 0)
                        return true;                        // Player can take their turn
                    }
                    if check(num, letter, SIZE - 1, 0, 0, -1,
                             board, p_piece, o_piece) {     // Check up (in case number == 0)
                        return true;                        // Player can take their turn
                    }
                }
                if letter < SIZE -1 {                       // If letter < SIZE - 1...
                    if check(num, letter, 0, SIZE - 1, 0, 1,
                             board, p_piece, o_piece) {     // Check down (if number != 0)
                        return true;                        // Player can take their turn
                    }
                    if check(num, letter, SIZE - 1, SIZE - 1, 0, 1,
                             board, p_piece, o_piece) {     // Check down (in case number == 0)
                        return true;                        // Player can take their turn
                    }
                }
            }
        }
    }
    return false;                                           // Failed all checks, the player can't take their turn
}

/***********************************************************
 * CHECK
 ***********************************************************
 * Checks if a capture can take place, and if it can it
 * captures all the relevant pieces. This is done by taking
 * in both indexes and their iteration value (-1, 0, or 1)
 * and looping through the line until the board edge (limit)
 * is reached, an empty space is reached, or one of the
 * current player's pieces are reached. If any of the
 * opponent's pieces were found they are captured and
 * true is returned. In all other cases false is returned.
***********************************************************/
fn check(num: usize, letter: usize,
         limit_n: usize, limit_l: usize,
         it_n: i8, it_l: i8,
         board: &mut [[&str; SIZE]; SIZE],
         p_piece: &str, o_piece: &str)
         -> bool {
    
    let mut n = num;                                // Copy number index for iteration
    let mut l = letter;                             // Copy letter index for iteration
    let mut v = Vec::new();                         // Create a Vector to hold potential capturable pieces
    while n != limit_n && l != limit_l {            // Until the edge of the board is reached...
        n = ((n as i8) + it_n) as usize;            // Iterate the number index (usize can't iterate without casting)
        l = ((l as i8) + it_l) as usize;            // Iterate the letter index (usize can't iterate without casting)
        if board[l][n] == o_piece {                 // If an opponent's piece is found...
            let nl = NumLet {                       // Create a NumLet struct to hold the location and...
                n:n,    // Number index
                l:l     // Letter index
            };
            v.push(nl);                             // Store the struct in the Vector
        }
        else if board[l][n] == p_piece {            // If one of the current player's pieces are found...
            if v.len() != 0 {                       // Check if the Vector holds any locations. If it does...
                while v.len() != 0 {                // Loop through the Vector until it is empty
                    let pos = v.remove(0);          // Remove and store the location from the Vector
                    if p_piece == _X {              // If the current player is "X"...
                        board[pos.l][pos.n] = _X;   // Change that location to "X"
                    }
                    else {                          // Otherwise the player is "O", so...
                        board[pos.l][pos.n] = _O;   // Change that location to "O"
                    }
                }
                return true;                        // Once the Vetor is empty return true (you did capture)
            }
            return false;                           // If the Vector was empty then return false (you didn't capture)
        }
        else {                                      // If it's not the current player's piece or the opponent's piece...
            return false;                           // It's an empty location, so return false
        }
    }
    return false;                                   // If you reach the edge of the board return false
}