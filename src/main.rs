use crate::bat::bat::BatterProfile;
use crate::over::over::Over;
pub mod bat;
pub mod over;


fn main() {
    let mut bat1 = BatterProfile::init_batter(true);
    let mut bat2 = BatterProfile::init_batter(false);
    let mut over = Over::init_over();

    

    over.bowl_over(&mut bat1, &mut bat2);
    
}
