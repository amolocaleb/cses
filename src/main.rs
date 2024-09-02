mod weird_algorithm;
mod missing_number;
use crate::weird_algorithm::weird_algo;
use crate::missing_number::find_missing;

fn main() {
    weird_algo(3);

    let arr_input = [21,3,4,5,6,9,33,8,11,1,17];
    let missing = find_missing( &arr_input);

    println!("{:?}", missing);

}
