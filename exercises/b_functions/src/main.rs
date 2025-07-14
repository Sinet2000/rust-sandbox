use rand::prelude::*;

use b_functions::area_of_rectangle;

fn main() {
    let mut rng = rand::rng();
    println!("char: '{}'", rng.random::<char>());
    // Try printing a random alphanumeric value instead!
    println!("alpha: '{}'", rng.sample(rand::distr::Alphanumeric) as char);

    // // let enigma: f32; <-- must be initialized
    // let enigma: f32;
    // let mut x = 5;
    // {
    //     enigma = 3.14; // Initialize enigma
    //     let y = 99;
    //     x += 3;
    //     println!("x is {}, y is {}", x, y);
    // }
    // println!("x is {}, enigma: {}", x, enigma);

    // println!("Multiplication result: {}", multiply(2.0, 3.0));

    let width = 4;
    let height = 8;
    let depth = 10;
    {
        let area = area_of_rectangle(width, height);
        println!("Area of rectangle: {}", area);
    }

    println!("Volume is {}", b_functions::volume(width, height, depth))
}
