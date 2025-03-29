pub mod input {
    fn get_input() -> String {
        let mut tmp = String::new();
        std::io::stdin()
            .read_line(&mut tmp)
            .expect("Failed to readline");
        tmp
    }
    //Returns char from stdin
    pub fn get_char() -> char {
        loop {
            let tmp_c = get_input();
            
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
            let tmp_num = get_input();
            
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
        let tmp_name = get_input();
        tmp_name
    }
}