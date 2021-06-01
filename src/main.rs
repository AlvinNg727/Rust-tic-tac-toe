use std::io;

#[derive(Copy, Clone, Debug)]
struct Moves {
    t_l: char, //top left
    t_c: char, //top center
    t_r: char, //top right
    c_l: char, //center left
    c_c: char, //center center
    c_r: char, //center right
    b_l: char, //bottom left
    b_c: char, //bottom center
    b_r: char, //bottom right
}

impl Moves {
    fn print_board(&self) {
        let mut print = String::new();

        for i in 0..9 {
            match i {
                0 => {
                    print.push_str("     |     |\n  ");
                    print.push_str(&match_move(self.t_l, i + 1));
                    print.push_str("  |")
                }
                1 => {
                    print.push_str("  ");
                    print.push_str(&match_move(self.t_c, i + 1));
                    print.push_str("  |")
                }
                2 => {
                    print.push_str("  ");
                    print.push_str(&match_move(self.t_r, i + 1));
                    print.push_str("  \n_____|_____|_____\n")
                }
                3 => {
                    print.push_str("     |     |\n  ");
                    print.push_str(&match_move(self.c_l, i + 1));
                    print.push_str("  |")
                }
                4 => {
                    print.push_str("  ");
                    print.push_str(&match_move(self.c_c, i + 1));
                    print.push_str("  |")
                }
                5 => {
                    print.push_str("  ");
                    print.push_str(&match_move(self.c_r, i + 1));
                    print.push_str("  \n_____|_____|_____\n")
                }
                6 => {
                    print.push_str("     |     |\n  ");
                    print.push_str(&match_move(self.b_l, i + 1));
                    print.push_str("  |")
                }
                7 => {
                    print.push_str("  ");
                    print.push_str(&match_move(self.b_c, i + 1));
                    print.push_str("  |")
                }
                8 => {
                    print.push_str("  ");
                    print.push_str(&match_move(self.b_r, i + 1));
                    print.push_str("  \n     |     |")
                }
                _ => println!("wtf(print_board)"),
            }
        }

        println!("{}", print);
    }

    fn update_board(&mut self, current_move: u32, player: char) {
        match current_move {
            1 => self.t_l = player,
            2 => self.t_c = player,
            3 => self.t_r = player,
            4 => self.c_l = player,
            5 => self.c_c = player,
            6 => self.c_r = player,
            7 => self.b_l = player,
            8 => self.b_c = player,
            9 => self.b_r = player,
            _ => println!("wtf(update_board)"),
        }
    }
}

fn match_move(current: char, index: u32) -> String {
    match current {
        '_' => return index.to_string(),
        'X' => return String::from("X"),
        'O' => return String::from("O"),
        _ => println!("wtf(match_move)"),
    }
    return String::from("error");
}

fn main() {
    let mut player_moves = Moves {
        t_l: '_',
        t_c: '_',
        t_r: '_',
        c_l: '_',
        c_c: '_',
        c_r: '_',
        b_l: '_',
        b_c: '_',
        b_r: '_',
    };

    let mut current_move: u32;

    let mut current_player: char = 'X';

    let mut winner: u32 = 0; // 0 = continue game, 1 = bot won, 2 = you won

    // game loop
    loop {
        player_moves.print_board();

        println!("Player {}'s turn", current_player);

        current_move = get_move();

        match check_valid_move(current_move, player_moves) {
            true => (),
            false => {
                println!("Invalid move\n");
                continue;
            }
        };

        player_moves.update_board(current_move, current_player);

        winner = check_won(player_moves, current_player);

        match current_player {
            'X' => current_player = 'O',
            'O' => current_player = 'X',
            _ => println!("wtf"),
        }

        match winner {
            3 => {
                println!("Tie");
                break;
            }
            2 => {
                println!("Player O wins");
                break;
            }
            1 => {
                println!("Player X wins");
                break;
            }
            0 => (),
            _ => break,
        };
    }
}

// get player move
fn get_move() -> u32 {
    loop {
        println!("Please input your move:");

        let mut player_move = String::new();

        io::stdin()
            .read_line(&mut player_move)
            .expect("Failed to read line");

        println!("");

        let _player_move: u32 = match player_move.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter in a number\n");
                continue;
            }
        };
    }
}

// check if the player move is valid
fn check_valid_move(player_move: u32, past_moves: Moves) -> bool {
    if player_move <= 0 {
        return false;
    } else if player_move > 9 {
        return false;
    }

    match player_move {
        1 => return past_moves.t_l != 'X' && past_moves.t_l != 'O',
        2 => return past_moves.t_c != 'X' && past_moves.t_c != 'O',
        3 => return past_moves.t_r != 'X' && past_moves.t_r != 'O',
        4 => return past_moves.c_l != 'X' && past_moves.c_l != 'O',
        5 => return past_moves.c_c != 'X' && past_moves.c_c != 'O',
        6 => return past_moves.c_r != 'X' && past_moves.c_r != 'O',
        7 => return past_moves.b_l != 'X' && past_moves.b_l != 'O',
        8 => return past_moves.b_c != 'X' && past_moves.b_c != 'O',
        9 => return past_moves.b_r != 'X' && past_moves.b_r != 'O',
        _ => println!("wtf(check_valid_move)"),
    }

    return true;
}

fn check_won(past_moves: Moves, current_player: char) -> u32 {
    //horizontal
    if past_moves.t_l == past_moves.t_c && past_moves.t_c == past_moves.t_r && past_moves.t_c != '_'
    {
        match current_player {
            'X' => return 1,
            'O' => return 2,
            _ => println!("wtf(check_won)"),
        }
    } else if past_moves.c_l == past_moves.c_c
        && past_moves.c_c == past_moves.c_r
        && past_moves.c_c != '_'
    {
        match current_player {
            'X' => return 1,
            'O' => return 2,
            _ => println!("wtf(check_won)"),
        }
    } else if past_moves.b_l == past_moves.b_c
        && past_moves.b_c == past_moves.b_r
        && past_moves.b_c != '_'
    {
        match current_player {
            'X' => return 1,
            'O' => return 2,
            _ => println!("wtf(check_won)"),
        }
    }

    //vertical
    if past_moves.t_l == past_moves.c_l && past_moves.c_l == past_moves.b_l && past_moves.c_l != '_'
    {
        match current_player {
            'X' => return 1,
            'O' => return 2,
            _ => println!("wtf(check_won)"),
        }
    } else if past_moves.t_c == past_moves.c_c
        && past_moves.c_c == past_moves.b_c
        && past_moves.c_c != '_'
    {
        match current_player {
            'X' => return 1,
            'O' => return 2,
            _ => println!("wtf(check_won)"),
        }
    } else if past_moves.t_r == past_moves.c_r
        && past_moves.c_r == past_moves.b_r
        && past_moves.c_r != '_'
    {
        match current_player {
            'X' => return 1,
            'O' => return 2,
            _ => println!("wtf(check_won)"),
        }
    }

    //diagonal
    if past_moves.t_l == past_moves.c_c && past_moves.c_c == past_moves.b_r && past_moves.c_c != '_'
    {
        match current_player {
            'X' => return 1,
            'O' => return 2,
            _ => println!("wtf(check_won)"),
        }
    } else if past_moves.t_r == past_moves.c_c
        && past_moves.c_c == past_moves.b_l
        && past_moves.c_c != '_'
    {
        match current_player {
            'X' => return 1,
            'O' => return 2,
            _ => println!("wtf(check_won)"),
        }
    }

    //check if game is still going
    for i in 0..9 {
        match i {
            0 => {
                if past_moves.t_l == '_' {
                    return 0;
                }
            }
            1 => {
                if past_moves.t_c == '_' {
                    return 0;
                }
            }
            2 => {
                if past_moves.t_r == '_' {
                    return 0;
                }
            }
            3 => {
                if past_moves.c_l == '_' {
                    return 0;
                }
            }
            4 => {
                if past_moves.c_c == '_' {
                    return 0;
                }
            }
            5 => {
                if past_moves.c_r == '_' {
                    return 0;
                }
            }
            6 => {
                if past_moves.b_l == '_' {
                    return 0;
                }
            }
            7 => {
                if past_moves.b_c == '_' {
                    return 0;
                }
            }
            8 => {
                if past_moves.b_r == '_' {
                    return 0;
                }
            }
            _ => println!("wtf(check_won)"),
        }
    }

    return 3;
}
