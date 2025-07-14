// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]

use c_simple_types::calculate_area;
use c_simple_types::ding;
use c_simple_types::on_off;
use c_simple_types::print_array;
use c_simple_types::print_difference;
use c_simple_types::print_distance;

fn main() {
    let width = 4;
    let height = 8;
    let depth = 10;

    {
        let area = calculate_area(width, height);
        println!("Area of rectangle: {}", area);
    }

    println!("Volume is {}", calculate_area(width, height) * depth);

    let info: (u8, f32, i16) = (1, 3.3, 999); // max 12?
    let (jets, fuel, ammo) = info;
    println!("Typle: ({},{},{})", jets, fuel, ammo);

    let coords: (f32, f32) = (6.3, 15.0);
    let (x, y) = coords;
    print_difference(x, y);

    let coords_arr: [f32; 2] = [x, y];
    print_array(&coords_arr);

    let series = [1, 1, 2, 3, 5, 8, 13];
    ding(series[4]);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    on_off(mess.2[0].0);
}
