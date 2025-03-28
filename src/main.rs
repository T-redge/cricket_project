use crate::over::over::*;
use crate::bat::bat::*;
pub mod over;
pub mod bat;

fn main() {
    let mut bat_one = init_batter();
    let mut over = init_over();

    over.bowl_over(&mut bat_one);
    
}
