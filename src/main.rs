use crate::over::over::*;
use crate::bat::bat::*;
pub mod over;
pub mod bat;

fn main() {
    let mut bat1 = BatterProfile::init_batter(true);
    let mut bat2 = BatterProfile::init_batter(false);
    let mut over = Over::init_over();

    over.bowl_over(&mut bat1, &mut bat2);
    
}
