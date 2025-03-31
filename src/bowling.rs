pub mod bowling {
    pub struct BowlerProfile {
        name: String,
        overs_bowled: u32,
        maidens_bowled: u32,
        wickets_taken: u32,
        runs_conceded: u32,
       
    }
    impl BowlerProfile {
        pub fn init_bowler() -> Self {
            Self {
                name: crate::BatterProfile::get_name().trim().to_string(),
                overs_bowled: 0,
                maidens_bowled: 0,
                wickets_taken: 0,
                runs_conceded: 0,
            }
        }
        pub fn print_bowler_info(&self) {
            print!("{}  ", self.name);
            print!("{}-", self.overs_bowled);
            print!("{}-", self.maidens_bowled);
            print!("{}-", self.wickets_taken);
            println!("{}",self.runs_conceded);
        }
    }
}