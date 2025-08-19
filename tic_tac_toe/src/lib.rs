pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    let player1 = 'O' as char;
    let player2 = 'X' as char;

    // Checking diagonals
    if diagonals(player1, table) {
        return "player O won".to_string();
    } else if diagonals(player2, table) {
        return "player X won".to_string();
    }

    // Checking horizontal
    if horizontal(player1, table) {
        return "player O won".to_string();
    } else if horizontal(player2, table) {
        return "player X won".to_string();
    }

    // Checking vertical
    if vertical(player1, table) {
        return "player O won".to_string();
    } else if vertical(player2, table) {
        return "player X won".to_string();
    }

    // Its a tie if we haven't found a winner
    "tie".to_string()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    // Checking for / diagonals
    for index in 0..3 {
        if table[index][index] != player {
            break;
        } else if index == 2 {
            return true;
        }
    }

    // Checking for \ diagonals
    for index in 0..3 {
        if table[index][2 - index] != player {
            break;
        } else if index == 2 {
            return true;
        }
    }

    false
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    // Checking line by line
    for x in 0..3 {
        for y in 0..3 {
            if table[x][y] != player {
                break;

            // If y get to 2 without breaking, return true
            } else if y == 2 {
                return true;
            }
        }
    }

    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    // Checking column by column
    for y in 0..3 {
        for x in 0..3 {
            if table[x][y] != player {
                break;

            // If x get to 2 without breaking, return true
            } else if x == 2 {
                return true;
            }
        }
    }

    false
}
