const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // let READY_AMOUNT = 1;
    let (missiles, ready):(i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    // let x = 0;

    println!("Firing {} of my {} missles...", ready, missiles);

    // missiles = missiles - ready;

    println!("{} missiles left", missiles - ready);
}
