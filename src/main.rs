use crate::bat::bat::*;
use crate::input::input::*;
use crate::over::over::*;
pub mod bat;
pub mod input;
pub mod over;

//--- Checks how many runs where scored by batter ---//
fn check_runs_scored(bat: &mut BatterProfile) -> u32 {
    
    println!("Enter runs scored: 1, 2, 3, 4, 6");
    loop {

        let runs_scored = get_num();

        match runs_scored {
                1 => {
                    bat.runs_scored_one();
                    return runs_scored;
                },
                2 => {
                    bat.runs_scored_two();
                    return runs_scored;
                },
                3 => {
                    bat.runs_scored_three();
                    return runs_scored;
                },
                4 => {
                    bat.runs_scored_four();
                    return runs_scored;
                },
                6 => {
                    bat.runs_scored_six();
                    return runs_scored;
                },
                _ => {
                    println!("Wrong input! Try again");
                    continue;
                },
        };
        
    }
}

fn noball_hit() -> bool {
    println!("Runs scored off No-Ball: Yes(y), No(n)");
    loop {

        let tmp_bool = get_char();
        match tmp_bool {
            'y' => return true,
            'n' => return false,
            _ => {
                println!("Wrong input! try again");
                continue;
            },
        }
    }
}

//--- Checks which extra has been bowled ---//
fn check_extra_event(bat: &mut BatterProfile) -> String {
    println!("Enter extra type: Wide(w), No-Ball(n), Bye(b), Leg-Bye(l)");
    loop {

        let tmp_c = get_char();
        
        match tmp_c {
            'w' => {
                bat.runs_scored_dot();
                let tmp_c: String = String::from("Wide");
                return tmp_c;
            },
            'n' => {
                
                let tmp_c: String = String::from("No-Ball");
                if noball_hit() {
                    let tmp_rs = check_runs_scored(bat);
                    println!("Runs scored: {}", tmp_rs);
                    println!("+");
                }
                else {
                    bat.runs_scored_dot();
                }
                return tmp_c;
            },
            'b' => {
                bat.runs_scored_dot();
                let tmp_c: String = String::from("Bye");
                return tmp_c;
            },
            'l' => {
                bat.runs_scored_dot();
                let tmp_c: String = String::from("Leg-Bye");
                return tmp_c;
            },
            _ => {
                println!("Wrong Input! Try again");
                continue;
            },
        }
    }
}

//--- Checks what occur once ball has been bowled ---//
fn check_ball_event(bat: &mut BatterProfile) {
    println!("What happened that ball");
    println!("Enter event: Dot(.), Run(r), Extra(e), Wicket(w)");
    loop {
        let ball_event = get_char();

        match ball_event {
            '.' => {
                bat.runs_scored_dot();
                println!("A Dot ball has been bowled");
            },
            'r' => {
                let tmp_rs = check_runs_scored(bat);
                println!("Runs scored: {}", tmp_rs);
            }, 
            'e' => {
                let tmp_es: String = check_extra_event(bat);
                println!("Extra bowled: {}", tmp_es);
            },
            'w' => println!("A wicket has been taken"),
            _ => {
                println!("Wrong input! Try again");
                continue;
            },
        }
        break;
    }
}

//--- Bowls atleast 6 balls to bat ---//
fn bowl_over(bat: &mut BatterProfile) {
    let mut balls_bowled = 0;
    while balls_bowled < 6 {
        bat.ball_faced();
        check_ball_event(bat);
        balls_bowled += 1;
    }
    println!("Over Completed.");    
}

fn main() {
    let _over = init_over();
    let mut bat_one = init_batter();
    
    bowl_over(&mut bat_one);

    print_bp(bat_one);
    
   
}
