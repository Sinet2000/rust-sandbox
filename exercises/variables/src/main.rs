const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let mut missiles: i16 = STARTING_MISSILES as i16;
    let (mut ready, mut fired): (i16, i16) = (READY_AMOUNT as i16, 0);

    println!("Firing {} of my {} missiles ...", ready, missiles);
    fired += ready;

    missiles -= ready;
    println!("{} missiles left", missiles);

    ready = 4;
    println!("Firing more {} missiles ...", ready);
    fired += ready;

    missiles -= ready;
    println!("{} missiles left", missiles);
    println!("Total fired: {}", fired);
}
