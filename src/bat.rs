pub mod bat {
    //Structs
    pub struct BatterProfile {
        name: String,
        runs_scored: u32,
        balls_faced: u32,
        run_profile: RunProfile,
    }
    impl BatterProfile {
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
        
    }
    
    //Init Functions
    pub fn init_batter() -> BatterProfile {
        let tmp_b = BatterProfile {

            name: String::from("Alex"),
            runs_scored: 0,
            balls_faced: 0,
            run_profile: init_run_profile(),  
        };
        tmp_b
    }
    fn init_run_profile() -> RunProfile {
        let tmp_rp = RunProfile {
            dot_balls: 0,
            runs_one: 0,
            runs_two: 0,
            runs_three: 0,
            runs_four: 0,
            runs_six: 0,
        };
        tmp_rp
    }

    //Print struct variable functions
    pub fn print_bp(bp: BatterProfile) {
        println!("---Batter Profile---");
        println!("Name: {}", bp.name);
        println!("Runs: {}", bp.runs_scored);
        println!("Balls: {}", bp.balls_faced);
        print_rp(bp.run_profile);
        
    }
    fn print_rp(rp: RunProfile) {
        println!("--Run Profile--");
        println!("Dot's: {}", rp.dot_balls);
        println!("1's: {}", rp.runs_one);
        println!("2's: {}", rp.runs_two);
        println!("3's: {}", rp.runs_three);
        println!("4's: {}", rp.runs_four);
        println!("6's: {}", rp.runs_six);
    }
}