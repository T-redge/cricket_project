pub mod bat {
    //Structs
    pub struct BatterProfile {
        name: String,
        runs_scored: u32,
        balls_faced: u32,
        pub on_strike: bool,
        run_profile: RunProfile,
    }
    impl BatterProfile {
        //Init Function
        pub fn init_batter(on_strike: bool) -> Self {
            Self {
                name: Self::get_name().trim().to_string(),
                runs_scored: 0,
                balls_faced: 0,
                on_strike,
                run_profile: RunProfile::init_run_profile(),
            }
        }
        //Adds to balls faced
        pub fn ball_faced(&mut self) {
            self.balls_faced += 1;
        }
        //Adds to dot balls faced
        pub fn runs_scored_dot(&mut self) {
            self.run_profile.add_dot();
        }
        //Adds to runs scored and ball faced
        pub fn runs_scored_one(&mut self) {
            self.runs_scored += 1;
            self.run_profile.add_run_one();
        }
        //Adds to runs scored and ball faced
        pub fn runs_scored_two(&mut self) {
            self.runs_scored += 2;
            self.run_profile.add_run_two();
        }
        //Adds to runs scored and ball faced
        pub fn runs_scored_three(&mut self) {
            self.runs_scored += 3;
            self.run_profile.add_run_three();
        }
        //Adds to runs scored and ball faced
        pub fn runs_scored_four(&mut self) {
            self.runs_scored += 4;
            self.run_profile.add_run_four();
        }
        //Adds to runs scored and ball faced
        pub fn runs_scored_six(&mut self) {
            self.runs_scored += 6;
            self.run_profile.add_run_six();
        }
        //Returns true if on strike
        pub fn check_on_strike(&self) -> bool {
            if self.on_strike {
                return true;
            }
            else {
                return false;
            }
        }
        //Changes on_strike variable to true or false
        pub fn change_strike(&mut self) {
            if self.on_strike {
                self.on_strike = false;
            }
            else {
                self.on_strike = true;
            }
        }
        //Print struct variable functions
        pub fn print_bp(&self) {
            print!("{} ", self.name);
            print!("( Balls: {} ", self.balls_faced);
            println!("Runs: {} )", self.runs_scored);
            self.run_profile.print_rp();
        }
        //Gets string from input
        pub fn get_input() -> String {
            let mut tmp = String::new();
            std::io::stdin()
            .read_line(&mut tmp)
            .expect("Failed to readline");
        tmp
        }
        //Returns char from stdin
        pub fn get_char() -> char {
            loop {
                let tmp_c = Self::get_input();
                
                let tmp_c: char = match tmp_c.trim().parse() {
                    Ok(char) => char,
                    Err(_) => {
                        println!("Wrong Input! Try again");
                        continue;
                    },  
                };
                return tmp_c;
            }
        }
        //returns num from string
        pub fn get_num() -> u32 {
            loop {
                let tmp_num = Self::get_input();
                
                let tmp_num: u32 = match tmp_num.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Wrong input! Try again");
                        continue;
                    },
                };
                return tmp_num;
            }
        }
        //Returns name from stdin
        pub fn get_name() -> String {
            println!("Input player name: ");
            let tmp_name = Self::get_input();
            tmp_name
        }
    }
   
    struct RunProfile {
        dot_balls: u32,
        runs_one: u32,
        runs_two: u32,
        runs_three: u32,
        runs_four: u32,
        runs_six: u32,
    }
    impl RunProfile {
        fn init_run_profile() -> RunProfile {
            RunProfile {
                dot_balls: 0,
                runs_one: 0,
                runs_two: 0,
                runs_three: 0,
                runs_four: 0,
                runs_six: 0,
            }
        }
        //Adds one to type of run scored
        fn add_dot(&mut self) {
            self.dot_balls += 1;
        }
        fn add_run_one(&mut self) {
            self.runs_one += 1;
        }
        fn add_run_two(&mut self) {
            self.runs_two += 1;
        }
        fn add_run_three(&mut self) {
            self.runs_three += 1;
        }
        fn add_run_four(&mut self) {
            self.runs_four += 1;
        }
        fn add_run_six(&mut self) {
            self.runs_six += 1;
        }
        fn print_rp(&self) {
            print!("Dot's: {} |", self.dot_balls);
            print!(" 1's: {} |", self.runs_one);
            print!(" 2's: {} |", self.runs_two);
            print!(" 3's: {} |", self.runs_three);
            print!(" 4's: {} |", self.runs_four);
            println!(" 6's: {} |", self.runs_six);
        }
    }
    
}