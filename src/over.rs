pub mod over {
    pub struct Over {
        balls_bowled: u32,
        ball_record: std::vec::Vec<char>,
    }

    pub fn init_over() -> Over {
        let tmp_over = Over {
            balls_bowled: 0,
            ball_record: vec![' '],
        };
        tmp_over
    }

    pub fn ball_bowled(over: &mut Over) {
        over.balls_bowled += 1;
    }

    pub fn print_over(over: Over) {
        println!("Over Bowled");
        for element in over.ball_record {
            println!("{element}")

        }

    }
}