use crate::over::input::input::*;
pub mod input;


pub mod over {
    
    #[derive(Clone)]
    pub struct Over {
        balls_bowled: u32,
        ball_record: std::vec::Vec<char>,
    }
    impl Over {
         //Init over
        pub fn init_over() -> Over {
            Over {
                balls_bowled: 0,
                ball_record: vec![],
            }
        }    
        //--- Bowls atleast 6 balls to bat ---//
        pub fn bowl_over(&mut self, 
            bat_one: &mut crate::bat::bat::BatterProfile, 
            bat_two: &mut crate::bat::bat::BatterProfile) 
        {
            while self.balls_bowled < 6 {
                if bat_one.check_on_strike() {
                    bat_one.ball_faced();
                    self.check_ball_event(bat_one);
                }
                if bat_two.check_on_strike() {
                    bat_one.ball_faced();
                    self.check_ball_event(bat_two);
                }
                self.balls_bowled += 1;
            }
            println!("Over Completed.");
            self.print_over();
            bat_one.print_bp();
            bat_two.print_bp();
        }

        //--- Add to balls_bowled counter ---//
        pub fn ball_bowled(&mut self) {
            self.balls_bowled += 1;
        }
        
        //--- Checks how many runs where scored by batter ---//
        pub fn check_runs_scored(&mut self, bat: &mut crate::bat::bat::BatterProfile) -> u32 {
            
            println!("Enter runs scored: 1, 2, 3, 4, 6");
            loop {
    
                let runs_scored = crate::over::get_num();
    
                match runs_scored {
                        1 => {
                            bat.runs_scored_one();
                            self.push_to_ball_record('1');
                            return runs_scored;
                        },
                        2 => {
                            bat.runs_scored_two();
                            self.push_to_ball_record('2');
                            return runs_scored;
                        },
                        3 => {
                            bat.runs_scored_three();
                            self.push_to_ball_record('3');
                            return runs_scored;
                        },
                        4 => {
                            bat.runs_scored_four();
                            self.push_to_ball_record('4');
                            return runs_scored;
                        },
                        6 => {
                            bat.runs_scored_six();
                            self.push_to_ball_record('6');
                            return runs_scored;
                        },
                        _ => {
                            println!("Wrong input! Try again");
                            continue;
                        },
                };
                
            }
        }
    
        //--- Checks if No ball was hit for runs ---//
        fn noball_hit() -> bool {
            println!("Runs scored off No-Ball: Yes(y), No(n)");
            loop {
    
                let tmp_bool = crate::over::get_char();
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
    
        //--- Decreases balls_bowled count due to wide/noball ---//
        pub fn rebowl_ball(&mut self) {
            self.balls_bowled -= 1;
        }
    
        //--- Checks which extra has been bowled ---//
        pub fn check_extra_event(&mut self, bat: &mut crate::bat::bat::BatterProfile) -> String {
            println!("Enter extra type: Wide(w), No-Ball(n), Bye(b), Leg-Bye(l)");
            loop {
    
                let tmp_c = crate::over::get_char();
                
                match tmp_c {
                    'w' => {
                        bat.runs_scored_dot();
                        let tmp_c: String = String::from("Wide");
                        self.rebowl_ball();
                        self.push_to_ball_record('w');
                        return tmp_c;
                    },
                    'n' => {
                        
                        let tmp_c: String = String::from("No-Ball");
                        let tmp_bool = Over::noball_hit();
                        if tmp_bool {
                            let tmp_rs = self.check_runs_scored(bat);
                            println!("Runs scored: {}", tmp_rs);
                            println!("+");
                        }
                        else {
                            bat.runs_scored_dot();
                        }
                        self.rebowl_ball();
                        self.push_to_ball_record('n');
                        return tmp_c;
                    },
                    'b' => {
                        bat.runs_scored_dot();
                        let tmp_c: String = String::from("Bye");
                        self.push_to_ball_record('b');
                        return tmp_c;
                    },
                    'l' => {
                        bat.runs_scored_dot();
                        let tmp_c: String = String::from("Leg-Bye");
                        self.push_to_ball_record('l');
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
        pub fn check_ball_event(&mut self, bat: &mut crate::bat::bat::BatterProfile) {
        println!("What happened that ball");
        println!("Enter event: Dot(.), Run(r), Extra(e), Wicket(x)");
        loop {
            let ball_event = crate::over::get_char();
    
            match ball_event {
                '.' => {
                    bat.runs_scored_dot();
                    self.push_to_ball_record('.');
                    println!("A Dot ball has been bowled");
                },
                'r' => {
                    let tmp_rs = self.check_runs_scored(bat);
                    println!("Runs scored: {}", tmp_rs);
                }, 
                'e' => {
                    let tmp_es: String = self.check_extra_event(bat);
                    println!("Extra bowled: {}", tmp_es);
                },
                'x' => {
                    self.push_to_ball_record('x');
                    println!("A wicket has been taken");
                },
                _ => {
                    println!("Wrong input! Try again");
                    continue;
                },
            }
            break;
        }
    }
    
        //--- Print over information ---//
        pub fn print_over(&self) {
            print!("Over Bowled ( ");
            for element in self.ball_record.clone() {
                print!("{element} ")
            }
            println!(" )");
        }
    
        pub fn push_to_ball_record(&mut self, event: char) {
            self.ball_record.push(event);
        }
    }
}